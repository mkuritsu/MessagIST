use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetUser {
    pub id: String,
    pub name: String,
    pub public_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i64,
    pub contents: Vec<u8>,
    pub secret_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetMessage {
    pub inbound: Vec<Message>,
    pub outbound: Vec<Message>,
}
