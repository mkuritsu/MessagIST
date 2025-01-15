use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

use super::structs::{Contact, StoredMessage};

#[derive(Clone, Debug)]
pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(url: &str, password: &str) -> Result<Database, sqlx::Error> {
        let opts = SqliteConnectOptions::from_str(url)?
            .pragma("key", password.to_string())
            .pragma("cipher_page_size", "4096")
            .pragma("kdf_iter", "256000")
            .pragma("cipher_hmac_algorithm", "HMAC_SHA512")
            .pragma("cipher_kdf_algorithm", "PBKDF2_HMAC_SHA512")
            .foreign_keys(true)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(opts).await?;
        let db = Database { pool };
        db.create_table_contact().await?;
        db.create_table_message().await?;
        Ok(db)
    }

    async fn create_table_contact(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS Contact (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            public_key BLOB NOT NULL
        );",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_table_message(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS Message (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender_istid TEXT NOT NULL,
            receiver_istid TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            content TEXT NOT NULL,
            receive_counter INTEGER NOT NULL,
            sent_counter INTEGER NOT NULL,
            secret_key BLOB NOT NULL,
            server_id INTEGER NOT NULL
        );",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_contact(
        &self,
        id: &str,
        name: &str,
        public_key: &[u8],
    ) -> Result<Contact, sqlx::Error> {
        let contact = sqlx::query_as::<_, Contact>(
            "INSERT INTO Contact (id, name, public_key) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(id)
        .bind(name)
        .bind(public_key)
        .fetch_one(&self.pool)
        .await?;
        Ok(contact)
    }

    pub async fn create_message(
        &self,
        sender_istid: &str,
        receiver_istid: &str,
        timestamp: &str,
        content: &str,
        secret_key: &[u8],
        receive_counter: i64,
        sent_counter: i64,
        server_id: i64,
    ) -> Result<StoredMessage, sqlx::Error> {
        let message = sqlx::query_as::<_, StoredMessage>(
            "INSERT INTO Message (sender_istid, receiver_istid, timestamp, content, secret_key, receive_counter, sent_counter, server_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(sender_istid)
        .bind(receiver_istid)
        .bind(timestamp)
        .bind(content)
        .bind(secret_key)
        .bind(receive_counter)
        .bind(sent_counter)
        .bind(server_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(message)
    }

    pub async fn get_all_contacts(&self) -> Result<Vec<Contact>, sqlx::Error> {
        let contacts = sqlx::query_as::<_, Contact>("SELECT * FROM Contact c")
            .fetch_all(&self.pool)
            .await?;
        Ok(contacts)
    }

    pub async fn get_contact_by_id(&self, id: &str) -> Result<Contact, sqlx::Error> {
        let contact = sqlx::query_as::<_, Contact>("SELECT * FROM Contact c WHERE c.id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(contact)
    }

    pub async fn get_all_messages_by_contact(
        &self,
        contact: &Contact,
    ) -> Result<Vec<StoredMessage>, sqlx::Error> {
        let messages = sqlx::query_as::<_, StoredMessage>(
            "SELECT * FROM Message m WHERE m.sender_istid = $1 OR m.receiver_istid = $1",
        )
        .bind(&contact.id)
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }

    pub async fn get_last_sent_message_id(&self, my_id: &str) -> Result<i32, sqlx::Error> {
        let max =
            sqlx::query_scalar("SELECT MAX(server_id) from Message m where m.sender_istid = $1")
                .bind(my_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(max)
    }

    pub async fn get_last_received_message_id(&self, my_id: &str) -> Result<i32, sqlx::Error> {
        let max =
            sqlx::query_scalar("SELECT MAX(server_id) from Message m where m.receiver_istid = $1")
                .bind(my_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(max)
    }
}
