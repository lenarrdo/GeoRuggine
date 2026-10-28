use futures_util::{SinkExt, StreamExt};
use georuggine_server::{
    auth::services::token_service,
    create_app,
    database as db,
    messages::{PositionUpdate, TextMessage},
    websocket::clients::ClientsList,
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message;

/// Helper: crea un token JWT valido per i test
fn create_test_token(user_id: i64, username: &str) -> String {
    token_service::generate_jwt(user_id, username).expect("Generazione JWT fallita")
}

/// Helper: avvia un'istanza reale del server Axum su porta effimera (porta 0).
async fn spawn_test_server() -> (String, ClientsList) {
    let db_pool = db::init(true).await.expect("Inizializzazione DB fallita");
    let clients = ClientsList::new();
    let app = create_app(db_pool, clients.clone());

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    (format!("ws://127.0.0.1:{port}/api/v1/ws"), clients)
}

/// Helper per connettere un client WebSocket con credenziali
async fn connect_client(base_url: &str, user_id: i64, username: &str) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    let token = create_test_token(user_id, username);
    let url = format!("{base_url}?token={token}");
    let (ws, response) = connect_async(&url).await.expect("Connessione fallita");
    assert_eq!(response.status(), 101, "L'handshake WebSocket deve rispondere con 101");
    ws
}

/// TEST 1: Connessione e disconnessione pulita
#[tokio::test]
async fn test_ws_connection_and_disconnect() {
    let (server_url, clients) = spawn_test_server().await;

    // 1. Handshake WebSocket con token
    let ws_stream = connect_client(&server_url, 1, "alice").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifica registrazione nella lista clients
    assert_eq!(clients.len().await, 1, "Ci deve essere 1 client registrato");
    assert!(clients.is_connected(1).await, "L'utente 1 deve risultare connesso");

    // 2. Chiusura del socket (drop)
    drop(ws_stream);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifica pulizia automatica
    assert_eq!(clients.len().await, 0, "Il client disconnesso deve essere rimosso automaticamente");
    assert!(!clients.is_connected(1).await, "L'utente 1 non deve più risultare connesso");
}

/// TEST 2: Il client invia coordinate di posizione (PositionUpdate) anche senza specificare sender_id
#[tokio::test]
async fn test_client_sends_position() {
    let (server_url, clients) = spawn_test_server().await;

    let mut ws_stream = connect_client(&server_url, 101, "vehicle_101").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Invio posizione JSON (con timestamp calcolato dal client, senza ID)
    let json = serde_json::json!({
        "x_coord": 450700,
        "y_coord": 76800,
        "timestamp": "2026-10-16T12:00:00Z"
    });
    ws_stream.send(Message::Text(json.to_string().into())).await.unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(clients.len().await, 1);
    assert!(clients.is_connected(101).await, "L'utente 101 deve risultare registrato e connesso");
}

/// TEST 3: Il client invia un messaggio di chat al server
#[tokio::test]
async fn test_client_sends_chat_message() {
    let (server_url, _clients) = spawn_test_server().await;

    let mut ws_client1 = connect_client(&server_url, 42, "pilot_42").await;
    let mut ws_client2 = connect_client(&server_url, 99, "pilot_99").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Client 1 invia un messaggio destinato alla centrale (con contents e timestamp)
    let chat = serde_json::json!({
        "contents": "Richiesta rifornimento carburante",
        "timestamp": "2026-10-17T12:00:00Z"
    });
    ws_client1.send(Message::Text(chat.to_string().into())).await.unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // 1. Client 2 NON deve ricevere il messaggio (comunicazione diretta client -> server)
    let res = timeout(Duration::from_millis(200), ws_client2.next()).await;
    assert!(res.is_err(), "Client 2 non deve ricevere il messaggio che Client 1 ha inviato al server");

    // 2. Client 1 NON riceve alcuna conferma dal server
    let res1 = timeout(Duration::from_millis(200), ws_client1.next()).await;
    assert!(res1.is_err(), "Client 1 non deve ricevere alcuna conferma o ack dal server");
}

