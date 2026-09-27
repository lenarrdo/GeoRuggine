BEGIN
TRANSACTION;

DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS messages;

DROP TABLE IF EXISTS positions;

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    hash TEXT NOT NULL,
    salt TEXT NOT NULL
);

CREATE TABLE messages (
    message_id INTEGER PRIMARY KEY AUTOINCREMENT,
    sender_id INTEGER NOT NULL,
    receiver_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    contents TEXT,
    FOREIGN KEY(sender_id) REFERENCES users(id)
    FOREIGN KEY(receiver_id) REFERENCES users(id)
);

CREATE TABLE positions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    x_coord INTEGER NOT NULL,
    y_coord INTEGER NOT NULL,
    timestamp DATETIME NOT NULL
);

COMMIT;
