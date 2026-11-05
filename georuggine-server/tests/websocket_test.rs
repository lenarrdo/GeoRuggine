use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use georuggine_server::{
    auth::auth_service::AuthService,
    init_router,
    messages::{
        position::{Coordinates, repository::PositionRepository},
        text::TextMessage,
    },
    shared::app_state::AppState,
};
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

/***********
 * HELPERS *
 ***********/

async fn spawn_test_server() -> (String, Arc<AppState>) {
    let app_state = Arc::new(AppState::mock().await);
    let router = init_router(Arc::clone(&app_state));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    (format!("ws://127.0.0.1:{port}/api/v1/ws"), app_state)
}

async fn connect_client(
    state: Arc<AppState>,
    base_url: &str,
    user_id: u32,
    username: &str,
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    let token = AuthService::new(state.db.clone(), state.config.jwt_secret.clone())
        .generate_jwt(user_id, username)
        .expect("Failed to generate token.");

    let url = format!("{base_url}?token={token}");
    let (ws, response) = connect_async(&url).await.expect("Connessione fallita");
    assert_eq!(
        response.status(),
        101,
        "L'handshake WebSocket deve rispondere con 101"
    );
    ws
}

/*********
 * TESTS *
 *********/

#[tokio::test]
async fn ws_connection_and_disconnect() {
    let (server_url, state) = spawn_test_server().await;

    // 1. Handshake WebSocket con token
    let ws_stream = connect_client(Arc::clone(&state), &server_url, 1, "alice").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifica registrazione nella lista state.clients
    assert_eq!(
        state.clients.list.len(),
        1,
        "Ci deve essere 1 client registrato"
    );
    assert!(
        state.clients.is_connected(1),
        "L'utente 1 deve risultare connesso"
    );

    // 2. Chiusura del socket (drop)
    drop(ws_stream);
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Verifica pulizia automatica
    assert_eq!(
        state.clients.list.len(),
        0,
        "Il client disconnesso deve essere rimosso automaticamente"
    );
    assert!(
        !state.clients.is_connected(1),
        "L'utente 1 non deve più risultare connesso"
    );
}

#[tokio::test]
async fn client_sends_position() {
    let (server_url, state) = spawn_test_server().await;

    let mut ws_stream = connect_client(Arc::clone(&state), &server_url, 101, "vehicle_101").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Invio posizione JSON (con timestamp calcolato dal client, senza ID)
    let json = serde_json::json!({
        "lat": 45.07,
        "lon": 76.80,
        "timestamp": "2026-10-16T12:00:00Z"
    });
    ws_stream
        .send(Message::Text(json.to_string().into()))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(state.clients.list.len(), 1);
    assert!(
        state.clients.is_connected(101),
        "L'utente 101 deve risultare registrato e connesso"
    );
}

#[tokio::test]
async fn client_sends_chat_message() {
    let (server_url, state) = spawn_test_server().await;

    let mut ws_client1 = connect_client(Arc::clone(&state), &server_url, 42, "pilot_42").await;
    let mut ws_client2 = connect_client(Arc::clone(&state), &server_url, 99, "pilot_99").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Client 1 invia un messaggio destinato alla centrale (con contents e timestamp)
    let chat = serde_json::json!({
        "contents": "Richiesta rifornimento carburante",
        "timestamp": "2026-10-17T12:00:00Z"
    });
    ws_client1
        .send(Message::Text(chat.to_string().into()))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // 1. Client 2 NON deve ricevere il messaggio (comunicazione diretta client -> server)
    let res = timeout(Duration::from_millis(200), ws_client2.next()).await;
    assert!(
        res.is_err(),
        "Client 2 non deve ricevere il messaggio che Client 1 ha inviato al server"
    );

    // 2. Client 1 NON riceve alcuna conferma dal server
    let res1 = timeout(Duration::from_millis(200), ws_client1.next()).await;
    assert!(
        res1.is_err(),
        "Client 1 non deve ricevere alcuna conferma o ack dal server"
    );
}

