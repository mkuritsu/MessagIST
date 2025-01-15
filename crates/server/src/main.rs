use db::Database;
use notify::NotifyService;
use rocket::{config::LogLevel, figment::Figment, Config};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use user_cache::UserCacheService;

#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod notify;
mod session;
mod user_cache;

fn create_config() -> Figment {
    Config::figment().merge(("log_level", LogLevel::Normal))
}

const CA_CERT_BYTES: &[u8] = include_bytes!("../../../certs/ca.crt");
const CLIENT_CERT_BYTES: &[u8] = include_bytes!("../../../certs/server.crt");
const CLIENT_KEY_BYTES: &[u8] = include_bytes!("../../../certs/server.key");

#[launch]
async fn rocket() -> _ {
    let options = PgConnectOptions::new()
        .ssl_mode(PgSslMode::VerifyCa)
        .database("messagist")
        .username("messagist_server")
        .ssl_root_cert_from_pem(CA_CERT_BYTES.to_vec())
        .ssl_client_key_from_pem(CLIENT_KEY_BYTES)
        .ssl_client_cert_from_pem(CLIENT_CERT_BYTES);
    let db = Database::new(options)
        .await
        .expect("Failed to connect to database!");

    let user_cache_service = UserCacheService::new();
    let notify_service = NotifyService::new();

    let config = create_config();

    rocket::custom(config)
        .manage(db)
        .manage(user_cache_service)
        .manage(notify_service)
        .mount(
            "/api",
            routes![
                handlers::hello,
                handlers::register,
                handlers::login,
                handlers::get_user,
                handlers::get_messages,
                handlers::send_message,
                handlers::logout,
                handlers::notifications,
            ],
        )
}
