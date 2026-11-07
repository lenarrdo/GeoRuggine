# User Manual - GEO-RUST

Geo Ruggine is a desktop client/server application for user authentication,
real-time text messaging and geographical position tracking. The client is a
Tauri application with a React graphical interface and a Rust native backend.
The server is a separate Rust process.

## Table of Contents

1. [Prerequisites](#1-prerequisites)
2. [Installation](#2-installation)
3. [Building](#3-building)
4. [Running](#4-running)
5. [Usage](#5-usage)
6. [Vehicle States](#6-vehicle-states)
7. [Movement Simulation](#7-movement-simulation)
8. [Route Analysis](#8-route-analysis)
9. [Messaging](#9-messaging)
10. [Available Commands](#10-available-commands)
11. [Coordinate File Format](#11-coordinate-file-format)
12. [Accounts](#12-accounts)

## 1. Prerequisites

Before using GEO-RUST, install:

### 1.1 Rust and Cargo

- **Rust**
- **Cargo**

Installation: [https://rustup.rs](https://rustup.rs/)

Rust is required for both the server and the Tauri backend of the client.

### 1.2 Node.js and npm

- **Node.js**, including **npm**

Node.js and npm are required to install the React dependencies and run the
Vite/Tauri commands.

### 1.3 Tauri prerequisites

Tauri also requires the platform-specific development libraries documented at
[https://tauri.app/start/prerequisites/](https://tauri.app/start/prerequisites/).

The application is intended to run on desktop platforms supported by Tauri,
including Linux, Windows and macOS when their corresponding Tauri
prerequisites are installed.

## 2. Installation

### 2.1 Getting the project

Extract the archive or clone the repository, then enter the project folder:

```bash
cd GeoRust/G5
```

### 2.2 Installing server configuration

Create the server environment file from the provided template:

```bash
cd georuggine-server
cp .env.template .env
```

The default configuration starts the server on `127.0.0.1:3000`, stores the
SQLite database in `data/georust.sqlite` and writes logs to
`logs/georuggine.log`. Change `.env` before starting the server if a different
configuration is required.

### 2.3 Installing client dependencies

```bash
cd ../georuggine-client
npm install
```

## 3. Building

### 3.1 Building the server

From `georuggine-server/`:

```bash
cargo build --release
```

The executable is generated in:

```text
georuggine-server/target/release/georuggine-server
```

On Windows, the executable has the `.exe` extension.

### 3.2 Building the client

From `georuggine-client/`:

```bash
npm run build
```

This type-checks the TypeScript code and creates the frontend bundle in
`dist/`.

To create a native Tauri bundle:

```bash
npm run tauri build
```

The generated installers are placed under
`georuggine-client/src-tauri/target/release/bundle/`.

## 4. Running

### 4.1 Starting the server

Open a terminal in `georuggine-server/`:

```bash
cargo run
```

The server starts using the values in `.env`. With the default configuration,
the HTTP API is available at:

```text
http://127.0.0.1:3000/api/v1
```

The WebSocket endpoint used by the client is:

```text
ws://127.0.0.1:3000/api/v1/ws
```

The server creates the configured SQLite database and log directories when
required by the local setup.

### 4.2 Starting the client

Open a second terminal in `georuggine-client/`:

```bash
npm run tauri dev
```

This starts the Vite development server and opens the Tauri desktop window.
The client is configured to connect to the default local server at
`127.0.0.1:3000`.

The client can also be started after building an installer with
`npm run tauri build`.

## 5. Usage

1. Start the server (see [4.1](#41-starting-the-server)).
2. Start the Tauri client (see [4.2](#42-starting-the-client)).
3. Register a new account from the registration screen, or log in with an
   existing account.
4. After a successful login, the client opens an authenticated WebSocket.
5. Use the chat area to send text messages.
6. Use **Coordinate File** to select a JSON file and upload a route.
7. The client sends one coordinate every 30 seconds.
8. Use **Logout** to close the session and the WebSocket connection.

If the server connection is lost while the user is logged in, the client shows
an error dialog. Select **Back to login** to close the local session and return
to the authentication screen.

## 6. Vehicle States

The server tracks the state of each connected client:

- **Disconnected**: the client has no active WebSocket connection;
- **Stopped**: the client is connected but its position is not changing;
- **Moving**: the server detects a change in coordinates.

The client does not manually select these states. It only transmits position
updates; the server determines the current state from the received data.

## 7. Movement Simulation

The client simulates a route by reading an ordered list of coordinate points
from a JSON file. After the upload:

1. the Rust backend validates the JSON;
2. the coordinate list is converted into position updates;
3. a background tracking task starts;
4. one position is sent every 30 seconds;
5. the task stops when the list is exhausted.

Uploading a new coordinate file stops the previous tracking task before starting
the new one. Logging out or closing the application also stops tracking.

The current client sends the coordinates exactly as they appear in the file. It
does not interpolate intermediate points or calculate route statistics locally.

## 8. Route Analysis

The client currently does not expose a route-analysis screen or a route-analysis
command. Its responsibility is to upload position samples to the server.

Distance, speed, movement duration, pause duration and time-window analysis
must be provided by the server component when the corresponding server API is
available.

## 9. Messaging

Communication is bidirectional:

- the client sends text messages to the server through the WebSocket;
- the client receives messages from the server through Tauri events.

When a user sends a message, it is immediately displayed in the chat. If
transmission fails, the message is marked as failed and the text is restored
in the input field.

Messages received from the server are displayed with their timestamp. The
client prevents duplicate messages with the same text and display time.

## 10. Available Commands

### 10.1 Server commands

From `georuggine-server/`:

```bash
cargo run                 # start the server in development mode
cargo build --release     # build the optimized server
cargo test                # run server tests
```

### 10.2 Client commands

From `georuggine-client/`:

```bash
npm install               # install dependencies
npm run dev               # start the Vite frontend
npm test                  # run frontend tests
npm run test:coverage     # run tests with coverage
npm run build             # type-check and build the frontend
npm run tauri dev         # start the desktop client
npm run tauri build       # create the desktop installer
```

The graphical application does not provide a terminal command interface. Its
operations are performed through the login, chat and coordinate-upload
controls.

## 11. Coordinate File Format

The coordinate uploader accepts files with the `.json` extension only. The
root value must be a non-empty JSON array. Each element must contain numeric
coordinates.

The canonical format is:

```json
[
  { "x": 45.0703, "y": 7.6869 },
  { "x": 45.2, "y": 8.5 },
  { "x": 45.4641, "y": 9.1919 }
]
```

The aliases `lat` and `lon` are also accepted:

```json
[
  { "lat": 45.0703, "lon": 7.6869 },
  { "lat": 45.4641, "lon": 9.1919 }
]
```

The file is rejected when:

- its extension is not `.json`;
- it is empty;
- it is not valid JSON;
- the root array contains no points;
- the WebSocket is not connected.

The timestamp of each transmitted position is generated by the client in UTC.

## 12. Accounts

GEO-RUST does not ship with predefined users. Create an account from the
**Register** screen, then use the **Login** screen.

Registration requires:

- a non-empty username;
- a password of at least four characters.

The native client stores the current username and JWT inside the protected Rust
application state. The JWT is not exposed to React components.

To end a session, select **Logout**. This closes the WebSocket, stops background
tracking, clears the local session and returns to the login screen.
