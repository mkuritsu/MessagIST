pub struct MessageUserInfo {
    pub id: String,
    pub secret_key: Vec<u8>,
}

impl MessageUserInfo {
    pub fn new(id: &str, secret_key: &[u8]) -> Self {
        Self {
            id: String::from(id),
            secret_key: secret_key.to_vec(),
        }
    }
}
