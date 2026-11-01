BEGIN
TRANSACTION;

DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS journeys;

DROP TABLE IF EXISTS messages;

DROP TABLE IF EXISTS positions;

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS journeys (
    id INTEGER PRIMARY KEY,
    started_by INTEGER NOT NULL,
    started_at DATETIME NOT NULL,
    ended_at DATETIME,
    FOREIGN KEY(started_by) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS text_messages (
    id INTEGER PRIMARY KEY,
    sender_id INTEGER NOT NULL,
    receiver_id INTEGER NOT NULL,
    journey_id INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    contents TEXT,
    FOREIGN KEY(sender_id) REFERENCES users(id),
    FOREIGN KEY(receiver_id) REFERENCES users(id),
    FOREIGN KEY(journey_id) REFERENCES journeys(id)
);

CREATE TABLE IF NOT EXISTS positions (
    id INTEGER PRIMARY KEY,
    journey_id INTEGER NOT NULL,
    lat INTEGER NOT NULL,
    lon INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    FOREIGN KEY(journey_id) REFERENCES journeys(id)
);

COMMIT;
