# GeoRuggine Server - Technical Documentation

## System Architecture & Tech Stack

The server follows a modular monolith architectural style, grouping domain capabilities into cohesive modules.

```text
georuggine-server/
├── src/
│   ├── main.rs                 # Configuration, channel initialization, bootstrap, and TUI lifecycle
│   ├── lib.rs                  # Axum router composition and route mappings
│   ├── auth/
│   │   ├── auth_service.rs     # Password hashing and JWT generation/validation
│   │   ├── dto.rs              # Authentication requests, responses, and error types
│   │   ├── mod.rs              # Registration and login HTTP handlers
│   │   └── user_repository.rs  # User entity queries and persistence
│   ├── messages/
│   │   ├── mod.rs              # Incoming polymorphic message envelope
│   │   ├── position/           # Position persistence, trajectory analysis, and journey stats
│   │   └── text/               # Text-message persistence and transfer objects
│   ├── shared/
│   │   ├── app_state.rs        # Global shared application state (Arc<AppState>)
│   │   ├── config.rs           # Environment variable parsing (.env)
│   │   ├── database.rs         # SQLite initialization, schema creation, and pooling
│   │   ├── logger.rs           # Periodic background metric and log flushing
│   │   └── utils.rs            # Global constants (e.g. INACTIVE_THRESHOLD, SERVER_ID) and utility functions
│   ├── websocket/
│   │   ├── clients.rs          # Thread-safe connected-client registry
│   │   ├── dto.rs              # WebSocket handshake query parameters and responses
│   │   └── mod.rs              # WS upgrade, connection lifecycle, and message handling
│   └── tui/                    # Terminal User Interface (Ratatui-based monitoring console)
└── Cargo.toml
```

The server application relies on the following core crates:

- **Tokio** (Runtime & Concurrency): Provides the asynchronous multithreaded runtime, driving task scheduling, non-blocking I/O, timers, and inter-task communication channels (`mpsc`).
- **Axum** (Web Framework & Routing): Handles HTTP endpoint routing, JSON payload extraction/validation, and WebSocket upgrades.
- **SQLx** (Database & Persistence): Provides async, non-blocking connection pooling and SQL query execution over SQLite.
- **Security & Authentication**:
  - **Argon2**: State-of-the-art password hashing algorithm resilient against GPU/ASIC brute-force attacks via memory-hard computational cost.
  - **Jsonwebtoken**: Encodes and verifies HMAC-signed JSON Web Tokens (JWT) for stateless authentication.
- **Serde & Serde JSON** (Serialization): Facilitates bidirectional deserialization and serialization between Rust data structures and JSON payloads across REST and WebSocket boundaries.
- **Ratatui & Crossterm** (Terminal User Interface): Powers a terminal-based administrator dashboard to monitor connected clients, inspect movement metrics, observe real-time logs, and broadcast/receive chat messages.
- **DashMap** (Concurrent Data Structures): Provides high-throughput concurrent hash map access for tracking active WebSocket sessions without coarse-grained read-write locks.

## Database

The database is built on top of SQLite via SQLx connection pooling. The relational schema comprises four primary tables:

1. **`users`**:
   - `id INTEGER PRIMARY KEY`: Unique numeric user identifier. The special ID `0` is pre-populated as `system@local` representing the server itself.
   - `username TEXT UNIQUE NOT NULL`: Unique username chosen during registration.
   - `password_hash TEXT NOT NULL`: Argon2-encoded password hash string (containing salt, parameters, and derived key).

2. **`journeys`**:
   - `id INTEGER PRIMARY KEY`: Unique journey identifier created automatically each time a user initiates a live WebSocket session.
   - `started_by INTEGER NOT NULL`: References `users(id)`.
   - `started_at DATETIME NOT NULL`: UTC timestamp when the session/journey was initiated.

3. **`positions`**:
   - `id INTEGER PRIMARY KEY`: Unique position entry identifier.
   - `journey_id INTEGER NOT NULL`: Foreign key referencing `journeys(id)`.
   - `lat REAL NOT NULL`: Latitude coordinate.
   - `lon REAL NOT NULL`: Longitude coordinate.
   - `timestamp DATETIME NOT NULL`: Timestamp of coordinate acquisition.

4. **`text_messages`**:
   - `id INTEGER PRIMARY KEY`: Unique message identifier.
   - `sender_id INTEGER NOT NULL`: References `users(id)`.
   - `receiver_id INTEGER NOT NULL`: References `users(id)`.
   - `journey_id INTEGER NOT NULL`: References `journeys(id)` corresponding to the active session.
   - `timestamp DATETIME NOT NULL`: UTC timestamp when the message was recorded.
   - `contents TEXT`: Text body of the message.

The high-level idea is the following: each new connection opened by a user corresponds to a new journey, during which the user may send text messages and position updates.

## Authentication & Security

All user identity and credential management logic is encapsulated within the [`src/auth`](../src/auth/) module.

### Account Creation and Login

The Axum router exposes two public REST endpoints under the `/api/v1` namespace:

