use crate::{
    app::{App, AppEvent, Pages},
    ui::event_handler::{AsyncStatefulEventHandler, EventHandler},
};
use crossterm::event::{Event, KeyCode};
use reqwest::StatusCode;

use super::{
    state::{FocusedElement, RegisterResult},
    RegisterPage,
};

impl AsyncStatefulEventHandler<AppEvent> for RegisterPage {
    type State = App;

    async fn handle_event(&mut self, event: AppEvent, app: &mut Self::State) {
        if let AppEvent::Input(event) = event {
            if let Event::Key(event) = event {
                match event.code {
                    KeyCode::Down | KeyCode::Tab => {
                        self.state.next_focus();
                        return;
                    }
                    KeyCode::Up | KeyCode::BackTab => {
                        self.state.prev_focus();
                        return;
                    }
                    KeyCode::Esc => {
                        app.current_page = Pages::Entry;
                        return;
                    }
                    KeyCode::Enter => {
                        if self.state.focused == FocusedElement::Submit {
                            let id = &self.state.id_text_box.text;
                            let name = &self.state.name_text_box.text;
                            let password = &self.state.password_text_box.text;
                            let re_password = &self.state.re_password_text_box.text;
                            if let Some((_, e)) = [id, name, password, re_password]
                                .iter()
                                .zip([
                                    "IST ID cannot be empty",
                                    "Name cannot be empty",
                                    "Password cannot be empty",
                                    "Password confirmation cannot be empty",
                                ])
                                .filter(|(f, _)| f.trim().is_empty())
                                .next()
                            {
                                self.state.register_result = RegisterResult::Error(e.to_string());
                                return;
                            }
                            let (_, puk) = match app.gen_key_pair(id).await {
                                Ok(v) => v,
                                Err(e) => {
                                    self.state.register_result =
                                        RegisterResult::Error(e.to_string());
                                    return;
                                }
                            };
                            let Ok(key_bytes) = cryptolib::utils::public_key_to_bytes(puk) else {
                                self.state.register_result = RegisterResult::Error(format!(
                                    "Failed to parse rsa public key!"
                                ));
                                return;
                            };
                            match app
                                .net_client
                                .register(id, name, password, &key_bytes)
                                .await
                            {
                                Ok(_) => {
                                    app.just_registered = true;
                                    app.current_page = Pages::Login
                                }
                                Err(e) => {
                                    if let Some(status) = e.status() {
                                        if status == StatusCode::FORBIDDEN {
                                            self.state.register_result = RegisterResult::Error(
                                                "A user with that ID already exists!".to_string(),
                                            );
                                        }
                                    }
                                    self.state.register_result =
                                        RegisterResult::Error(format!("ERROR: {}", e));
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            match self.state.focused {
                FocusedElement::Username => self.state.id_text_box.handle_event(event),
                FocusedElement::Name => self.state.name_text_box.handle_event(event),
                FocusedElement::Password => self.state.password_text_box.handle_event(event),
                FocusedElement::ConfirmPassword => {
                    self.state.re_password_text_box.handle_event(event)
                }
                _ => (),
            }
        }
    }
}
