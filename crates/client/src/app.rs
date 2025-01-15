use std::{collections::HashMap, path::Path, sync::Arc, time::Duration};

use crossterm::event::Event;
use cryptolib::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
    RsaPrivateKey, RsaPublicKey,
};
use rand::rngs::OsRng;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::{
    client_handler::MessageISTClient,
    db::{
        structs::{Contact, StoredMessage},
        Database,
    },
    logger::LoggerRecord,
    message_data::MessageData,
    ui::theming::{self, AppTheme},
};

#[derive(Clone, Debug)]
pub enum AppEvent {
    Input(Event),
    Log(LoggerRecord),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pages {
    Connect,
    Entry,
    Login,
    Register,
    Main,
    AddContact,
}

#[derive(Debug)]
pub enum LoginError {
    RequestError(reqwest::Error),
    KeyGen,
}

#[derive(Debug)]
pub struct SessionUser {
    pub id: String,
    pub name: String,
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

impl SessionUser {
    pub fn new(
        id: String,
        name: String,
        public_key: RsaPublicKey,
        private_key: RsaPrivateKey,
    ) -> Self {
        Self {
            id,
            name,
            public_key,
            private_key,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub current_page: Pages,
    pub should_quit: bool,
    pub show_terminal: bool,
    pub contacts: Vec<Contact>,
    pub messages: HashMap<String, Vec<StoredMessage>>,
    pub frame_duration: Duration,
    pub net_client: MessageISTClient,
    pub just_registered: bool,
    pub db: Option<Arc<Database>>,
    pub theme: AppTheme,
    pub notification_receiver: Option<UnboundedReceiver<StoredMessage>>,
    pub current_user: Option<SessionUser>,
}

impl App {
    pub fn new() -> anyhow::Result<App> {
        Ok(App {
            current_page: Pages::Connect,
            should_quit: false,
            show_terminal: false,
            contacts: Vec::new(),
            messages: HashMap::new(),
            frame_duration: Duration::ZERO,
            net_client: MessageISTClient::new(),
            just_registered: false,
            db: None,
            theme: theming::CATPUCCIN_MOCHA,
            notification_receiver: None,
            current_user: None,
        })
    }

    pub async fn gen_key_pair(
        &mut self,
        id: &str,
    ) -> anyhow::Result<(RsaPrivateKey, RsaPublicKey)> {
        let prk_path = format!("{}.priv", id);
        let puk_path = format!("{}.pub", id);
        let prk_path = Path::new(&prk_path);
        let puk_path = Path::new(&puk_path);
        let prk = if prk_path.exists() {
            let prk_pem = tokio::fs::read_to_string(prk_path).await?;
            RsaPrivateKey::from_pkcs8_pem(&prk_pem)?
        } else {
            let prk = RsaPrivateKey::new(&mut OsRng, 2048)?;
            let prk_pem = prk.to_pkcs8_pem(LineEnding::default())?;
            tokio::fs::write(&prk_path, prk_pem).await?;
            prk
        };
        let puk = if puk_path.exists() {
            let puk_pem = tokio::fs::read_to_string(puk_path).await?;
            RsaPublicKey::from_public_key_pem(&puk_pem)?
        } else {
            let puk = RsaPublicKey::from(&prk);
            let puk_pem = puk.to_public_key_pem(LineEnding::default())?;
            tokio::fs::write(&puk_path, puk_pem).await?;
            puk
        };
        Ok((prk, puk))
    }

    pub async fn login<'a>(&'a mut self, id: &str, password: &str) -> Result<(), LoginError> {
        let user = match self.net_client.login(id, password).await {
            Ok(v) => v,
            Err(e) => return Err(LoginError::RequestError(e)),
        };
        let Ok((prk, puk)) = self.gen_key_pair(id).await else {
            return Err(LoginError::KeyGen);
        };
        let user = SessionUser::new(user.id, user.name, puk, prk);
        self.current_user = Some(user);
        Ok(())
    }

    pub async fn connect_database(&mut self, user_id: &str, password: &str) -> anyhow::Result<()> {
        let db = Database::new(&format!("sqlite://{}.db", user_id), password).await?;
        self.db = Some(Arc::new(db.clone()));
        self.sync_database(&db).await?;
        Ok(())
    }

    pub async fn get_last_message_id(&self) -> i64 {
        let mut last_id = -1;
        for (_, messages) in &self.messages {
            if let Some(msg) = messages.last() {
                if msg.id > last_id {
                    last_id = msg.id;
                }
            }
        }
        last_id
    }

    pub async fn sync_database(&mut self, db: &Database) -> anyhow::Result<()> {
        let contacts = db.get_all_contacts().await?;
        for c in contacts {
            let messages = db.get_all_messages_by_contact(&c).await?;
            self.messages.insert(c.id.clone(), messages);
            self.contacts.push(c);
        }
        let last_sent_id = db
            .get_last_sent_message_id(&self.current_user.as_ref().unwrap().id)
            .await?;
        let last_recv_id = db
            .get_last_received_message_id(&self.current_user.as_ref().unwrap().id)
            .await?;
        let messages = self
            .net_client
            .get_messages(last_recv_id, Some(last_sent_id))
            .await?;
        log::debug!(
            "Received sync messages: inbound: {:?} outbout: {:?}",
            messages.inbound,
            messages.outbound
        );
        for inbound in messages.inbound {
            let id = inbound.id;
            let secret_key = inbound.secret_key;
            let secret_key = cryptolib::decrypt_key_with_priv_key(
                &secret_key,
                &self.current_user.as_ref().unwrap().private_key,
            )?;
            let message_data = match MessageData::decrypt(&inbound.contents, &secret_key) {
                Ok(v) => v,
                Err(_) => {
                    log::warn!("Received tampered inbound message in sync!");
                    continue;
                }
            };
            let message = db
                .create_message(
                    &message_data.sender_istid,
                    &message_data.receiver_istid,
                    &message_data.timestamp,
                    &message_data.content,
                    &secret_key,
                    message_data.receive_counter,
                    message_data.sent_counter,
                    id,
                )
                .await?;
            self.add_message(message_data.sender_istid, message).await?;
        }
        for outbound in messages.outbound {
            let id = outbound.id;
            let secret_key = outbound.secret_key;
            let secret_key = cryptolib::decrypt_key_with_priv_key(
                &secret_key,
                &self.current_user.as_ref().unwrap().private_key,
            )?;
            let message_data = match MessageData::decrypt(&outbound.contents, &secret_key) {
                Ok(v) => v,
                Err(_) => {
                    log::warn!("Received tampered outbount message in sync!");
                    continue;
                }
            };
            let message = db
                .create_message(
                    &message_data.sender_istid,
                    &message_data.receiver_istid,
                    &message_data.timestamp,
                    &message_data.content,
                    &secret_key,
                    message_data.receive_counter,
                    message_data.sent_counter,
                    id,
                )
                .await?;
            self.add_message(message_data.receiver_istid, message)
                .await?;
        }
        Ok(())
    }

    pub async fn add_contact(
        &mut self,
        id: &str,
        name: &str,
        public_key: &[u8],
    ) -> anyhow::Result<()> {
        match &self.db {
            Some(db) => {
                let contact = db.create_contact(id, name, public_key).await?;
                self.messages.insert(contact.id.clone(), vec![]);
                self.contacts.push(contact);
                Ok(())
            }
            None => panic!("Add contact without DB"),
        }
    }

    pub async fn add_message(
        &mut self,
        contact_id: String,
        message: StoredMessage,
    ) -> anyhow::Result<()> {
        self.messages
            .entry(contact_id.clone())
            .or_insert(Vec::new())
            .push(message);
        let has_contact = self.contacts.iter().any(|c| c.id == contact_id);
        if !has_contact {
            match self.net_client.get_user(&contact_id).await {
                Ok(contact) => {
                    let contact = self
                        .db
                        .as_ref()
                        .unwrap()
                        .create_contact(&contact.id, &contact.name, &contact.public_key)
                        .await?;
                    self.contacts.push(contact);
                }
                Err(e) => log::error!("Failed to get contact: {e}"),
            }
        }
        Ok(())
    }

    pub async fn send_message(&mut self, contact: &Contact, content: &str) -> anyhow::Result<()> {
        let curr_user = self
            .current_user
            .as_ref()
            .expect("No current user when sending message");
        let (sent_counter, receive_counter) = self.last_counters(&contact.id);
        let message_data = MessageData::new(
            &curr_user.id,
            &contact.id,
            content,
            receive_counter,
            sent_counter + 1,
        );
        let secret_key = cryptolib::generate_secret_key();
        let encrypt = MessageData::encrypt(&message_data, &secret_key)?;
        let my_secret_key =
            cryptolib::encrypt_key_with_pub_key(&secret_key, &curr_user.public_key)?;
        let contact_puk = cryptolib::utils::public_key_from_bytes(&contact.public_key)?;
        let recipient_secret_key = cryptolib::encrypt_key_with_pub_key(&secret_key, &contact_puk)?;

        log::info!("Sending message");
        let message = self
            .net_client
            .send_message(&contact.id, &encrypt, &my_secret_key, &recipient_secret_key)
            .await?;
        log::info!("Message sent {:?}", message);
        let message_data = MessageData::decrypt(&message.contents, &secret_key)?;
        log::info!("Message decrypted: {:?}", message_data);
        let stored_message = self
            .db
            .as_ref()
            .expect("No DB")
            .create_message(
                &message_data.sender_istid,
                &message_data.receiver_istid,
                &message_data.timestamp,
                &message_data.content,
                &secret_key,
                message_data.receive_counter,
                message_data.sent_counter,
                message.id,
            )
            .await?;
        log::info!("Message stored: {:?}", stored_message);
        self.messages
            .entry(contact.id.clone())
            .or_insert(Vec::new())
            .push(stored_message);
        Ok(())
    }

    fn last_counters(&self, contact_id: &str) -> (i64, i64) {
        let Some(messages) = self.messages.get(contact_id) else {
            return (0, 0);
        };
        let last_receive_counter = messages
            .iter()
            .map(|m| m.receive_counter)
            .max()
            .unwrap_or_default();
        let last_sent_counter = messages
            .iter()
            .map(|m| m.sent_counter)
            .max()
            .unwrap_or_default();
        (last_sent_counter, last_receive_counter)
    }
}
