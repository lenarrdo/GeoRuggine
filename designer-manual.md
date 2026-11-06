# GEO RUGGINE - Designer Manual

Geo Ruggine is a real-time client/server application for user authentication,
geographical position tracking and bidirectional text messaging. The system is
designed as a distributed application composed of a Rust server and a desktop
client.

The server manages the central application logic, authenticated connections,
connected users, position updates and message routing. It exposes HTTP
endpoints for authentication and a WebSocket channel for real-time
communication.

The client is divided into two cooperating parts:

- a graphical frontend written in React and TypeScript, responsible for the
  user interface and local presentation state;
- a native backend written in Rust and hosted by Tauri, responsible for
  authentication requests, WebSocket communication, session management and
  position transmission.

This manual describes the architecture, communication protocols, main modules,
data flows and operational behavior of both the server and the client.

## Architecture

### Server

The server is a modular Rust application built on the Tokio asynchronous
runtime. It exposes HTTP endpoints through Axum, upgrades authenticated
connections to WebSocket, persists application data in SQLite and exposes a
terminal interface for monitoring connected users.

The server entry point is
[`main.rs`](../georuggine-server/src/main.rs). At startup it loads the
configuration, initializes the database and logger, creates the shared
application state, starts the Axum listener and launches the Ratatui terminal
interface.

The shared state is represented by `AppState`. It contains:

- the `sqlx::SqlitePool`;
- the list of connected clients;
- the Tokio channel used to send events to the TUI;
- the server configuration.

### Server communication protocol

The server exposes versioned HTTP endpoints under `/api/v1`:

| Method | Endpoint | Purpose |
|---|---|---|
| `POST` | `/api/v1/auth/register` | Register a new user |
| `POST` | `/api/v1/auth/login` | Authenticate a user |
| `GET` | `/api/v1/ws?token=<JWT>` | Open an authenticated WebSocket |

HTTP requests and responses use JSON. HTTP is used for operations that follow a
request-response pattern, while WebSocket is used for continuous bidirectional
communication.

The router is assembled in
[`lib.rs`](../georuggine-server/src/lib.rs). Every handler receives the shared
`AppState` through an `Arc`, allowing multiple asynchronous tasks to access the
same resources safely.

### WebSocket connection management

The WebSocket handler is implemented in
[`websocket/mod.rs`](../georuggine-server/src/websocket/mod.rs).

When a client connects:

1. the server extracts the JWT from the query string;
2. the token is validated by the authentication service;
3. a second connection for the same user is rejected;
4. a journey is created for the new connection;
5. the client is inserted into `ClientsList`;
6. independent sending and receiving tasks are started.

Each connected client owns an MPSC channel. The server uses this channel to
send messages to that client without blocking other connections. When either
the sending or receiving task ends, the other task is aborted and the client
is removed from the active-client registry.

Incoming WebSocket messages are deserialized as `ClientIncomingMessage`. The
server currently handles two application-level messages:

```json
{ "type": "position", "lat": 45.0, "lon": 7.0, "timestamp": "..." }
```

```json
{ "type": "text", "contents": "Message text" }
```

Invalid JSON is logged and ignored without terminating the user's connection.

### Server concurrency management

The server uses Tokio tasks instead of creating one operating-system thread for
each operation. This is appropriate because network and database operations
spend most of their time waiting for I/O.

The main concurrent activities are:

- accepting new TCP connections;
- reading and writing WebSocket messages;
- saving positions and text messages;
- sending events to the terminal interface;
- periodically collecting CPU usage information.

The use of asynchronous channels ensures that a slow client or a slow
interface update does not block the processing of unrelated clients.

## Server authentication

The authentication module is located in
[`georuggine-server/src/auth/`](../georuggine-server/src/auth/).

Passwords are never stored in plaintext. During registration the server hashes
the password with Argon2 before persisting the user. During login it retrieves
the stored hash and verifies the supplied password.

When login succeeds, the server creates a JWT containing the user's identity.
The client uses this token when opening the WebSocket. The server validates the
token before accepting the connection.

Authentication therefore has two distinct steps:

1. HTTP login verifies the credentials and returns the authentication result;
2. WebSocket authentication verifies the JWT before enabling real-time
   operations.

## Server geolocation and movement

### Position reception

The server receives position updates through the authenticated WebSocket. For
each update it:

1. identifies the user from the WebSocket session;
2. retrieves the journey associated with that connection;
3. validates and saves the position through `PositionRepository`;
4. updates the position stored in the connected-client registry;
5. determines whether the user is moving or stopped;
6. sends the corresponding event to the TUI.

The server does not trust a user-provided user id to identify the sender. The
identity is taken from the authenticated WebSocket connection.

### Vehicle states

