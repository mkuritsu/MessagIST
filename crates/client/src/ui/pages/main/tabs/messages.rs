use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::App,
    ui::{
        event_handler::EventHandler,
        widgets::{
            dm_list::{DMList, DMListState},
            opened_chat::{OpenedChat, OpenedChatState},
        },
    },
};

const SYMBOL: &str = "⠀⠀⠀⠀⣀⣤⣤⣤⣄⠀⠀⠀⠀
⠀⠀⠀⣼⣿⠟⠛⠻⣿⣷⡀⠀⠀
⠀⠀⠐⠛⠛⠀⠀⣠⣿⣿⠁⠀⠀
⠀⠀⠀⠀⠀⢀⣾⣿⠟⠁⠀⠀⠀
⠀⠀⠀⠀⠀⠸⠿⠇⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣰⣶⡄⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠘⠛⠁⠀⠀⠀⠀⠀";

const TIP_NO_CONTACT: &str = "Select a contact to start chatting";

pub enum SelectedPage {
    Chatlist,
    ChatMessages,
}

pub struct MessagesTabState {
    chatlist_state: DMListState,
    openchat_state: OpenedChatState,
    selected_page: SelectedPage,
}

impl MessagesTabState {
    pub fn new(chat_size: usize) -> Self {
        Self {
            chatlist_state: DMListState::new(chat_size),
            openchat_state: OpenedChatState::new(),
            selected_page: SelectedPage::Chatlist,
        }
    }

    pub fn get_send_message(&self) -> Option<(usize, String)> {
        if let None = self.chatlist_state.selected() {
            return None;
        }
        if let None = self.openchat_state.get_input_message() {
            return None;
        }
        let selected = self.chatlist_state.selected().unwrap();
        let message = self.openchat_state.get_input_message().unwrap();

        Some((selected, message))
    }

    pub fn clear_input(&mut self) {
        self.openchat_state.clear_input();
    }
}

pub struct MessagesTab<'a> {
    app: &'a mut App,
}

impl<'a> MessagesTab<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> StatefulWidget for MessagesTab<'a> {
    type State = MessagesTabState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [side_bar, info_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(area);
        let [header_area, list_area] =
            Layout::vertical([Constraint::Length(5), Constraint::Min(0)]).areas(side_bar);
        let [_, messages_area] =
            Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(info_area);
        let header = Paragraph::new(vec![Line::default(), Line::from("MY MESSAGES")])
            .alignment(Alignment::Center)
            .block(Block::bordered().border_type(BorderType::Thick));
        header.render(header_area, buf);
        let list: DMList<'_> = match state.selected_page {
            SelectedPage::Chatlist => DMList::new(&self.app.messages)
                .with_scrollbar()
                .inner_block(Block::bordered().border_type(BorderType::Thick)),
            SelectedPage::ChatMessages => DMList::new(&self.app.messages)
                .with_scrollbar()
                .inner_block(Block::bordered()),
        };
        list.render(list_area, buf, &mut state.chatlist_state);

        match state.selected_page {
            SelectedPage::ChatMessages => {
                let highlight = Block::bordered().border_type(BorderType::Thick);
                highlight.render(messages_area, buf);
            }
            _ => {}
        }

        // if no chat is selected, render a tip
        if state.chatlist_state.selected().is_none() {
            render_tip(messages_area, buf);
            return;
        }

        // get the index of the selected chat
        let selected_chat = state.chatlist_state.selected().unwrap();

        // get the contact id associated with the selected chat
        let keys: Vec<&String> = self.app.messages.keys().collect();

        // if the selected chat index is out of bounds, render a tip
        if selected_chat >= keys.len() {
            render_tip(messages_area, buf);
            return;
        }

        let selected_chat_contact_id = keys[selected_chat];

        // get the contact id of the selected chat
        // filter the contacts to get the contact with the selected chat id
        let opened_contact = self
            .app
            .contacts
            .iter()
            .find(|contact| contact.id == *selected_chat_contact_id);

        // if the contact is not found, render a tip
        if opened_contact.is_none() {
            render_tip(messages_area, buf);
            return;
        }

        let opened_contact = opened_contact.unwrap();

        if let Some(messages) = &self.app.messages.get(opened_contact.id.as_str()) {
            let sender_id = self.app.current_user.as_ref().unwrap().id.as_str();
            let opened_chat = OpenedChat::new(&opened_contact, messages, &sender_id);
            opened_chat.render(messages_area, buf, &mut state.openchat_state);
        } else {
            let sender_id = self.app.current_user.as_ref().unwrap().id.as_str();
            let opened_chat = OpenedChat::new(&opened_contact, &[], &sender_id);
            opened_chat.render(messages_area, buf, &mut state.openchat_state);
        }
    }
}

fn render_tip(area: Rect, buf: &mut Buffer) {
    let [_, center, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(9),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [logo_area, _, notify] = Layout::vertical([
        Constraint::Length(7),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(center);

    let logo = Paragraph::new(SYMBOL)
        .alignment(Alignment::Center)
        .block(Block::default());
    logo.render(logo_area, buf);

    let tip = Paragraph::new(TIP_NO_CONTACT)
        .alignment(Alignment::Center)
        .block(Block::default());
    tip.render(notify, buf);
}

impl<'a> EventHandler<Event> for MessagesTabState {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Right => {
                    self.selected_page = SelectedPage::ChatMessages;
                    self.openchat_state.get_focus();
                    return;
                }
                KeyCode::Left => {
                    self.selected_page = SelectedPage::Chatlist;
                    self.openchat_state.lose_focus();
                    return;
                }
                _ => match self.selected_page {
                    SelectedPage::ChatMessages => {
                        self.openchat_state.handle_event(Event::Key(event))
                    }
                    SelectedPage::Chatlist => self.chatlist_state.handle_event(Event::Key(event)),
                },
            },
            _ => {}
        }
    }
}
