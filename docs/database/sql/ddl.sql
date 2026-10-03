BEGIN
TRANSACTION;

DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS messages;

DROP TABLE IF EXISTS positions;

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE messages (
    message_id INTEGER PRIMARY KEY,
    sender_id INTEGER NOT NULL,
    receiver_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    contents TEXT,
    FOREIGN KEY(sender_id) REFERENCES users(id),
    FOREIGN KEY(receiver_id) REFERENCES users(id)
);

CREATE TABLE positions (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    x_coord INTEGER NOT NULL,
    y_coord INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

COMMIT;