The active client state is derived from received positions:

- **Disconnected**: the WebSocket is not active;
- **Stopped**: the latest position has not changed for the configured
  inactivity interval;
- **Moving**: a change in coordinates has been detected.

When a connection closes, the client is removed from `ClientsList` and a
disconnection event is sent to the TUI.

### Journey persistence

When a WebSocket connection is established, the server creates a journey for
the user. Every position received during that connection is associated with
that journey. This preserves the temporal order of the route and allows the
server to distinguish separate tracking sessions.

## Server messaging

The server supports text messages received from clients and messages sent from
the server to connected clients.

When a text message is received:

1. it is deserialized from the WebSocket payload;
2. a `NewMessage` event is sent to the TUI;
3. the message is persisted through `TextMessageRepository`;
4. the server can route it to the intended recipient.

The connected-client registry provides the in-memory routing information needed
to send a message to one client or to multiple clients. The message data and
the routing data are kept separate: persistence is handled by the message
repository, while active delivery is handled by the WebSocket client registry.

## Server database

The server uses SQLite through `sqlx`. SQLite is suitable for the project
because it is embedded, requires no separate database service and simplifies
local deployment.

The database is initialized by the shared database module at server startup.
Repositories isolate SQL operations by domain:

| Repository | Responsibility |
|---|---|
| `UserRepository` | Users and authentication data |
| `PositionRepository` | Journeys and received coordinates |
| `TextMessageRepository` | Persisted text messages |

The database file is configured through the server environment. The client
never accesses this file directly.

## Server terminal interface

The server includes a Ratatui terminal interface for operational monitoring.
Events are sent to the TUI through an asynchronous Tokio MPSC channel.

The interface can display:

- users connecting and disconnecting;
- current user states;
- received positions;
- incoming text messages;
- server log messages;
- user and journey information.

This interface is independent from the graphical React client. It is an
administrative view running in the server process.

## Server performance monitoring

The server starts a periodic logger using the configured interval. In the
current configuration, CPU information is recorded every two minutes. The
monitoring implementation uses the `sysinfo` crate and writes diagnostic data
to the configured log file.

The monitoring task is asynchronous and does not participate in the
WebSocket message path, so collecting measurements does not block client
communication.

## Server project structure

```text
georuggine-server/
├── src/
│   ├── main.rs                 # Configuration and server bootstrap
│   ├── lib.rs                  # Axum router composition
│   ├── auth/
│   │   ├── auth_service.rs     # Password and JWT operations
│   │   ├── dto.rs              # Authentication DTOs
│   │   └── user_repository.rs  # User persistence
│   ├── messages/
│   │   ├── text/               # Text-message persistence
│   │   └── position/           # Journey and position persistence
│   ├── shared/
│   │   ├── app_state.rs        # Shared application state
│   │   ├── config.rs           # Environment configuration
│   │   ├── database.rs         # SQLite initialization
│   │   └── logger.rs           # Periodic logging
│   ├── websocket/
│   │   ├── clients.rs          # Connected-client registry
│   │   ├── dto.rs              # WebSocket DTOs
│   │   └── mod.rs               # Upgrade and message handling
│   └── tui/                    # Terminal monitoring interface
└── Cargo.toml
```

### Main server modules

| Module | Responsibility | Main files |
|---|---|---|
| `auth/` | Registration, login, password hashing and JWT validation | `auth_service.rs`, `user_repository.rs` |
| `messages/position/` | Journeys, position storage and movement updates | `repository.rs`, `service.rs` |
| `messages/text/` | Text-message storage | `repository.rs` |
| `websocket/` | Authentication of connections and message transport | `clients.rs`, `mod.rs` |
| `shared/` | Configuration, database, state and logging | `app_state.rs`, `database.rs`, `logger.rs` |
| `tui/` | Server-side terminal administration | `mod.rs`, `chat_box/`, `user_list/` |

The server is a modular monolith: all modules run in one binary, but each
domain has its own data-access and application responsibilities. This keeps
deployment simple while preventing the WebSocket layer, authentication and
database code from being mixed together.



### Client

The client follows a two-layer architecture. React is responsible for the
graphical interface and its local state, while Tauri/Rust is responsible for
the native application logic and communication with the server.

```text
georuggine-client/
├── src/                         # React frontend
│   ├── components/              # Graphical components
│   │   ├── auth/                # Login and registration
│   │   ├── chat/                # Chat interface
│   │   └── upload/              # Position input
│   ├── services/
│   │   └── tauriBridge.ts       # React/Tauri communication
│   ├── types/                   # TypeScript data types
│   ├── App.tsx                  # Main UI component
│   └── main.tsx                 # React entry point
├── src-tauri/
│   └── src/                     # Native Rust backend
│       ├── auth/                # Authentication commands and services
│       ├── messages/            # Chat and position commands
│       ├── websocket/           # WebSocket connection
│       ├── state.rs             # Shared client state
│       └── lib.rs               # Tauri application bootstrap
└── package.json
```

