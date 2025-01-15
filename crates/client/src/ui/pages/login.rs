use crossterm::event::{Event, KeyCode};
use handler::LoginHandler;
use ratatui::layout::{self, Constraint, Direction, Rect};
use reqwest::StatusCode;
use state::{FocusedElement, LoginResult, LoginState};
use tokio::sync::mpsc;

use crate::{
    app::{App, AppEvent, LoginError, Pages},
    notifications,
    ui::event_handler::{AsyncStatefulEventHandler, StatefulEventHandler},
};

mod handler;
mod state;
mod widget;

#[derive(Default)]
pub struct Layout {
    header: Rect,
    notification: Rect,
    username: Rect,
    password: Rect,
    submit: Rect,
    footer: Rect,
}

impl Layout {
    pub fn new(area: Rect) -> Self {
        let [_, header, notification, body, footer] = layout::Layout::vertical([
            Constraint::Min(0),
            Constraint::Max(8),
            Constraint::Length(1),
            Constraint::Length(15),
            Constraint::Min(5),
        ])
        .areas(area);
        let [_, body, _] = layout::Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Min(20),
            Constraint::Fill(1),
        ])
        .areas(body);
        let body = layout::Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(body);
        Self {
            header,
            notification,
            username: body[0],
            password: body[1],
            submit: body[2],
            footer,
        }
    }
}

pub struct LoginPage {
    state: LoginState,
    handler: LoginHandler,
}

impl LoginPage {
    pub fn new() -> Self {
        Self {
            state: LoginState::default(),
            handler: LoginHandler::new(),
        }
    }
}

impl AsyncStatefulEventHandler<AppEvent> for LoginPage {
    type State = App;

    async fn handle_event(&mut self, event: AppEvent, app: &mut Self::State) {
        match event {
            AppEvent::Input(event) => {
                if let Event::Key(event) = event {
                    let key = event.code;
                    if key == KeyCode::Esc {
                        app.current_page = Pages::Entry;
                        return;
                    }
                    if key == KeyCode::Enter && self.state.focused == FocusedElement::Submit {
                        let id = &self.state.username_text_box.text;
                        let password = &self.state.password_text_box.text;
                        if id.trim().is_empty() {
                            self.state.login_result = LoginResult::UsernameEmpty;
                            return;
                        }
                        if password.trim().is_empty() {
                            self.state.login_result = LoginResult::PasswordEmpty;
                            return;
                        }
                        match app.login(id, password).await {
                            Ok(_) => match app.connect_database(id, password).await {
                                Ok(_) => match app.net_client.connect_notifications_ws().await {
                                    Ok(ws) => {
                                        let (sender, receiver) = mpsc::unbounded_channel();
                                        app.notification_receiver = Some(receiver);
                                        let private_key =
                                            app.current_user.as_ref().unwrap().private_key.clone();
                                        let db_clone = app.db.as_ref().unwrap().clone();
                                        tokio::spawn(async move {
                                            if let Err(e) = notifications::notification_handler(
                                                ws,
                                                sender,
                                                db_clone,
                                                private_key,
                                            )
                                            .await
                                            {
                                                log::error!("Websocket terminated due to {}", e);
                                            }
                                        });
                                        app.current_page = Pages::Main;
                                    }
                                    Err(e) => {
                                        self.state.login_result = LoginResult::Error;
                                        log::error!("Failed to connect to WebSocket {e}");
                                    }
                                },
                                Err(e) => {
                                    self.state.login_result = LoginResult::Error;
                                    log::error!("{}", e);
                                }
                            },
                            Err(e) => match e {
                                LoginError::RequestError(e) => {
                                    if let Some(status) = e.status() {
                                        if status == StatusCode::NOT_FOUND {
                                            self.state.login_result = LoginResult::UserNotFound;
                                            return;
                                        }
                                        if status == StatusCode::UNAUTHORIZED {
                                            self.state.login_result = LoginResult::WrongPassword;
                                            return;
                                        }
                                    }
                                    self.state.login_result = LoginResult::Error;
                                    return;
                                }
                                LoginError::KeyGen => self.state.login_result = LoginResult::Error,
                            },
                        }
                        return;
                    }
                }
                self.handler.handle_event(event, &mut self.state);
            }
            _ => (),
        }
    }
}
