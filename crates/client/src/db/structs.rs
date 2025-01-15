use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub public_key: Vec<u8>,
}

#[derive(FromRow, Debug, Clone)]
pub struct StoredMessage {
    pub id: i64,
    pub sender_istid: String,
    pub receiver_istid: String,
    pub timestamp: String,
    pub content: String,
    pub secret_key: Vec<u8>,
    pub receive_counter: i64,
    pub sent_counter: i64,
    pub server_id: i64,
}