The frontend never opens an HTTP or WebSocket connection directly. It
communicates with Rust through Tauri commands (`invoke`) and Tauri events
(`listen`). This keeps network details outside the graphical components.

### Communication protocol

The client uses two communication protocols:

- **HTTP/JSON** for registration and login;
- **WebSocket/JSON** for the persistent real-time connection used by chat and
  position updates.

HTTP is appropriate for registration and login because these operations follow
a request-response pattern. WebSocket is used after login because both client
and server may send messages at any time without repeatedly opening new
connections.

The Rust backend uses `reqwest` for HTTP and `tokio-tungstenite` for WebSocket
communication. The React frontend accesses both through the bridge in
[`tauriBridge.ts`](../georuggine-client/src/services/tauriBridge.ts).

### Connection and concurrency management

The native client uses the Tokio asynchronous runtime. After a successful
login, the WebSocket connection is split into independent sending and
receiving tasks:

1. the transmission task consumes messages from an asynchronous MPSC channel;
2. the reception task continuously reads messages from the WebSocket;
3. the tracking task periodically sends the next simulated coordinate.

The graphical UI and the tracking task are producers. The transmission task is
the only consumer that writes to the WebSocket. This prevents concurrent
writes on the same connection.

The shared state in [`state.rs`](../georuggine-client/src-tauri/src/state.rs)
stores the MPSC sender and the abort handles for all background tasks. When the
connection is closed, the client aborts the tasks, drops the channel sender
and releases the WebSocket.

### Application shutdown

When the Tauri window is closed, the native backend calls `disconnect_ws`.
This explicitly terminates:

- the background tracking task;
- the WebSocket transmission task;
- the WebSocket reception task;
- the MPSC channel connecting producers to the transmission task.

The same cleanup is executed during logout and before opening a new WebSocket
connection. This prevents tasks and connections from a previous session from
remaining active.

## Authentication

The graphical authentication interface is implemented by `LoginForm` and
`RegisterForm`. Both components submit their data to `App`, which delegates
the operation to the Tauri bridge.

Registration follows this flow:

1. the user enters a username and password;
2. React invokes the `register` Tauri command;
3. the native backend validates the input;
4. `AuthService` sends an HTTP request to the server;
5. the result is returned to React as an `AuthResult`;
6. the interface shows either an error or a success message.

The client performs preliminary validation locally. Empty usernames are
rejected and registration passwords must contain at least four characters.
Server-side validation remains authoritative.

Login follows these steps:

1. React invokes the `login` Tauri command;
2. `AuthService` sends the credentials through HTTP;
3. the native backend receives the authentication token;
4. the backend opens the WebSocket using the token;
5. only after the WebSocket is active, the complete session is stored in the
   protected Rust state;
6. React receives a successful result without receiving the token.

The login implementation is in
[`auth/controller.rs`](../georuggine-client/src-tauri/src/auth/controller.rs).
The HTTP operations are implemented in
[`auth_service.rs`](../georuggine-client/src-tauri/src/auth/auth_service.rs).

## Geolocation and movement

### Position transmission

The client can send a sequence of coordinates through the
`upload_coordinates` command. Coordinates are converted into position DTOs
containing latitude, longitude and an optional UTC timestamp.

The position is placed on the shared MPSC channel and is then transmitted by
the WebSocket sending task. React therefore does not need to know how the
coordinate message is serialized or transmitted.

### Movement simulation

The current tracking implementation accepts a vector of coordinates. The
background tracking task sends one coordinate every 30 seconds:

1. a tracking task is started after coordinates are provided;
2. a Tokio interval waits 30 seconds between transmissions;
3. the next coordinate is converted into a `PositionUpdate`;
4. the update is enqueued on the shared WebSocket channel;
5. the task ends when all coordinates have been sent.

Starting a new tracking task aborts the previous tracking task. If the
WebSocket is unavailable, the task stops and the client disconnects its
connection.

The implementation is in
[`tracking.rs`](../georuggine-client/src-tauri/src/messages/position/tracking.rs).

### Position data

The client-side position DTO contains:

```text
PositionUpdate {
    lat: f64,
    lon: f64,
    timestamp: Option<String>
}
```

The client does not calculate route statistics or access the persistence
layer. It only collects, timestamps and transmits position updates. Analysis
and persistence belong to the server component.

## Messaging

Communication between the client and the server is bidirectional.

### Sending messages

When the user sends a chat message:

