use crossterm::event::{Event, KeyCode};
use logout_popup::LogoutPopupState;
use profile_popup::ProfilePopupState;
use ratatui::layout::{self, Constraint, Rect};
use state::{MainState, MessagesTab, SelectedTab, ShowingPopup};

use crate::{
    app::{App, AppEvent, Pages},
    db::structs::Contact,
    ui::event_handler::{AsyncStatefulEventHandler, EventHandler},
};

mod logout_popup;
mod profile_popup;
mod state;
mod tabs;
mod widget;

#[derive(Default)]
pub struct Layout {
    title: Rect,
    tabs: Rect,
    body: Rect,
    footer: Rect,
}

impl Layout {
    pub fn new(area: Rect) -> Self {
        let [header, tabs, body, _, footer, _] = layout::Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Percentage(100),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .areas(area);
        let [title] = layout::Layout::horizontal([Constraint::Length(20)]).areas(header);
        Self {
            title,
            tabs,
            body,
            footer,
        }
    }
}

pub struct MainPage {
    state: MainState,
}

impl MainPage {
    pub fn new(contacts: Vec<Contact>, chats: Vec<String>) -> Self {
        Self {
            state: MainState::new(contacts, chats),
        }
    }
}

impl AsyncStatefulEventHandler<AppEvent> for MainPage {
    type State = App;

    async fn handle_event(&mut self, event: AppEvent, app: &mut Self::State) {
        match event {
            AppEvent::Input(event) => {
                if let ShowingPopup::Logout(logout) = &self.state.poup {
                    self.handle_popup(event, app, logout.clone()).await;
                    return;
                }
                if let ShowingPopup::Profile(_) = self.state.poup {
                    if let Event::Key(event) = event {
                        if event.code == KeyCode::Esc {
                            self.state.poup = ShowingPopup::None;
                        }
                    }
                    return;
                }
                if let Event::Key(event) = event {
                    let key = event.code;
                    match key {
                        KeyCode::Tab | KeyCode::BackTab => {
                            self.state.switch_tab();
                        }
                        KeyCode::Left => {
                            self.state.switch_msglist();
                        }
                        KeyCode::Right => {
                            self.state.switch_msgchat();
                        }
                        KeyCode::Esc => {
                            self.state.poup =
                                ShowingPopup::Logout(LogoutPopupState::new(app.theme.background));
                            return;
                        }
                        KeyCode::Char('u') | KeyCode::Char('U') => {
                            if !(self.state.selected_tab
                                == SelectedTab::Messages(MessagesTab::ChatMessages))
                            {
                                self.state.poup = ShowingPopup::Profile(ProfilePopupState::new(
                                    app.theme.background,
                                ));
                            }
                        }
                        KeyCode::Char('+') => {
                            if !(self.state.selected_tab
                                == SelectedTab::Messages(MessagesTab::ChatMessages))
                            {
                                app.current_page = Pages::AddContact;
                            }
                        }
                        KeyCode::Enter => {
                            if self.state.selected_tab
                                == SelectedTab::Messages(MessagesTab::ChatMessages)
                            {
                                if self.state.messages_state.get_send_message().is_none() {
                                    return;
                                }
                                let (contact_index, message) =
                                    self.state.messages_state.get_send_message().unwrap();

                                if app.messages.keys().nth(contact_index).is_none() {
                                    return;
                                }
                                let contact_id = app.messages.keys().nth(contact_index).unwrap();
                                // iterate through contacts and find the contact with the same id
                                let contact = app
                                    .contacts
                                    .iter()
                                    .find(|contact| contact.id == *contact_id)
                                    .cloned();
                                if contact.is_none() {
                                    return;
                                }
                                let contact = contact.unwrap();
                                match app.send_message(&contact, &message).await {
                                    Ok(_) => {
                                        self.state.messages_state.clear_input();
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        _ => (),
                    }
                }
                match self.state.selected_tab {
                    SelectedTab::Messages(_) => self.state.messages_state.handle_event(event),
                    SelectedTab::Contacts => self.state.contacts_state.handle_event(event),
                }
            }
            _ => (),
        }
    }
}

impl MainPage {
    async fn handle_popup(&mut self, event: Event, app: &mut App, mut logout: LogoutPopupState) {
        logout.handle_event(event.clone());
        self.state.poup = ShowingPopup::Logout(logout.clone());
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Esc => self.state.poup = ShowingPopup::None,
                KeyCode::Enter => match logout.selected_option {
                    logout_popup::SelectedOption::Logout => {
                        let _ = app.net_client.logout().await;
                        app.should_quit = true;
                    }
                    logout_popup::SelectedOption::Cancel => self.state.poup = ShowingPopup::None,
                },
                _ => (),
            }
        }
    }
}