/// TEST 4: Il server invia un messaggio in BROADCAST a tutti i client connessi
#[tokio::test]
async fn test_server_sends_broadcast() {
    let (server_url, clients) = spawn_test_server().await;

    let mut ws1 = connect_client(&server_url, 1, "user1").await;
    let mut ws2 = connect_client(&server_url, 2, "user2").await;
    let mut ws3 = connect_client(&server_url, 3, "user3").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(clients.len().await, 3, "Ci devono essere 3 client connessi");

    // Il SERVER invia un broadcast
    let broadcast_msg = TextMessage {
        timestamp: "2026-10-03T21:30:00Z".to_string(),
        contents: "Avviso emergenza meteo a tutta la flotta!".to_string(),
    };
    clients.send_all(broadcast_msg).await;

    // Tutti e 3 i client devono ricevere il messaggio contemporaneamente
    for (i, ws) in [&mut ws1, &mut ws2, &mut ws3].into_iter().enumerate() {
        let msg = timeout(Duration::from_secs(1), ws.next()).await.unwrap().unwrap().unwrap();
        if let Message::Text(text) = msg {
            let received: TextMessage = serde_json::from_str(&text).unwrap();
            assert_eq!(received.contents, "Avviso emergenza meteo a tutta la flotta!", "Client {} non ha ricevuto il testo corretto", i + 1);
        } else {
            panic!("Client {} non ha ricevuto Message::Text", i + 1);
        }
    }
}

/// TEST 5: Il server invia un messaggio DIRETTO a un singolo utente
#[tokio::test]
async fn test_server_sends_direct_message_to_user() {
    let (server_url, clients) = spawn_test_server().await;

    let mut ws1 = connect_client(&server_url, 55, "user55").await;
    let mut ws2 = connect_client(&server_url, 77, "user77").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Server invia a user 55
    let direct_msg = TextMessage {
        timestamp: "2026-10-03T21:05:00Z".to_string(),
        contents: "Cambio itinerario assegnato al veicolo 55".to_string(),
    };
    let sent = clients.send_one(55, direct_msg).await;
    assert!(sent, "Invio al client 55 deve avere successo");

    // Client 1 riceve
    let msg1 = timeout(Duration::from_secs(1), ws1.next()).await.unwrap().unwrap().unwrap();
    if let Message::Text(text) = msg1 {
        let received: TextMessage = serde_json::from_str(&text).unwrap();
        assert_eq!(received.contents, "Cambio itinerario assegnato al veicolo 55");
    } else {
        panic!("Client 1 non ha ricevuto testo");
    }

    // Client 2 NON riceve nulla
    let msg2 = timeout(Duration::from_millis(200), ws2.next()).await;
    assert!(msg2.is_err(), "Client 2 non deve ricevere il messaggio privato di Client 1");
}

/// TEST 6: Chiusura aggraziata tramite CloseFrame (Message::Close)
#[tokio::test]
async fn test_graceful_close_frame() {
    let (server_url, clients) = spawn_test_server().await;

    let mut ws = connect_client(&server_url, 1, "test_close").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(clients.len().await, 1);

    // Invio frame di chiusura con Normal Closure (1000)
    let close_frame = CloseFrame {
        code: CloseCode::Normal,
        reason: "Disconnessione utente completata".into(),
    };
    ws.send(Message::Close(Some(close_frame))).await.unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Il server deve aver rimosso il client
    assert_eq!(clients.len().await, 0, "Il client che ha inviato Close deve essere rimosso");
}

