use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    time::OffsetDateTime,
    Request,
};

use crate::{
    db::{structs::User, Database},
    user_cache::UserCacheService,
};

#[derive(Debug)]
pub struct ClientSession {
    pub user: User,
}

impl ClientSession {
    fn new(user: User) -> Self {
        Self { user }
    }
}

#[derive(Debug)]
pub enum SessionError {
    NotLoggedIn,
    InvalidToken,
    Expired,
    ServerError,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientSession {
    type Error = SessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(cookie) = req.cookies().get_private("user") else {
            return Outcome::Error((Status::Unauthorized, SessionError::NotLoggedIn));
        };
        if let Some(expiration) = cookie.expires_datetime() {
            if OffsetDateTime::now_utc().unix_timestamp() > expiration.unix_timestamp() {
                req.cookies().remove(cookie);
                return Outcome::Error((Status::Unauthorized, SessionError::Expired));
            }
        }
        let user_id = cookie.value();
        let user_cache_service: &UserCacheService = match req.rocket().state() {
            Some(v) => v,
            None => {
                log::error!("No UserCache found!");
                return Outcome::Error((Status::InternalServerError, SessionError::ServerError));
            }
        };
        let mut user_cache = user_cache_service.cache.lock().await;
        if let Some(user) = user_cache.get(user_id) {
            return Outcome::Success(ClientSession::new(user.clone()));
        }
        let db: &Database = match req.rocket().state() {
            Some(v) => v,
            None => {
                log::error!("No Database found!");
                return Outcome::Error((Status::InternalServerError, SessionError::ServerError));
            }
        };
        let Ok(user) = db.get_user_by_id(user_id).await else {
            return Outcome::Error((Status::Unauthorized, SessionError::InvalidToken));
        };
        user_cache.store(user.clone());
        Outcome::Success(ClientSession::new(user))
    }
}
