use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::StatefulWidget};

use crate::app::{App, AppEvent, Pages};

use super::{
    event_handler::AsyncStatefulEventHandler,
    pages::{
        add_contact::AddContactPage, connect::ConnectPage, entry::EntryPage, login::LoginPage,
        main::MainPage, register::RegisterPage,
    },
};

pub struct Router {
    connect_page: ConnectPage,
    entry_page: EntryPage,
    login_page: LoginPage,
    register_page: RegisterPage,
    main_page: MainPage,
    add_contact_page: AddContactPage,
    current_page: Pages,
}

impl Router {
    pub fn new() -> Self {
        Self {
            connect_page: ConnectPage::new(),
            entry_page: EntryPage::new(),
            login_page: LoginPage::new(),
            register_page: RegisterPage::new(),
            main_page: MainPage::new(vec![], vec![]),
            add_contact_page: AddContactPage::new(Color::default()),
            current_page: Pages::Connect,
        }
    }

    pub fn update_pages(&mut self, app: &mut App) {
        if app.current_page != self.current_page {
            self.current_page = app.current_page;
            match app.current_page {
                Pages::Connect => self.connect_page = ConnectPage::new(),
                Pages::Entry => self.entry_page = EntryPage::new(),
                Pages::Login => self.login_page = LoginPage::new(),
                Pages::Register => self.register_page = RegisterPage::new(),
                Pages::Main => {
                    let mut contacts = app.contacts.clone();
                    contacts.sort_by(|c1, c2| c1.name.chars().cmp(c2.name.chars()));
                    let chat_ids = app.messages.keys().cloned().collect();
                    self.main_page = MainPage::new(contacts, chat_ids);
                }
                Pages::AddContact => {
                    self.add_contact_page = AddContactPage::new(app.theme.background)
                }
            }
        }
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer, app: &mut App) {
        match app.current_page {
            Pages::Connect => self.connect_page.render(area, buf, app),
            Pages::Entry => self.entry_page.render(area, buf, app),
            Pages::Login => self.login_page.render(area, buf, app),
            Pages::Register => self.register_page.render(area, buf, app),
            Pages::Main => self.main_page.render(area, buf, app),
            Pages::AddContact => self.add_contact_page.render(area, buf, app),
        }
    }
}

impl AsyncStatefulEventHandler<AppEvent> for Router {
    type State = App;

    async fn handle_event(&mut self, event: AppEvent, app: &mut Self::State) {
        match app.current_page {
            Pages::Connect => self.connect_page.handle_event(event, app).await,
            Pages::Entry => self.entry_page.handle_event(event, app).await,
            Pages::Login => self.login_page.handle_event(event, app).await,
            Pages::Register => self.register_page.handle_event(event, app).await,
            Pages::Main => self.main_page.handle_event(event, app).await,
            Pages::AddContact => self.add_contact_page.handle_event(event, app).await,
        }
    }
}
