use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};

use super::{
    structs::{Message, User},
    utils::MessageUserInfo,
};

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(options: PgConnectOptions) -> Result<Database, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        Ok(Database { pool })
    }

    pub async fn create_user(
        &self,
        id: &str,
        name: &str,
        password_hash: &str,
        public_key: &[u8],
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO Users (id, name, password_hash, public_key) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(name)
        .bind(password_hash)
        .bind(public_key)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_user_by_id(&self, id: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM Users u WHERE u.id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn create_message(
        &self,
        sender: &MessageUserInfo,
        receiver: &MessageUserInfo,
        content: &[u8],
    ) -> Result<(Message, Message), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let in_msg = sqlx::query_as::<_, Message>(
            "INSERT INTO InMessages (user_id, content, secret_key) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&receiver.id)
        .bind(content)
        .bind(&receiver.secret_key)
        .fetch_one(&mut *tx)
        .await?;

        let out_msg = sqlx::query_as::<_, Message>(
            "INSERT INTO OutMessages (user_id, content, secret_key) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&sender.id)
        .bind(content)
        .bind(&sender.secret_key)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok((in_msg, out_msg))
    }

    pub async fn get_in_messages(
        &self,
        user: &User,
        after: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let messages = sqlx::query_as::<_, Message>(
            "SELECT * FROM InMessages msg WHERE msg.user_id = $1 AND msg.id > $2",
        )
        .bind(&user.id)
        .bind(&after)
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }

    pub async fn get_out_messages(
        &self,
        user: &User,
        after: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let messages = sqlx::query_as::<_, Message>(
            "SELECT * FROM OutMessages msg WHERE msg.user_id = $1 AND msg.id > $2",
        )
        .bind(&user.id)
        .bind(&after)
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }
}
