= Client

== Architettura del Client


  - Applicazione desktop realizzata con *Tauri*.
  - *React* gestisce l'interfaccia grafica.
  - *Rust* gestisce logica, sessione e comunicazione col server.

  #align(center + horizon)[
    #image("../images/TauriAppLogo.svg", height: 40%)
  ]

  == Login/Registrazione

  - L’utente può registrarsi o effettuare il login.
- La registrazione porta alla schermata di login.
- Dopo il login si accede alle funzionalità principali dell’applicazione.

#grid(
  columns: (1.1fr, 0.9fr),
  gutter: 1em,
  align: center,
)[
  #image("../../../georuggine-client/docs/images/login.png", width: 90%)
][
  #image("../../../georuggine-client/docs/images/register.png", width: 90%)
]

== Apertura Websocket

- Dopo il login, il client tenta di stabilire la connessione WebSocket.
- Una volta stabilita la connessione vengono creati due task:
  - *Task di invio*: trasmette i messaggi verso il server.
  - *Task di ricezione*: ascolta e gestisce i messaggi in arrivo.
  - *Task di tracking*: invia periodicamente le coordinate (questo solo quando il client ha caricato le coordinate).
- I task sono coordinati dallo stato condiviso `AppState`, che conserva anche
  gli `AbortHandle` dei task.

```rust
pub struct AppState {
    pub session: Session,
    pub ws_tx: WsTx,
    pub tracking_abort: Handle,
    pub send_abort: Handle,
    pub recv_abort: Handle,
}
```

== Invio Messaggi e Coordinate

- I messaggi vengono inviati al server e quelli ricevuti vengono mostrati nella chat.
- Le coordinate vengono lette da un file JSON e inviate tramite WebSocket.
- La prima coordinata viene inviata subito, le successive ogni 30 secondi.

#grid(
  columns: (1.1fr, 0.9fr),
  gutter: 1em,
  align: horizon,
)[
#text(size: 8pt)[
```rust
pub enum ClientIncomingMessage {
    Text(TextMessage),
    Position(PositionUpdate),
}

pub struct TextMessage {
    pub contents: String,
    pub timestamp: String,
}

pub struct PositionUpdate {
    pub lat: f64,
    pub lon: f64,
    pub timestamp: String,
}
```
]
][
  #grid[
    #image("../../../georuggine-client/docs/images/chat.png", width: 100%)
  ]
]

== Logout e disconnessione

- Il logout interrompe i task attivi e chiude il WebSocket.
- La sessione viene rimossa e il client torna alla schermata di login.
- In caso di errore di connessione, l'evento di disconnessione informa React.

#grid(
  columns: (1fr, 1fr),
  gutter: 1em,
  align: center,
)[
  #image("../../../georuggine-client/docs/images/logout.png", width: 90%)
][
  #image("../../../georuggine-client/docs/images/errorServer.png", width: 90%)
]

#text(size: 8pt)[
```rust
pub fn disconnect_ws(&self) {
    if let Ok(mut guard) = self.tracking_abort.lock() {
        if let Some(handle) = guard.take() {
            handle.abort();
        }
    }

    if let Ok(mut guard) = self.send_abort.lock() {
        if let Some(handle) = guard.take() {
            handle.abort();
        }
    }

    if let Ok(mut guard) = self.recv_abort.lock() {
        if let Some(handle) = guard.take() {
            handle.abort();
        }
    }

    if let Ok(mut tx_guard) = self.ws_tx.lock() {
        *tx_guard = None;
    }
}
```
]
