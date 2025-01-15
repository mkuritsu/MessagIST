use std::time::Duration;

use protocol::{
    ctos::{Login, Register, SendMessage},
    stoc::{self, Message, ResponseGetMessage, ResponseGetUser},
};
use rocket::{
    futures::SinkExt,
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    tokio, Shutdown, State,
};
use rocket_ws::{Channel, WebSocket};

use crate::{
    db::{Database, MessageUserInfo},
    notify::NotifyService,
    session::ClientSession,
};

type RequestResult<S> = Result<S, Status>;

#[head("/hello")]
pub async fn hello() -> RequestResult<()> {
    RequestResult::Ok(())
}

#[post("/users", data = "<body>")]
pub async fn register(body: Json<Register>, db: &State<Database>) -> RequestResult<()> {
    let password_hash = match cryptolib::hash_password(&body.password) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Password hashing failed {}", e);
            return Err(Status::InternalServerError);
        }
    };
    if let Ok(_) = db.get_user_by_id(&body.id).await {
        log::info!("Denied registration: user already exists!");
        return Err(Status::Forbidden);
    };
    match db
        .create_user(&body.id, &body.name, &password_hash, &body.public_key)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Failed to create user: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/login", data = "<body>")]
pub async fn login(
    body: Json<Login>,
    db: &State<Database>,
    cookies: &CookieJar<'_>,
) -> RequestResult<Json<ResponseGetUser>> {
    let Ok(user) = db.get_user_by_id(&body.username).await else {
        return RequestResult::Err(Status::NotFound);
    };
    if !cryptolib::verify_hashed_password(&body.password, &user.password_hash) {
        return RequestResult::Err(Status::Unauthorized);
    }
    let cookie = Cookie::new("user", user.id.to_string());
    cookies.add_private(cookie);
    let response = ResponseGetUser {
        id: user.id,
        name: user.name,
        public_key: user.public_key,
    };
    RequestResult::Ok(Json(response))
}

#[post("/logout")]
pub async fn logout(_session: ClientSession, cookies: &CookieJar<'_>) -> RequestResult<()> {
    let Some(cookie) = cookies.get_private("user") else {
        return RequestResult::Err(Status::NotFound);
    };
    cookies.remove_private(cookie);
    Ok(())
}

#[get("/users/<username>")]
pub async fn get_user(
    username: &str,
    _session: ClientSession,
    db: &State<Database>,
) -> RequestResult<Json<ResponseGetUser>> {
    let Ok(user) = db.get_user_by_id(&username).await else {
        return Err(Status::NotFound);
    };
    Ok(Json(ResponseGetUser {
        id: user.id,
        name: user.name,
        public_key: user.public_key,
    }))
}

#[post("/messages", data = "<body>")]
pub async fn send_message(
    body: Json<SendMessage>,
    session: ClientSession,
    db: &State<Database>,
    notify_service: &State<NotifyService>,
) -> RequestResult<Json<Message>> {
    let Ok(recipient) = db.get_user_by_id(&body.recipient).await else {
        return RequestResult::Err(Status::NotFound);
    };
    let sender_info = MessageUserInfo::new(&session.user.id, &body.my_secret_key);
    let receiver_info = MessageUserInfo::new(&recipient.id, &body.recipient_secret_key);
    match db
        .create_message(&sender_info, &receiver_info, &body.contents)
        .await
    {
        Ok((in_msg, out_msg)) => {
            if let Some(queue) = notify_service.store.get_client_queue(&recipient.id).await {
                log::info!("CLient WS connected sendiing notification");
                let notification = stoc::Message {
                    id: in_msg.id,
                    contents: in_msg.content,
                    secret_key: in_msg.secret_key,
                };
                queue.push(notification);
            };
            let response = Message {
                id: out_msg.id,
                contents: out_msg.content,
                secret_key: out_msg.secret_key,
            };
            RequestResult::Ok(Json(response))
        }
        Err(e) => {
            log::error!("{}", e);
            return RequestResult::Err(Status::InternalServerError);
        }
    }
}

#[get("/messages?<after>&<out_after>")]
pub async fn get_messages(
    after: i64,
    out_after: Option<i64>,
    session: ClientSession,
    db: &State<Database>,
) -> RequestResult<Json<ResponseGetMessage>> {
    let mut inbound = vec![];
    let mut outbound = vec![];
    let in_msgs = match db.get_in_messages(&session.user, after).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("{}", e);
            return RequestResult::Err(Status::InternalServerError);
        }
    };
    for msg in in_msgs {
        inbound.push(Message {
            id: msg.id,
            contents: msg.content,
            secret_key: msg.secret_key,
        });
    }
    if let Some(out_after) = out_after {
        let Ok(out_msgs) = db.get_out_messages(&session.user, out_after).await else {
            return RequestResult::Err(Status::InternalServerError);
        };
        for msg in out_msgs {
            outbound.push(Message {
                id: msg.id,
                contents: msg.content,
                secret_key: msg.secret_key,
            });
        }
    }
    Ok(Json(ResponseGetMessage { inbound, outbound }))
}

#[post("/notifications")]
pub async fn notifications(
    ws: WebSocket,
    session: ClientSession,
    notify_service: &State<NotifyService>,
    shutdown: Shutdown,
) -> Channel<'static> {
    let store = notify_service.store.clone();
    let queue = store.create_client_queue(&session.user.id).await;
    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                if let Some(message) = queue.try_pop() {
                    let Ok(json) = serde_json::to_string(&message) else {
                        break;
                    };
                    if let Err(_) = stream.send(rocket_ws::Message::text(json)).await {
                        break;
                    }
                }
                // used for graceful shutdown
                if let Ok(_) =
                    tokio::time::timeout(Duration::from_millis(500), shutdown.clone()).await
                {
                    log::info!("Shutting down websocket!");
                    break;
                }
            }
            store.remove_client_queue(&session.user.id).await;
            Ok(())
        })
    })
}