1. `ChatInput` passes the text to `App`;
2. `App` displays the message immediately as an optimistic update;
3. `apiSendChatMessage` invokes `send_chat_message`;
4. the Tauri command serializes the message;
5. the message is placed on the shared MPSC channel;
6. the WebSocket transmission task sends it to the server.

If sending fails, the message is marked as failed and the input text is
restored so that the user can retry.

### Receiving messages

The reception task reads text messages from the WebSocket. Valid messages are
emitted to React using the `chat-message-received` Tauri event. The frontend
listener then:

1. converts the timestamp into a displayable time;
2. identifies the sender as the server;
3. checks for an existing duplicate;
4. appends the message to the local UI state.

The frontend registers this listener only while a user is logged in and removes
it when the session ends.

### Disconnection notification

An unexpected WebSocket closure emits the `ws-disconnected` event. React
receives the event and updates the UI so that the user is informed that the
server connection is no longer available.

## Project Structure

The client source code is organized as follows:

```text
georuggine-client/
├── src/
│   ├── App.tsx
│   ├── App.css
│   ├── main.tsx
│   ├── components/
│   │   ├── auth/
│   │   │   ├── AuthLayout.tsx
│   │   │   ├── LoginForm.tsx
│   │   │   └── RegisterForm.tsx
│   │   ├── chat/
│   │   │   ├── ChatHeader.tsx
│   │   │   ├── ChatInput.tsx
│   │   │   ├── ChatMessageItem.tsx
│   │   │   ├── ChatSidebar.tsx
│   │   │   └── ChatView.tsx
│   │   └── upload/
│   │       └── FileUploader.tsx
│   ├── services/
│   │   └── tauriBridge.ts
│   ├── types/
│   │   ├── auth.ts
│   │   └── chat.ts
│   └── integration/
│       └── ipc.integration.test.tsx
└── src-tauri/
    └── src/
        ├── auth/
        │   ├── auth_service.rs
        │   ├── commands.rs
        │   ├── controller.rs
        │   └── models.rs
        ├── messages/
        │   ├── chatmessage/
        │   ├── position/
        │   └── dto.rs
        ├── websocket/
        │   └── mod.rs
        ├── state.rs
        ├── lib.rs
        └── main.rs
```

### Main backend modules

The Tauri backend is organized into independent modules with precise
responsibilities:

| Module | Responsibility | Main files |
|---|---|---|
| `auth/` | Input validation, registration, login and logout | `commands.rs`, `controller.rs`, `auth_service.rs` |
| `messages/chatmessage/` | Sending text messages from the UI | `commands.rs`, `controller.rs`, `dto.rs` |
| `messages/position/` | Position DTOs and periodic tracking | `command.rs`, `controller.rs`, `tracking.rs` |
| `websocket/` | WebSocket connection, sending, reception and events | `mod.rs` |
| `state.rs` | Session, channel and task ownership | `state.rs` |

### Frontend

The frontend is a React application built with TypeScript, Vite and
React-Bootstrap. It is composed of small components and does not contain
network implementation details.

[`App.tsx`](../georuggine-client/src/App.tsx) coordinates the main UI state:

- `currentUser` identifies the active session;
- `authView` selects login or registration;
- `messages` contains messages rendered by the chat;
- `isLoading` controls authentication feedback;
- `authError` and `authSuccess` contain user-facing status messages;
- `serverDown` represents an unexpected disconnection.

The visual components receive data and callbacks through props. Their behavior
is covered by colocated tests such as
[`LoginForm.test.tsx`](../georuggine-client/src/components/auth/LoginForm.test.tsx)
and
[`ChatView.test.tsx`](../georuggine-client/src/components/chat/ChatView.test.tsx).

### Tauri commands and events

The commands registered in `lib.rs` are:

| Command | Purpose |
|---|---|
| `register` | Register a new account |
| `login` | Authenticate and open the WebSocket |
| `logout` | Close the connection and clear the session |
| `get_current_user` | Read the current username |
| `send_chat_message` | Send a text message |
| `upload_coordinates` | Send position data |

The events emitted from Rust to React are:

| Event | Purpose |
|---|---|
| `chat-message-received` | Deliver a server message to the chat |
| `ws-disconnected` | Notify React about an unexpected disconnection |

## Testing

The client contains unit, component and integration tests:

```text
src/
├── App.test.tsx
├── components/**/*.test.tsx
├── services/tauriBridge.test.ts
└── integration/ipc.integration.test.tsx
```

The tests verify component rendering, form submission, error handling, chat
updates, Tauri bridge behavior and the WebSocket event pipeline using a fake
backend where appropriate.

The main commands are:

```bash
cd georuggine-client
npm test
npm run build
npm run tauri dev
```
