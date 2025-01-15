use protocol::{
    ctos::*,
    stoc::{Message, ResponseGetMessage, ResponseGetUser},
};
use reqwest::{Certificate, Client};
use reqwest_websocket::{RequestBuilderExt, WebSocket};

#[derive(Debug)]
pub struct MessageISTClient {
    base_url: String,
    client: Client,
}

impl MessageISTClient {
    pub fn new() -> Self {
        Self {
            base_url: String::default(),
            client: Client::default(),
        }
    }

    pub async fn connect(&mut self, address: &str) -> anyhow::Result<()> {
        const CERT_BYTES: &[u8] = include_bytes!("../../../certs/ca.crt");
        let cert = Certificate::from_pem(CERT_BYTES)?;
        log::debug!(
            "TRUSTED AUTHORITY CERTIFICATE: {}",
            String::from_utf8_lossy(CERT_BYTES)
        );
        let client = Client::builder()
            .add_root_certificate(cert)
            .tls_sni(false)
            .danger_accept_invalid_hostnames(true)
            .cookie_store(true)
            .build()?;
        log::debug!("Http client created!");
        let address = if address.to_lowercase().starts_with("https://") {
            format!("{}/api", address)
        } else {
            format!("https://{}/api", address.to_lowercase())
        };
        log::debug!("Trying to conenct to server at address {}", address);
        client
            .head(&format!("{}/hello", address))
            .send()
            .await?
            .error_for_status()?;
        log::info!("Connection established!");
        self.client = client;
        self.base_url = address;
        Ok(())
    }

    pub async fn check_connection(&self) -> Result<(), reqwest::Error> {
        let url = format!("{}/hello", self.base_url);
        self.client.head(url).send().await?;
        Ok(())
    }

    pub async fn register(
        &self,
        id: &str,
        name: &str,
        password: &str,
        public_key: &[u8],
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/users", self.base_url);
        let register = Register::new(id, name, password, public_key);
        log::info!(
            "Sending REGISTER request to {} with data: {:?}",
            self.base_url,
            register
        );
        self.client
            .post(url)
            .json(&register)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<ResponseGetUser, reqwest::Error> {
        let url = format!("{}/login", self.base_url);
        let login = Login::new(username, password);
        let user = self
            .client
            .post(url)
            .json(&login)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(user)
    }

    pub async fn logout(&self) -> Result<(), reqwest::Error> {
        let url = format!("{}/logout", &self.base_url);
        self.client.post(url).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn get_user(&self, username: &str) -> Result<ResponseGetUser, reqwest::Error> {
        let url = format!("{}/users/{username}", &self.base_url);
        let user = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(user)
    }

    pub async fn send_message(
        &self,
        recipient: &str,
        msg: &[u8],
        my_secret_key: &Vec<u8>,
        recipient_secret_key: &Vec<u8>,
    ) -> Result<Message, reqwest::Error> {
        let url = format!("{}/messages", &self.base_url);
        let send_msg = SendMessage::new(recipient, msg, my_secret_key, recipient_secret_key);
        let response = self
            .client
            .post(url)
            .json(&send_msg)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn get_messages(
        &self,
        after: i32,
        out_after: Option<i32>,
    ) -> Result<ResponseGetMessage, reqwest::Error> {
        let mut url = format!("{}/messages?after={}", self.base_url, after);
        if let Some(out) = out_after {
            url.push_str(&format!("&out_after={}", out));
        }
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn connect_notifications_ws(&self) -> Result<WebSocket, reqwest_websocket::Error> {
        let url = format!("{}/notifications", self.base_url);
        let response = self.client.post(url).upgrade().send().await?;
        let websocket = response.into_websocket().await?;
        Ok(websocket)
    }
}
