use crate::messages::{ChatMessage, ClientIncomingMessage};
use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{collections::HashMap, net::SocketAddr, ops::ControlFlow, sync::Arc};
use tokio::sync::{mpsc, RwLock};

/// Rappresenta un client WebSocket attualmente connesso al server.
/// - `user_id`: identificativo dell'utente autenticato (se disponibile).
/// - `tx`: estremità di invio di un canale MPSC dedicato. Ogni volta che il server
///   (o un'interfaccia grafica/operatore) vuole inviare un messaggio a questo client,
///   lo spedisce su questo canale in memoria senza bloccare la rete.
#[derive(Clone)]
pub struct ConnectedClient {
    pub user_id: Option<i64>,
    pub tx: mpsc::UnboundedSender<ChatMessage>,
}

/// Registro condiviso di tutte le sessioni WebSocket attive.
/// È racchiuso in:
/// - `Arc`: per poter condividere il riferimento tra thread/task diversi (UI, handler HTTP, task WebSocket).
/// - `RwLock`: permette letture concorrenti multiple (es. più broadcast contemporanei)
///   e accessi esclusivi in scrittura solo durante inserimenti (connessione) o rimozioni (disconnessione).
pub type Clients = Arc<RwLock<HashMap<SocketAddr, ConnectedClient>>>;

/// Funzione di utilità per inizializzare il registro dei client vuoto.
pub fn create_clients() -> Clients {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Invia un messaggio in BROADCAST a tutte le sessioni WebSocket attive.
/// Non invia direttamente sulla rete, ma inserisce il messaggio nelle code MPSC
/// in memoria di ciascun client: in questo modo un client con rete lenta non rallenta gli altri.
#[allow(dead_code)]
pub async fn send_broadcast(clients: &Clients, msg: ChatMessage) {
    let guard = clients.read().await;
    for (addr, client) in guard.iter() {
        if let Err(e) = client.tx.send(msg.clone()) {
            eprintln!("Errore inserimento messaggio in coda broadcast per {addr}: {e}");
        }
    }
}

/// Invia un messaggio a un SINGOLO CLIENT specifico cercando il suo `user_id`.
/// Ritorna `true` se il client è stato trovato ed è connesso, `false` altrimenti.
#[allow(dead_code)]
pub async fn send_to_user(clients: &Clients, target_user_id: i64, msg: ChatMessage) -> bool {
    let guard = clients.read().await;
    let mut sent = false;
    for (addr, client) in guard.iter() {
        if client.user_id == Some(target_user_id) {
            if let Err(e) = client.tx.send(msg.clone()) {
                eprintln!("Errore invio a user {target_user_id} su {addr}: {e}");
            } else {
                sent = true;
            }
        }
    }
    sent
}

/// Invia un messaggio a un SINGOLO CLIENT identificato direttamente dal suo `SocketAddr`.
#[allow(dead_code)]
pub async fn send_to_addr(clients: &Clients, addr: &SocketAddr, msg: ChatMessage) -> bool {
    let guard = clients.read().await;
    if let Some(client) = guard.get(addr) {
        client.tx.send(msg).is_ok()
    } else {
        false
    }
}

/// Handler HTTP iniziale per la rotta WebSocket (es. `GET /ws`).
/// 1. Estrae lo `User-Agent` del browser o dell'applicazione client.
/// 2. Estrae l'indirizzo IP e porta di connessione del client (`ConnectInfo(addr)`).
/// 3. Estrae lo stato condiviso dei client (`State(clients)`).
/// 4. Risponde con l'upgrade HTTP 101 e passa il socket a `handle_socket`.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(clients): State<Clients>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}` at {addr} connected.");

    // Se l'upgrade ha successo, Axum affida il socket attivo alla funzione `handle_socket`
    ws.on_upgrade(move |socket| handle_socket(socket, addr, clients))
}