#[tokio::test]
async fn server_sends_broadcast() {
    let (server_url, state) = spawn_test_server().await;

    let ids = vec![1, 2, 3];
    let mut ws1 = connect_client(Arc::clone(&state), &server_url, ids[0], "pilot_1").await;
    let mut ws2 = connect_client(Arc::clone(&state), &server_url, ids[1], "pilot_2").await;
    let mut ws3 = connect_client(Arc::clone(&state), &server_url, ids[2], "pilot_3").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(
        state.clients.list.len(),
        3,
        "Ci devono essere 3 client connessi"
    );

    // Il SERVER invia un broadcast
    let msg = "Avviso emergenza meteo a tutta la flotta!";
    for user_id in ids {
        state.clients.send_one(user_id, msg).await.unwrap();
    }

    // Tutti e 3 i client devono ricevere il messaggio contemporaneamente
    for (i, ws) in [&mut ws1, &mut ws2, &mut ws3].into_iter().enumerate() {
        let msg = timeout(Duration::from_secs(1), ws.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        if let Message::Text(text) = msg {
            let received: TextMessage = serde_json::from_str(&text).unwrap();
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

#[tokio::test]
async fn server_sends_direct_message_to_user() {
    let (server_url, state) = spawn_test_server().await;

    let mut ws1 = connect_client(Arc::clone(&state), &server_url, 55, "user55").await;
    let mut ws2 = connect_client(Arc::clone(&state), &server_url, 77, "user77").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Server invia a user 55
    let sent = state
        .clients
        .send_one(55, "Cambio itinerario assegnato al veicolo 55")
        .await;
    assert!(sent.is_ok(), "Invio al client 55 deve avere successo");

    // Client 1 riceve
    let msg1 = timeout(Duration::from_secs(1), ws1.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    if let Message::Text(text) = msg1 {
        let received: TextMessage = serde_json::from_str(&text).unwrap();
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

#[tokio::test]
async fn client_closes_websocket() {
    let (server_url, state) = spawn_test_server().await;

    let mut ws = connect_client(Arc::clone(&state), &server_url, 1, "test_close").await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(state.clients.list.len(), 1);

    // Invio frame di chiusura con Normal Closure (1000)
    let close_frame = CloseFrame {
        code: CloseCode::Normal,
        reason: "Disconnessione utente completata".into(),
    };
    ws.send(Message::Close(Some(close_frame))).await.unwrap();

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Il server deve aver rimosso il client
    assert_eq!(
        state.clients.list.len(),
        0,
        "Il client che ha inviato Close deve essere rimosso"
    );
}

#[tokio::test]
async fn unauthorized_connection_rejected() {
    let (server_url, _) = spawn_test_server().await;

    // Tentativo di connessione senza token
    let res = connect_async(&server_url).await;
    assert!(res.is_err(), "Connessione senza token deve fallire");

    // Tentativo con token invalido
    let res_invalid = connect_async(&format!("{server_url}?token=token_invalido")).await;
    assert!(
        res_invalid.is_err(),
        "Connessione con token invalido deve fallire"
    );
}

#[tokio::test]
async fn save_position_uses_client_timestamp() {
    const USER_ID: u32 = 42;

    use std::str::FromStr;
    let options = sqlx::sqlite::SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();

    sqlx::query(
        r#"CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"CREATE TABLE positions (
            id INTEGER PRIMARY KEY,
            journey_id INTEGER NOT NULL,
            lat REAL NOT NULL,
            lon REAL NOT NULL,
            timestamp DATETIME NOT NULL,
            FOREIGN KEY(journey_id) REFERENCES journeys(id)
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"CREATE TABLE journeys (
            id INTEGER PRIMARY KEY,
            started_by INTEGER NOT NULL,
            started_at DATETIME NOT NULL,
            FOREIGN KEY(started_by) REFERENCES users(id)
        );"#,
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO users VALUES (42, 'vehicle_42', 'hash');")
        .execute(&pool)
        .await
        .unwrap();

    let repo = PositionRepository::new(pool.clone());
    let journey_id = repo.create_new_journey(USER_ID).await.unwrap();
    let pos = Coordinates::new(12.34, 67.89, Utc::now());
    repo.save_position(journey_id, &pos).await.unwrap();

    let coord = sqlx::query_as::<_, Coordinates>(
        r#"SELECT lat, lon, timestamp
           FROM positions
           WHERE journey_id = ?"#,
    )
    .bind(journey_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(coord.lat, 12.34);
    assert_eq!(coord.lon, 67.89);
    assert_eq!(coord.timestamp, pos.timestamp);
}
