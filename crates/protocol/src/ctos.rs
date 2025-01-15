use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub id: String,
    pub name: String,
    pub password: String,
    pub public_key: Vec<u8>,
}

impl Register {
    pub fn new(id: &str, name: &str, password: &str, public_key: &[u8]) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            password: password.to_string(),
            public_key: public_key.to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl Login {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    pub recipient: String,
    pub contents: Vec<u8>,
    pub my_secret_key: Vec<u8>,
    pub recipient_secret_key: Vec<u8>,
}

impl SendMessage {
    pub fn new(
        recipient: &str,
        contents: &[u8],
        secret_key: &[u8],
        recipient_secret_key: &[u8],
    ) -> Self {
        Self {
            recipient: recipient.to_string(),
            contents: contents.to_vec(),
            my_secret_key: secret_key.to_vec(),
            recipient_secret_key: recipient_secret_key.to_vec(),
        }
    }
}