/// TEST 7: Flusso completo end-to-end con flotta di veicoli
#[tokio::test]
async fn test_fleet_lifecycle_flow() {
    let (server_url, clients) = spawn_test_server().await;

    // 1. Connettiamo 4 veicoli con credenziali valide
    let mut fleet = Vec::new();
    for i in 1..=4 {
        let ws = connect_client(&server_url, i, &format!("vehicle_{i}")).await;
        fleet.push((i, ws));
    }
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(clients.len().await, 4);

    // 2. Ciascun veicolo invia le proprie coordinate GPS con timestamp calcolato dal client
    for (id, ws) in fleet.iter_mut() {
        let pos = PositionUpdate {
            x_coord: 1000 * (*id),
            y_coord: 2000 * (*id),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        };
        ws.send(Message::Text(serde_json::to_string(&pos).unwrap().into())).await.unwrap();
    }
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifichiamo che tutti e 4 siano connessi
    for i in 1..=4 {
        assert!(clients.is_connected(i).await, "Veicolo {} deve essere connesso", i);
    }

    // 3. La centrale invia broadcast di inizio turno
    let shift_msg = TextMessage {
        timestamp: "2026-10-03T21:46:00Z".to_string(),
        contents: "Inizio turno operativo flotta".to_string(),
    };
    clients.send_all(shift_msg).await;

    for (id, ws) in fleet.iter_mut() {
        let msg = timeout(Duration::from_secs(1), ws.next()).await.unwrap().unwrap().unwrap();
        if let Message::Text(t) = msg {
            let parsed: TextMessage = serde_json::from_str(&t).unwrap();
            assert_eq!(parsed.contents, "Inizio turno operativo flotta");
        } else {
            panic!("Veicolo {} non ha ricevuto broadcast", id);
        }
    }

    // 4. La centrale assegna una missione privata al solo veicolo 3
    let order_v3 = TextMessage {
        timestamp: "2026-10-03T21:47:00Z".to_string(),
        contents: "Raggiungere waypoint Charlie".to_string(),
    };
    let sent = clients.send_one(3, order_v3).await;
    assert!(sent);

    // Solo il veicolo 3 deve riceverlo
    let (_, ws3) = &mut fleet[2];
    let msg3 = timeout(Duration::from_secs(1), ws3.next()).await.unwrap().unwrap().unwrap();
    if let Message::Text(t) = msg3 {
        let parsed: TextMessage = serde_json::from_str(&t).unwrap();
        assert_eq!(parsed.contents, "Raggiungere waypoint Charlie");
    }

    // Gli altri 3 veicoli (1, 2, 4) NON devono ricevere nulla
    for (id, ws) in fleet.iter_mut() {
        if *id != 3 {
            let timeout_res = timeout(Duration::from_millis(150), ws.next()).await;
            assert!(timeout_res.is_err(), "Veicolo {} non doveva ricevere l'ordine per il veicolo 3", id);
        }
    }
}

/// TEST 8: Connessione senza token JWT viene respinta con 401 Unauthorized
#[tokio::test]
async fn test_unauthorized_connection_rejected() {
    let (server_url, _) = spawn_test_server().await;

    // Tentativo di connessione senza token
    let res = connect_async(&server_url).await;
    assert!(res.is_err(), "Connessione senza token deve fallire");

    // Tentativo con token invalido
    let res_invalid = connect_async(&format!("{server_url}?token=token_invalido")).await;
    assert!(res_invalid.is_err(), "Connessione con token invalido deve fallire");
}

/// TEST 9: MessageRepository salva nel database il timestamp calcolato dal client
#[tokio::test]
async fn test_save_position_uses_client_timestamp() {
    use std::str::FromStr;
    let options = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let pool = sqlx::sqlite::SqlitePoolOptions::new().connect_with(options).await.unwrap();

    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, username TEXT NOT NULL, password_hash TEXT NOT NULL);").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE positions (id INTEGER PRIMARY KEY, user_id INTEGER NOT NULL, x_coord INTEGER NOT NULL, y_coord INTEGER NOT NULL, timestamp DATETIME NOT NULL, FOREIGN KEY(user_id) REFERENCES users(id));").execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO users VALUES (42, 'vehicle_42', 'hash');").execute(&pool).await.unwrap();

    let repo = georuggine_server::messages::message_repository::MessageRepository::new(pool.clone());
    let client_time = "2026-10-16T12:30:00+02:00".to_string();
    let pos = PositionUpdate {
        x_coord: 12345,
        y_coord: 67890,
        timestamp: Some(client_time.clone()),
    };
    let timestamp = pos.timestamp.as_deref().unwrap();
    repo.save_position(42, &pos, timestamp).await.unwrap();

    let record: (i64, i64, String) = sqlx::query_as("SELECT x_coord, y_coord, timestamp FROM positions WHERE user_id = ?")
        .bind(42)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(record.0, 12345);
    assert_eq!(record.1, 67890);
    assert_eq!(record.2, client_time);
}
