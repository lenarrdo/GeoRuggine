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
  [`ConnectedClient.last_position` memorizza l'ultima posizione _diversa_ inviata dal client.],
)
```rust
pub fn is_moving(&self, user_id: u32) -> bool {
    let Some(last_position) = client.last_position.as_ref() else {
        return false;
    };

    Utc::now().signed_duration_since(last_position.timestamp) < INACTIVE_THRESHOLD
}
```


== CPU Log e Dimensioni

#list(
  [

    Il server riporta, ogni due minuti, utilizzo e tempo di CPU.
    ```txt
    [2026-11-07 14:25:05] CPU Usage: 0.58% | CPU Time: 701.74ms
    ```
  ],
  [
    Dimensioni eseguibile
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

