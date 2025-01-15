use std::sync::Arc;

use cryptolib::RsaPrivateKey;
use futures_util::TryStreamExt;
use protocol::stoc::Message;
use reqwest_websocket::{Message as WSMessage, WebSocket};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    db::{structs::StoredMessage, Database},
    message_data::MessageData,
};

pub async fn notification_handler(
    mut websocket: WebSocket,
    sender: UnboundedSender<StoredMessage>,
    db: Arc<Database>,
    private_key: RsaPrivateKey,
) -> Result<(), reqwest_websocket::Error> {
    log::info!("WS handler started!");
    while let Some(notify) = websocket.try_next().await? {
        log::info!("Received notify: {:?}", notify);
        if let WSMessage::Text(text) = notify {
            let message = match serde_json::from_str::<Message>(&text) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to deserialized received message: {e}");
                    continue;
                }
            };
            log::info!("MESSAGE: {:?}", message);
            let secret_key = message.secret_key;
            let secret_key = match cryptolib::decrypt_key_with_priv_key(&secret_key, &private_key) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to decrypt secret key: {e}");
                    continue;
                }
            };
            let data = match MessageData::decrypt(&message.contents, &secret_key) {
                Ok(v) => v,
                Err(_) => {
                    log::warn!("Coudln't decrypt message, integrity failed!");
                    continue;
                }
            };
            match db
                .create_message(
                    &data.sender_istid,
                    &data.receiver_istid,
                    &data.timestamp,
                    &data.content,
                    &secret_key,
                    data.receive_counter,
                    data.sent_counter,
                    message.id,
                )
                .await
            {
                Ok(stored) => {
                    if let Err(e) = sender.send(stored) {
                        log::error!("Failed to send message notification to UI: {e}");
                    }
                }
                Err(e) => {
                    log::error!("Failed to store message in database: {e}")
                }
            }
        }
    }
    Ok(())
}
