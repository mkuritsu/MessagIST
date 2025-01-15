use base64::{prelude::BASE64_STANDARD, Engine};
use client::{client_handler::MessageISTClient, message_data::MessageData};
use cryptolib::RsaPublicKey;
use protocol::stoc::ResponseGetUser;
use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    time::Duration,
};
use tokio::fs;

mod utils;

const IST_ID: &'static str = "ist0000000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let address = args.get(1).expect("No server address provided as argument");
    let name = "Bob Hackerman";
    let password = "verysecurepassword";
    let priv_key_path = format!("{}.priv", IST_ID);
    let priv_key_path = Path::new(&priv_key_path);
    let registered = fs::try_exists(&priv_key_path).await?;
    let (prk, puk) = utils::gen_key_pair(IST_ID).await?;
    let prk_bytes = cryptolib::utils::private_key_to_bytes(prk)?;
    let mut http_client = MessageISTClient::new();
    http_client.connect(address).await?;
    if !registered {
        http_client
            .register(IST_ID, &name, &password, &prk_bytes)
            .await?;
    }
    http_client.login(IST_ID, &password).await?;
    println!("Logged in as {} - {}", IST_ID, name);
    print!("IST ID of target user: ");
    stdout().flush()?;
    let mut target_ist_id = String::new();
    stdin().read_line(&mut target_ist_id)?;
    let user = http_client.get_user(&target_ist_id).await?;
    println!("Target user: {} - {}", user.id, user.name);
    send_correct_message(&http_client, &user, &puk).await;
    tokio::time::sleep(Duration::from_secs(5)).await;
    send_messages_out_of_order(&http_client, &user, &puk).await;
    tokio::time::sleep(Duration::from_secs(5)).await;
    send_missing_message(&http_client, &user, &puk).await;
    tokio::time::sleep(Duration::from_secs(5)).await;
    send_tampered_message(&http_client, &user, &puk).await;
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("ALL TESTS COMPLETED");
    Ok(())
}

async fn send_message(
    message: MessageData,
    http_client: &MessageISTClient,
    user: &ResponseGetUser,
    puk: &RsaPublicKey,
    tampered: bool,
    verbose: bool,
) {
    let secret_key = cryptolib::generate_secret_key();
    let mut encrypted = message.encrypt(&secret_key).expect("Failed to encrypt");
    if tampered {
        encrypted.push(1); // add one aditional byte
    }
    if verbose {
        println!("Encrypted payload: {:?}", encrypted);
    }
    let my_secret_key = cryptolib::encrypt_key_with_pub_key(&secret_key, &puk)
        .expect("Failed to encrypt secret key");
    let other_puk = cryptolib::utils::public_key_from_bytes(&user.public_key)
        .expect("Failed to get key from bytes");
    let other_secret_key = cryptolib::encrypt_key_with_pub_key(&secret_key, &other_puk)
        .expect("Failed to encrypt secret key");
    if verbose {
        println!("Secret key: {:?}", BASE64_STANDARD.encode(&secret_key));
        println!(
            "My secret key: {:?}",
            BASE64_STANDARD.encode(&my_secret_key)
        );
        println!(
            "Recipient secret key: {:?}",
            BASE64_STANDARD.encode(&other_secret_key)
        );
    }
    http_client
        .send_message(&user.id, &encrypted, &my_secret_key, &other_secret_key)
        .await
        .expect("Failed to send message");
    println!("Message sent!");
}

async fn send_correct_message(
    http_client: &MessageISTClient,
    user: &ResponseGetUser,
    puk: &RsaPublicKey,
) {
    println!("------------------------[ TEST CORRECT MESSAGE ]------------------------");
    println!("Sending 'Hello world' to client");
    let message = MessageData::new(IST_ID, &user.id, "Hello world", 0, 1);
    send_message(message, http_client, user, puk, false, true).await;
    println!("------------------------------------------------------------------------")
}

async fn send_messages_out_of_order(
    http_client: &MessageISTClient,
    user: &ResponseGetUser,
    puk: &RsaPublicKey,
) {
    println!("------------------------[ TEST OUT OF ORDER MESSAGE ]------------------------");
    let third = MessageData::new(IST_ID, &user.id, "Third message", 0, 3);
    let second = MessageData::new(IST_ID, &user.id, "Second message", 0, 2);
    send_message(third, http_client, user, puk, false, false).await;
    tokio::time::sleep(Duration::from_millis(1000)).await;
    send_message(second, http_client, user, puk, false, false).await;
    println!("------------------------------------------------------------------------------")
}

async fn send_missing_message(
    http_client: &MessageISTClient,
    user: &ResponseGetUser,
    puk: &RsaPublicKey,
) {
    println!("------------------------[ TEST MISSING MESSAGE ]------------------------");
    let fifth = MessageData::new(IST_ID, &user.id, "Fifth message", 0, 5);
    send_message(fifth, http_client, user, puk, false, false).await;
    println!("------------------------------------------------------------------------")
}

async fn send_tampered_message(
    http_client: &MessageISTClient,
    user: &ResponseGetUser,
    puk: &RsaPublicKey,
) {
    println!("------------------------[ TEST TAMPERED MESSAGE ]------------------------");
    let tampered = MessageData::new(IST_ID, &user.id, "Sixth message (tampered)", 0, 6);
    send_message(tampered, http_client, user, puk, true, false).await;
    println!("-------------------------------------------------------------------------")
}
