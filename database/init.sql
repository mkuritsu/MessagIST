CREATE TABLE IF NOT EXISTS Users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    public_key bytea NOT NULL
);

CREATE TABLE IF NOT EXISTS InMessages (
    id BIGSERIAL PRIMARY KEY,
    user_id TEXT NOT NULL,
    content bytea NOT NULL,
    secret_key bytea NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users (id)
);

CREATE TABLE IF NOT EXISTS OutMessages (
    id BIGSERIAL PRIMARY KEY,
    user_id TEXT NOT NULL,
    content bytea NOT NULL,
    secret_key bytea NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users (id)
);
