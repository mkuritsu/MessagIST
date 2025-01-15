use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password_hash: String,
    pub public_key: Vec<u8>,
}

#[derive(FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Message {
    pub id: i64,
    pub user_id: String,
    pub content: Vec<u8>,
    pub secret_key: Vec<u8>,
}