- **`POST /api/v1/auth/register`**:
  1. Accepts an `AuthRequest` JSON body containing `username` and `password`.
  2. Validates that credentials are non-empty.
  3. Uses `Argon2::default().hash_password(...)` to generate a salted and computationally secure password hash.
  4. Inserts the new record into the `users` table via `UserRepository`. If the username is already registered, an explicit `409 Conflict` (`UsernameAlreadyExists`) error is returned.
  5. Upon success, notifies the TUI event loop via `AppEvent::UserRegistered` so the administrator's user list immediately reflects the newly registered user, returning `201 Created`.

- **`POST /api/v1/auth/login`**:
  1. Accepts an `AuthRequest` JSON payload and validates non-emptiness.
  2. Fetches the corresponding user record from the SQLite database via `UserRepository::find_by_username`. If the user does not exist, an invalid credentials error (`401 Unauthorized`) is returned.
  3. Uses `Argon2::default().verify_password(...)` to verify the plain-text password against the stored PHC-formatted password hash. If verification fails, `401 Unauthorized` is returned.
  4. If verification succeeds, generates a signed JWT and returns `200 OK` with an `AuthResponse` containing the token.

### JWT (JSON Web Tokens)

Upon successful login, `AuthService::generate_jwt` issues a signed JWT containing claims:

- `user_id`: The numeric user ID.
- `username`: The user's handle.
- `exp`: Expiration timestamp set to 24 hours from issuance (`Utc::now() + Duration::hours(24)`).

The token is encoded and signed using an HMAC-SHA256 secret configured in the environment (`JWT_SECRET` variable). WebSocket handshake requests require this token to establish an active streaming session.

## Live Messaging

Real-time bidirectional communication is managed inside [`src/websocket`](../src/websocket).

### WebSocket Lifecycle & Concurrency Architecture

The WebSocket endpoint is exposed at `GET /api/v1/ws?token=<jwt_token>`.

```mermaid
sequenceDiagram
    autonumber
    actor Client
    participant Axum as Axum Router
    participant WS as WebSocket Handler
    participant DashMap as ClientsList (DashMap)
    participant Tasks as Tokio Tasks (Send / Recv)
    participant TUI as TUI Event Channel

    Client->>Axum: GET /api/v1/ws?token=JWT
    Axum->>WS: Handshake upgrade
    WS->>WS: Validate JWT claims
    WS->>DashMap: Check if user_id already connected
    alt Already connected
        WS-->>Client: 403 Forbidden (Single connection rule)
    else Authorized & Unique
        WS->>WS: Create Journey in DB
        WS->>DashMap: Register user with Sender&lt;TextMessage&gt;
        WS->>TUI: Emit AppEvent::UserConnected
        WS->>Tasks: Spawn send_task & recv_task
        Tasks-->>Client: Established WebSocket Stream
    end
```

The connection workflow proceeds as follows:

1. **Handshake & Verification**: `ws_handler` parses the `token` query parameter and verifies it against `AuthService::validate_jwt`. Invalid tokens result in `401 Unauthorized`.
2. **Single-Connection Enforcement**: The server enforces a single concurrent connection per user ID. If the user is already present in `ClientsList`, the upgrade is rejected with `403 Forbidden`.
3. **Session Initialization & Channel Setup**:
   - A new journey record is inserted into SQLite (`journeys` table).
   - A bounded Tokio `mpsc::channel::<TextMessage>(100)` is created for the user.
   - The user is added to `ClientsList` storing their `mpsc::Sender<TextMessage>`, journey ID, and initial position.
   - An `AppEvent::UserConnected(user_id)` event is emitted to update the server's TUI.
4. **Task Splitting**: The upgraded `WebSocket` stream is split into a sink (`ws_sender`) and a stream (`ws_receiver`). Two distinct concurrent Tokio tasks are spawned:
   - **`send_task`**: Awaits messages from the user's dedicated `mpsc::Receiver`. When a message arrives (e.g., sent from the server TUI), it serializes it to JSON and sends it over the WebSocket sink.
   - **`recv_task`**: Continuously reads incoming WebSocket frames from `ws_receiver` and forwards them to `process_message`.
5. **Termination via `tokio::select!`**: The two tasks run alongside each other in a `tokio::select!` block. If either task encounters an error or reaches EOF (e.g. client disconnects or network drops), the opposing task is aborted, the user is removed from `ClientsList`, and an `AppEvent::UserDisconnected(user_id)` event is published.

### Message Deserialization & Payload Types

