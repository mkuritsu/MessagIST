use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageData {
    pub sender_istid: String,
    pub receiver_istid: String,
    pub timestamp: String,
    pub content: String,
    pub receive_counter: i64,
    pub sent_counter: i64,
}

impl MessageData {
    pub fn new(
        sender: &str,
        receiver: &str,
        content: &str,
        receive_counter: i64,
        sent_counter: i64,
    ) -> Self {
        Self {
            sender_istid: String::from(sender),
            receiver_istid: String::from(receiver),
            timestamp: chrono::Utc::now().to_rfc3339(),
            content: String::from(content),
            receive_counter,
            sent_counter,
        }
    }

    pub fn encrypt(&self, secret_key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let data = serde_json::to_string(self)?;
        let (mut ciphertext, mut nonce) = match cryptolib::protect(data.as_bytes(), secret_key) {
            Ok(v) => v,
            Err(_) => return Err(anyhow::Error::msg("Failed to protect message")),
        };
        ciphertext.append(&mut nonce);
        Ok(ciphertext)
    }

    pub fn decrypt(data: &[u8], secret_key: &[u8]) -> anyhow::Result<MessageData> {
        let (ciphertext, nonce) = cryptolib::utils::separate_cipher_nonce(&data);
        let json = match cryptolib::unprotect(ciphertext, secret_key, &nonce) {
            Ok(v) => v,
            Err(_) => return Err(anyhow::Error::msg("Failed to decrypt message")),
        };
        let json = String::from_utf8(json)?;
        let message = serde_json::from_str::<MessageData>(&json)?;
        Ok(message)
    }
}
