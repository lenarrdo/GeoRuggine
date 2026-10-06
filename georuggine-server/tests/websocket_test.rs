use futures_util::{SinkExt, StreamExt};
use georuggine_server::{
    create_app, database as db,
    messages::{ChatMessage, PositionUpdate},
    websocket::{self, send_broadcast, send_to_addr, send_to_user},
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

/// Helper: avvia un'istanza reale del server Axum su porta effimera (porta 0).
async fn spawn_test_server() -> (String, websocket::Clients) {
    let db_pool = db::init(true).await.expect("Inizializzazione DB fallita");
    let clients = websocket::create_clients();
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

    (format!("ws://127.0.0.1:{port}/ws"), clients)
}

/// TEST 1: Connessione e disconnessione pulita
#[tokio::test]
async fn test_ws_connection_and_disconnect() {
    let (server_url, clients) = spawn_test_server().await;

    // 1. Handshake WebSocket
    let (ws_stream, response) = connect_async(&server_url)
        .await
        .expect("Connessione fallita");
    assert_eq!(
        response.status(),
        101,
        "L'handshake WebSocket deve rispondere con 101 Switching Protocols"
    );

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifica registrazione nella mappa clients
    {
        let guard = clients.read().await;
        assert_eq!(guard.len(), 1, "Ci deve essere 1 client registrato");
    }

    // 2. Chiusura del socket (drop)
    drop(ws_stream);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifica pulizia automatica
    {
        let guard = clients.read().await;
        assert_eq!(
            guard.len(),
            0,
            "Il client disconnesso deve essere rimosso automaticamente"
        );
    }
}

/// TEST 2: Il client invia coordinate di posizione (PositionUpdate)
#[tokio::test]
async fn test_client_sends_position() {
    let (server_url, clients) = spawn_test_server().await;

    let (mut ws_stream, _) = connect_async(&server_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Invio posizione JSON
    let pos = PositionUpdate {
        user_id: 101,
        x_coord: 450700,
        y_coord: 76800,
        timestamp: "2026-10-03T21:00:00Z".to_string(),
    };
    let json = serde_json::to_string(&pos).unwrap();
    ws_stream.send(Message::Text(json.into())).await.unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Il server deve aver associato l'user_id 101
    {
        let guard = clients.read().await;
        assert_eq!(guard.len(), 1);
        let client = guard.values().next().unwrap();
        assert_eq!(
            client.user_id,
            Some(101),
            "L'user_id deve essere impostato a 101"
        );
    }
}

/// TEST 3: Il client invia un messaggio di chat testuale al server (NON broadcast ad altri client)
#[tokio::test]
async fn test_client_sends_chat_message() {
    let (server_url, clients) = spawn_test_server().await;

    let (mut ws_client1, _) = connect_async(&server_url).await.unwrap();
    let (mut ws_client2, _) = connect_async(&server_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Client 1 invia un messaggio destinato alla centrale (server)
    let chat = ChatMessage {
        message_id: None,
        sender_id: 42,
        receiver_id: Some(0), // Server
        timestamp: "2026-10-03T21:10:00Z".to_string(),
        contents: "Richiesta rifornimento carburante".to_string(),
    };
    let json = serde_json::to_string(&chat).unwrap();
    ws_client1.send(Message::Text(json.into())).await.unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // 1. Il server aggiorna l'user_id di Client 1 a 42
    {
        let guard = clients.read().await;
        let c1 = guard.values().find(|c| c.user_id == Some(42));
        assert!(c1.is_some(), "Client 1 deve essere associato a user_id 42");
    }

    // 2. Client 2 NON deve ricevere il messaggio (perché il client comunica solo col server, no broadcast tra client)
    let res = timeout(Duration::from_millis(200), ws_client2.next()).await;
    assert!(
        res.is_err(),
        "Client 2 non deve ricevere il messaggio che Client 1 ha inviato al server"
    );
}

/// TEST 4: Il server invia un messaggio in BROADCAST a tutti i client connessi
#[tokio::test]
async fn test_server_sends_broadcast() {
    let (server_url, clients) = spawn_test_server().await;

    let (mut ws1, _) = connect_async(&server_url).await.unwrap();
    let (mut ws2, _) = connect_async(&server_url).await.unwrap();
    let (mut ws3, _) = connect_async(&server_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    {
        let guard = clients.read().await;
        assert_eq!(guard.len(), 3, "Ci devono essere 3 client connessi");
    }

    // Il SERVER invia un broadcast
    let broadcast_msg = ChatMessage {
        message_id: None,
        sender_id: 0,      // Server
        receiver_id: None, // Broadcast
        timestamp: "2026-10-03T21:30:00Z".to_string(),
        contents: "Avviso emergenza meteo a tutta la flotta!".to_string(),
    };
    send_broadcast(&clients, broadcast_msg.clone()).await;

    // Tutti e 3 i client devono ricevere il messaggio contemporaneamente
    for (i, ws) in [&mut ws1, &mut ws2, &mut ws3].into_iter().enumerate() {
        let msg = timeout(Duration::from_secs(1), ws.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        if let Message::Text(text) = msg {
            let received: ChatMessage = serde_json::from_str(&text).unwrap();
            assert_eq!(
                received.contents,
                "Avviso emergenza meteo a tutta la flotta!",
                "Client {} non ha ricevuto il testo corretto",
                i + 1
            );
        } else {
            panic!("Client {} non ha ricevuto Message::Text", i + 1);
        }
    }
}

/// TEST 5: Il server invia un messaggio DIRETTO a un singolo utente (send_to_user)
#[tokio::test]
async fn test_server_sends_direct_message_to_user() {
    let (server_url, clients) = spawn_test_server().await;

    let (mut ws1, _) = connect_async(&server_url).await.unwrap();
    let (mut ws2, _) = connect_async(&server_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Client 1 si registra come user 55
    let pos1 = PositionUpdate {
        user_id: 55,
        x_coord: 10,
        y_coord: 20,
        timestamp: "2026-10-03T21:00:00Z".to_string(),
    };
    ws1.send(Message::Text(serde_json::to_string(&pos1).unwrap().into()))
        .await
        .unwrap();

    // Client 2 si registra come user 77
    let pos2 = PositionUpdate {
        user_id: 77,
        x_coord: 30,
        y_coord: 40,
        timestamp: "2026-10-03T21:00:00Z".to_string(),
    };
    ws2.send(Message::Text(serde_json::to_string(&pos2).unwrap().into()))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Server invia a user 55
    let direct_msg = ChatMessage {
        message_id: None,
        sender_id: 0,
        receiver_id: Some(55),
        timestamp: "2026-10-03T21:05:00Z".to_string(),
        contents: "Cambio itinerario assegnato al veicolo 55".to_string(),
    };
    let sent = send_to_user(&clients, 55, direct_msg).await;
    assert!(sent, "Invio al client 55 deve avere successo");

    // Client 1 riceve
    let msg1 = timeout(Duration::from_secs(1), ws1.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    if let Message::Text(text) = msg1 {
        let received: ChatMessage = serde_json::from_str(&text).unwrap();
        assert_eq!(
            received.contents,
            "Cambio itinerario assegnato al veicolo 55"
        );
    } else {
        panic!("Client 1 non ha ricevuto testo");
    }

    // Client 2 NON riceve nulla
    let msg2 = timeout(Duration::from_millis(200), ws2.next()).await;
    assert!(
        msg2.is_err(),
        "Client 2 non deve ricevere il messaggio privato di Client 1"
    );
}

/// TEST 6: Il server invia un messaggio diretto tramite SocketAddr (send_to_addr)
#[tokio::test]
async fn test_server_sends_to_addr() {
    let (server_url, clients) = spawn_test_server().await;

    let (mut ws, _) = connect_async(&server_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Recuperiamo l'indirizzo SocketAddr del client dalla mappa
    let client_addr = {
        let guard = clients.read().await;
        *guard.keys().next().expect("Client deve essere presente")
    };

    let msg = ChatMessage {
        message_id: None,
        sender_id: 0,
        receiver_id: None,
        timestamp: "2026-10-03T21:40:00Z".to_string(),
        contents: "Messaggio diretto su SocketAddr".to_string(),
    };

    let sent = send_to_addr(&clients, &client_addr, msg).await;
    assert!(sent, "send_to_addr deve restituire true");

    let received_raw = timeout(Duration::from_secs(1), ws.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    if let Message::Text(text) = received_raw {
        let received: ChatMessage = serde_json::from_str(&text).unwrap();
        assert_eq!(received.contents, "Messaggio diretto su SocketAddr");
    } else {
        panic!("Atteso messaggio di testo");
    }
}

/// TEST 7: Chiusura aggraziata tramite CloseFrame (Message::Close)
#[tokio::test]
async fn test_graceful_close_frame() {
    let (server_url, clients) = spawn_test_server().await;

    let (mut ws, _) = connect_async(&server_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;

    {
        let guard = clients.read().await;
        assert_eq!(guard.len(), 1);
    }

    // Invio frame di chiusura con Normal Closure (1000)
    let close_frame = CloseFrame {
        code: CloseCode::Normal,
        reason: "Disconnessione utente completata".into(),
    };
    ws.send(Message::Close(Some(close_frame))).await.unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Il server deve aver rimosso il client
    {
        let guard = clients.read().await;
        assert_eq!(
            guard.len(),
            0,
            "Il client che ha inviato Close deve essere rimosso"
        );
    }
}

/// TEST 8: Flusso completo end-to-end con flotta di veicoli
#[tokio::test]
async fn test_fleet_lifecycle_flow() {
    let (server_url, clients) = spawn_test_server().await;

    // 1. Connettiamo 4 veicoli
    let mut fleet = Vec::new();
    for i in 1..=4 {
        let (ws, _) = connect_async(&server_url).await.unwrap();
        fleet.push((i, ws));
    }
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(clients.read().await.len(), 4);

    // 2. Ciascun veicolo invia le proprie coordinate GPS
    for (id, ws) in fleet.iter_mut() {
        let pos = PositionUpdate {
            user_id: *id,
            x_coord: 1000 * (*id),
            y_coord: 2000 * (*id),
            timestamp: "2026-10-03T21:45:00Z".to_string(),
        };
        ws.send(Message::Text(serde_json::to_string(&pos).unwrap().into()))
            .await
            .unwrap();
    }
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifichiamo che tutti e 4 gli user_id siano stati censiti dal server
    {
        let guard = clients.read().await;
        for i in 1..=4 {
            let found = guard.values().any(|c| c.user_id == Some(i));
            assert!(
                found,
                "Veicolo {} deve essere registrato con il suo user_id",
                i
            );
        }
    }

    // 3. La centrale invia broadcast di inizio turno
    let shift_msg = ChatMessage {
        message_id: None,
        sender_id: 0,
        receiver_id: None,
        timestamp: "2026-10-03T21:46:00Z".to_string(),
        contents: "Inizio turno operativo flotta".to_string(),
    };
    send_broadcast(&clients, shift_msg).await;

    for (id, ws) in fleet.iter_mut() {
        let msg = timeout(Duration::from_secs(1), ws.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        if let Message::Text(t) = msg {
            let parsed: ChatMessage = serde_json::from_str(&t).unwrap();
            assert_eq!(parsed.contents, "Inizio turno operativo flotta");
        } else {
            panic!("Veicolo {} non ha ricevuto broadcast", id);
        }
    }

    // 4. La centrale assegna una missione privata al solo veicolo 3
    let order_v3 = ChatMessage {
        message_id: None,
        sender_id: 0,
        receiver_id: Some(3),
        timestamp: "2026-10-03T21:47:00Z".to_string(),
        contents: "Raggiungere waypoint Charlie".to_string(),
    };
    let sent = send_to_user(&clients, 3, order_v3).await;
    assert!(sent);

    // Solo il veicolo 3 deve riceverlo
    let (_, ws3) = &mut fleet[2];
    let msg3 = timeout(Duration::from_secs(1), ws3.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    if let Message::Text(t) = msg3 {
        let parsed: ChatMessage = serde_json::from_str(&t).unwrap();
        assert_eq!(parsed.contents, "Raggiungere waypoint Charlie");
    }

    // Gli altri 3 veicoli (1, 2, 4) NON devono ricevere nulla
    for (id, ws) in fleet.iter_mut() {
        if *id != 3 {
            let timeout_res = timeout(Duration::from_millis(150), ws.next()).await;
            assert!(
                timeout_res.is_err(),
                "Veicolo {} non doveva ricevere l'ordine per il veicolo 3",
                id
            );
        }
    }
}