Incoming text frames are parsed using `serde_json` into the `ClientIncomingMessage` enum defined in [`src/messages/mod.rs`](../src/messages/mod.rs). Using Serde's `#[serde(untagged)]` attribute, incoming JSON payloads are automatically mapped to one of two message variants:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(Coordinates),
    Text(TextMessage),
}
```

- **`ClientIncomingMessage::Text(TextMessage)`**:
  - Contains `{ "text": "...", "timestamp": "..." }`.
  - Dispatches `AppEvent::NewMessage((user_id, text_msg))` to the server TUI chat box.
  - Persists the message into SQLite via `TextMessageRepository::save_message` with `sender_id = user_id`, `receiver_id = SERVER_ID (0)`, and the active `journey_id`.

- **`ClientIncomingMessage::Position(Coordinates)`**:
  - Contains `{ "lat": f64, "lon": f64, "timestamp": "..." }`.
  - Persists the coordinates into the `positions` table linked to the current `journey_id`.
  - Updates client movement state and triggers TUI updates (detailed in the next section).

If an incoming frame does not match either JSON schema, it is logged as an unstructured message via `AppEvent::LogMessage` without terminating the connection.

## User Handling & Mobility Tracking

Active online clients and their run-time metrics are managed through [`ClientsList`](../src/websocket/clients.rs).

### Concurrent State with `DashMap`

Rather than synchronizing connected clients through a monolithic `Arc<RwLock<HashMap<u32, ConnectedClient>>>`, GeoRuggine employs a sharded [`DashMap<u32, ConnectedClient>`](../src/websocket/clients.rs).

- **Reduced Lock Contention**: `DashMap` divides storage across multiple internal lock stripes. Reads and updates to different client sessions proceed concurrently without blocking one another.
- **Ergonomic API**: Avoids lock poisoning hazards and complex nested guard management.

Each entry stores a `ConnectedClient`:

```rust
pub struct ConnectedClient {
    tx: Sender<TextMessage>,
    journey_id: u32,
    last_position: Option<Coordinates>,
}
```

### Movement Analysis & Inactivity Detection

Clients periodically report their GPS coordinates. The server evaluates user movement dynamics as follows:

1. **Selective Position Updates**: When a position is received, `ClientsList::update_position` compares incoming coordinates against `client.last_position`:

   ```rust
   if client.last_position.as_ref().is_none_or(|last| *last != new) {
       client.last_position = Some(new);
   }
   ```

   If a client submits the same latitude and longitude repeatedly, `last_position` is **not** overwritten. Consequently, `last_position.timestamp` retains the timestamp when the client first arrived at that stationary point.

2. **Inactivity Detection**: The helper `ClientsList::is_moving(user_id)` evaluates whether the client is actively moving:

   ```rust
   Utc::now().signed_duration_since(last_position.timestamp) < INACTIVE_THRESHOLD
   ```

   By default, `INACTIVE_THRESHOLD` is set to **3 minutes**. If no new distinct coordinate has been recorded within 3 minutes, the user is classified as stopped/inactive. The server then dispatches `AppEvent::UserStopped(user_id)` or `AppEvent::UserMoving(user_id)` to update the TUI.

3. **Distance Computation**: physical distance between geographic points is computed using the Haversine formula implemented on `Coordinates` ([`src/messages/position/models/coordinates.rs`](../src/messages/position/models/coordinates.rs)):

   $$
   \Delta\sigma = 2 \arcsin \left( \sqrt{\sin^2\left(\frac{\Delta\phi}{2}\right) + \cos(\phi_1)\cos(\phi_2)\sin^2\left(\frac{\Delta\lambda}{2}\right)} \right)
   $$

   $$
   d = R \cdot \Delta\sigma \quad (R \approx 6371\text{ km})
   $$

4. **Trajectory & Journey Statistics**: The service module ([`src/messages/position/service.rs`](../src/messages/position/service.rs)) aggregates trajectory data over one or multiple journeys:
   - **Total Distance**: Evaluates distance across adjacent coordinate pairs using sliding windows.
   - **Average Speed**: Computes total travelled distance divided by total elapsed hours.
   - **Movement vs. Stationary Time**: Splits the user's logged trajectory into active moving time and stationary time based on coordinate changes and the `INACTIVE_THRESHOLD` time limit.

## Terminal User Interface (TUI)

The server integrates an interactive terminal dashboard powered by **Ratatui** and **Crossterm**, allowing server operators to monitor users, view streaming logs, inspect journeys, and send direct messages in real time.

### TUI Channel Architecture

During startup in [`main.rs`](../src/main.rs), an asynchronous Tokio channel is established:

```rust
let (tx, rx) = mpsc::channel(100);
```

- The sending end `tx` (`Sender<AppEvent>`) is passed into `AppState` and cloned across the HTTP/WebSocket handlers.
- The receiving end `rx` (`Receiver<AppEvent>`) is transferred to `App::new(...)`, which drives the Ratatui event loop on the main thread.

### Event Dispatch System

The TUI reacts to events defined by the `AppEvent` enum:

```rust
pub enum AppEvent {
    UserRegistered((u32, String)), // Adds newly registered user to the UI list
    UserConnected(u32),            // Marks user as online
    UserDisconnected(u32),         // Marks user as offline
    UserMoving(u32),               // Marks user status as moving
    UserStopped(u32),              // Marks user status as stopped/inactive
    LogMessage(String),            // Appends structured or system log entries to the logs tab
    NewMessage((u32, TextMessage)),// Adds received text message to the chat view
    Tick,                          // Periodic refresh tick to update the UI
}
```

This decoupled architecture allows Axum request handlers and WebSocket worker threads to dispatch UI updates non-blockingly while Ratatui handles rendering and input processing independently.