/// Gestisce l'intero ciclo di vita di una singola connessione WebSocket attiva.
pub async fn handle_socket(socket: WebSocket, who: SocketAddr, clients: Clients) {
    // 1. Dividiamo il socket in due metà indipendenti:
    //    - `ws_sender`: per trasmettere messaggi verso il client.
    //    - `ws_receiver`: per ricevere i messaggi inviati dal client.
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // 2. Creiamo un canale MPSC dedicato a questa specifica connessione.
    //    - `tx`: viene salvato nella mappa `clients` affinché il resto dell'app possa inviare dati a questo client.
    //    - `rx`: viene spostato nel `send_task` per leggere i messaggi e spararli sul WebSocket.
    let (tx, mut rx) = mpsc::unbounded_channel::<ChatMessage>();

    // 3. Registriamo il client nella mappa dei client attivi
    {
        let mut guard = clients.write().await;
        guard.insert(
            who,
            ConnectedClient {
                user_id: None,
                tx,
            },
        );
        println!("Client {who} connesso (totale attivi: {})", guard.len());
    }

    // 4. TASK DI INVIO (Send Task):
    //    Rimane in attesa passiva su `rx.recv().await` senza consumare CPU.
    //    Non appena la UI o il server inviano un `ChatMessage` su `tx`, questo task si sveglia,
    //    lo serializza in JSON e lo trasmette sul WebSocket del client.
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            if ws_sender.send(Message::Text(json.into())).await.is_err() {
                // Se la trasmissione di rete fallisce (es. client disconnesso), usciamo dal loop
                break;
            }
        }
    });

    // 5. TASK DI RICEZIONE (Recv Task):
    //    Ascolta continuamente i pacchetti in arrivo dal client tramite `ws_receiver.next()`.
    //    Ogni messaggio ricevuto viene analizzato ed elaborato da `process_message`.
    let clients_clone = clients.clone();
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = ws_receiver.next().await {
            cnt += 1;

            // Se process_message ritorna Break(()), interrompiamo la ricezione
            if process_message(msg, who, &clients_clone).await.is_break() {
                break;
            }
        }

        cnt
    });

    // 6. COORDINAMENTO CONCORRENTE:
    //    `tokio::select!` attende il PRIMO dei due task che si conclude.
    //    Se il client chiude la connessione (recv_task termina), abortiamo subito anche il send_task,
    //    e viceversa, garantendo che non rimangano task "zombie" in background.
    tokio::select! {
        rv_a = (&mut send_task) => {
            if let Err(e) = rv_a {
                eprintln!("Errore send_task per {who}: {e:?}");
            }
            recv_task.abort();
        }

        rv_b = (&mut recv_task) => {
            if let Err(e) = rv_b {
                eprintln!("Errore recv_task per {who}: {e:?}");
            }
            send_task.abort();
        }
    }

    // 7. PULIZIA DELLE RISORSE ALLA DISCONNESSIONE:
    //    Rimuoviamo la sessione dal registro dei client attivi per liberare la memoria.
    {
        let mut guard = clients.write().await;
        guard.remove(&who);
        println!("Client {who} disconnesso (rimanenti: {})", guard.len());
    }

    println!("Websocket context {who} destroyed");
}

/// Elabora i singoli messaggi ricevuti dal client via WebSocket.
/// Ritorna:
/// - `ControlFlow::Continue(())`: la connessione prosegue normalmente.
/// - `ControlFlow::Break(())`: il ciclo deve interrompersi (es. richiesta di chiusura).
pub async fn process_message(
    msg: Message,
    who: SocketAddr,
    clients: &Clients,
) -> ControlFlow<(), ()> {
    match msg {
        // Messaggi di testo: il client invia dati al server (coordinate GPS o messaggi di testo)
        Message::Text(t) => {
            if let Ok(incoming) = serde_json::from_str::<ClientIncomingMessage>(&t) {
                match incoming {
                    // Caso 1: Il client invia le coordinate della sua posizione
                    ClientIncomingMessage::Position(pos) => {
                        println!(
                            ">>> [COORDINATE DA CLIENT] Da {who} (User {}): x={}, y={}, time={}",
                            pos.user_id, pos.x_coord, pos.y_coord, pos.timestamp
                        );

                        // Aggiorniamo l'identificativo user_id associato a questa connessione
                        let mut guard = clients.write().await;
                        if let Some(client) = guard.get_mut(&who) {
                            client.user_id = Some(pos.user_id);
                        }
                    }

                    // Caso 2: Il client invia un messaggio di testo destinato alla centrale/server
                    ClientIncomingMessage::Chat(chat_msg) => {
                        println!(
                            ">>> [MESSAGGIO AL SERVER] Da {who} (User {}): `{}`",
                            chat_msg.sender_id, chat_msg.contents
                        );

                        // Aggiorniamo l'identificativo user_id associato a questa connessione
                        let mut guard = clients.write().await;
                        if let Some(client) = guard.get_mut(&who) {
                            client.user_id = Some(chat_msg.sender_id);
                        }

                        // NOTA ARCHITETTURALE: Il messaggio del client NON viene re-inoltrato ad altri client.
                        // Rimane al server per essere gestito dall'operatore o salvato a database.
                    }
                }
            } else {
                println!(">>> {who} ha inviato testo generico non strutturato: {t:?}");
            }
        }

        // Messaggio binario
        Message::Binary(d) => {
            println!(">>> {who} ha inviato {} bytes binari: {d:?}", d.len());
        }

        // Richiesta di chiusura pulita della connessione WebSocket da parte del client
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {who} ha inviato chiusura socket code={} reason=`{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> {who} ha chiuso la connessione");
            }

            // Segnaliamo al loop del receiver di uscire e chiudere la sessione
            return ControlFlow::Break(());
        }

        // Risposta a un Ping (mantenimento connessione)
        Message::Pong(v) => {
            println!(">>> {who} ha risposto con pong {v:?}");
        }

        // Ping ricevuto dal client (Axum/tungstenite risponde automaticamente con un Pong)
        Message::Ping(v) => {
            println!(">>> {who} ha inviato ping {v:?}");
        }
    }

    ControlFlow::Continue(())
}
