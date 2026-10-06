# WebSockets

WebSocket is a network protocol that provides **full-duplex communication over a single, persistent TCP connection**.

## How do WebSockets work?

WebSockets is a TCP based protocol: its only relation with HTTP is that its handshake is interpreted via an **HTTP upgrade request**. The client starts the communication by sending an HTTP request containing the following information:

```
Host: server.name
Upgrade: websocket
Connection: Upgrade <-- Tells the server that wants to use another mean of communication
Sec-WebSocket-Key: very-long-key
Sec-WebSocket-Version: 1.3
```

If even one of these fields is missing, the server should respond with a `400 Bad Request` error. The server agrees to establish the connection by sending an HTTP package containing a `Sec-WebSocket-Accept` response header. Now the connection has been established and messages can now be sent in either direction with minimal overhead.

## How are WebSockets used in the project

### `ws_handler` - Upgrading the HTTP request to a WebSocket

The app exposes an API endpoint `{BASE_URL}/ws` that can be used by the user to establish a new WebSocket connection. This request is handled by the function `ws_handler`, which has the following signature:

```rust
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(clients): State<Clients>,
) -> impl IntoResponse { /* ... */ }
```

`State(clients)` is explicitly passed via a `.with_state(app_state)` call, so no surprise there. The other three parameters, in the context of Axum, are called **Extractors** and, as the name suggests, they _extract_ the relevant information _directly from the HTTP packet_.

- `ws: WebSocketUpgrade`: ensures that the packet contains a valid HTTP upgrade request by checking whether the field `Upgrade: websocket` is present. It can be used to upgrade the connection from HTTP to a WebSocket.
- `user_agent: Option<TypedHeader<headers::UserAgent>>`: reads the `User-Agent` to get more info about the user. The following is an example of a valid `User-Agent` header.

  ```
  [User-Agent: Mozilla/<version> (<system-information>) <platform> (<platform-details>) <extensions>]

  Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.3
  ```

- `ConnectInfo(addr): ConnectInfo<SocketAddr>`: extracts the client's IP address and port number. This will be used to identify a specific user in the HashMap of connected clients.

### `handle_socket` - Interacting with the client via the WebSocket

Once the connection has been upgraded to a WebSocket, the actual communication with the client is handled via the `handle_socket` function. The first thing that this function does is **splitting the socket**. As we've said before, WebSockets allow for full-duplex communication: the `.split()` method returns a `Sink` and a `Stream` that can be used, respectively, to send and receive messages.

At a high level, this function works in the following way:

- The socket is split into a `Sink` and a `Stream`. The jobs of sending and receiving messages are handled by two separate Tokio tasks.
- An `mpsc::unbounded_channel` is created. Here the idea is the following:
  1. Connected clients are stored as a `ConnectedClient` struct, which contains a `user_id` and the transmitter end of the channel that has just been created.
  2. The sender task owns the receiving end of the channel.
  3. In this way, when you want to send a message to a specific client, you just need to access the HashMap to get the desired `ConnectedClient` instance and then you can send the message via the `tx` end, which will be received and later sent to the client by the `send_task` (who owns the `rx` end).
- `recv_task` receives and processes messages coming from the client. The `process_message` function checks which kind of message has been received and performs some operation accordingly.
