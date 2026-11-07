= Server

== Organizzazione Database

#figure(
  table(
    align: left,
    columns: 2,
    table.header([Tabella], [Scopo]),
    [`users`], [Memorizza username e password degli utenti.],
    [`journeys`], [Ogni nuova connessione corrisponde a una journey distinta.],
    [`positions`], [Memorizza le posizioni inviate durante una journey.],
    [`text_messages`],
    [Memorizza i messaggi scambiati tra il server e l'utente.],
  ),
)

== Strutture Principali

```rust
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct Journey {
    _id: u32,
    coordinates: Vec<Coordinates>,
}

pub struct TextMessage {
    pub contents: String,
    pub timestamp: DateTime<Utc>,
}
```

== Autenticazione

```json
// POST
{
  "username": "foo",
  "password": "bar"
}
```

Il server espone due endpoint:

#enum(
  [`/api/v1/auth/register`: memorizza username e hash della password (Argon2) nel database.],
  [`/api/v1/auth/login`: se l'username esiste e l'hash della password inserita corrisponde all'hash memorizzato, restituisce un token JWT utilizzabile per aprire il WebSocket.],
)

== Apertura del WebSocket

#figure(image("/figures/server-opens-websocket.svg", width: 90%))

== Gestione dei client attivi

I client attivi vengono memorizzati in una `ClientsList`:
```rust
pub struct ClientsList {
    pub list: DashMap<u32, ConnectedClient>,
}
```

Dove un `ConnectedClient` è una struttura implementata come segue:
```rust
pub struct ConnectedClient {
    tx: Sender<TextMessage>, // per inviare messaggi all'utente
    journey_id: u32,
    last_position: Option<Coordinates>,
}
```

== Gestione del Movimento

#list(
  [
    `ConnectedClient.last_position` memorizza l'ultima posizione _diversa_ inviata dal client.
    ```rust
    pub fn is_moving(&self, user_id: u32) -> bool {
        let Some(last_position) = client.last_position.as_ref() else {
            return false;
        };

        Utc::now().signed_duration_since(last_position.timestamp) < INACTIVE_THRESHOLD
    }
    ```
  ],
  [
    In base al risultato, viene comunicato un evento all'interfaccia per riflettere il cambiamento.
    ```rust
    let event = if state.clients.is_moving(user_id) {
        AppEvent::UserMoving(user_id)
    } else {
        AppEvent::UserStopped(user_id)
    };
    ```
  ],
)

#pagebreak()

#list(
  [
    Per calcolare le statistiche di movimento, è possibile specificare quali record recuperare.
    ```rust
    pub async fn get_positions(
        &self,
        user_id: u32,
        time_frame: TimeFrame, // Today, ThisWeek, ThisMonth
    ) -> Result<Vec<Journey>, sqlx::Error>;
    ```
  ],
  [
    Poi delle funzioni di servizio calcolano le statistiche solamente sulle journey recuperate.
    ```rust
    ///                                     time moving ---v         v--- time not moving
    pub fn get_movement_stats(journeys: &[Journey]) -> (Duration, Duration);
    pub fn get_average_speed(journeys: &[Journey]) -> Option<f64>;
    pub fn get_total_distance(journeys: &[Journey]) -> f64;
    ```
  ],
)

== Terminal User Interface

#grid(
  columns: 2,
  column-gutter: 10pt,
  [#image("/figures/georuggine-tui.webp")], [#image("/figures/user-info.webp")],
)

== CPU Log e Dimensioni

#list(
  [

    Il server riporta, ogni due minuti, utilizzo e tempo di CPU.
    ```txt
    [2026-11-07 14:25:05] CPU Usage: 0.58% | CPU Time: 701.74ms
    ```
  ],
  [
    Dimensioni degli eseguibili
    #align(center)[
      #grid(
        columns: 2,
        column-gutter: 100pt,
        [
          *Linux*
          #figure(
            table(
              columns: 2,
              table.header([Client], [Server]),
              [], [],
            ),
          )
        ],
        [
          *Windows*
          #figure(
            table(
              columns: 2,
              table.header([Client], [Server]),
              [], [],
            ),
          )
        ],
      )
    ]
  ],
)

