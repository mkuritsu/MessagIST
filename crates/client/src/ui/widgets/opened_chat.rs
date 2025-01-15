use std::{collections::VecDeque, usize};

use chrono::DateTime;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph, Scrollbar,
        ScrollbarState, StatefulWidget, Widget,
    },
};

use crate::{
    db::structs::{Contact, StoredMessage},
    ui::event_handler::EventHandler,
};

use super::{
    cursor::Cursor,
    text_box::{TextBox, TextBoxState},
};

#[derive(Default)]
pub struct OpenedChatState {
    scroll_offset: usize,
    input_state: TextBoxState,
    selected_element: SelectedElement,
}

#[derive(Debug, Default, PartialEq)]
pub enum SelectedElement {
    #[default]
    None,
    Input,
}

impl OpenedChatState {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0,
            input_state: TextBoxState::default(),
            selected_element: SelectedElement::None,
        }
    }

    pub fn clear_input(&mut self) {
        self.input_state.clear();
    }

    pub fn get_input_message(&self) -> Option<String> {
        if self.input_state.text.is_empty() {
            return None;
        }
        Some(self.input_state.text.clone())
    }

    pub fn get_focus(&mut self) {
        self.selected_element = SelectedElement::Input;
    }

    pub fn lose_focus(&mut self) {
        self.selected_element = SelectedElement::None;
    }

    pub fn next(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn prev(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}

pub struct OpenedChat<'a> {
    self_id: &'a str,
    contact: &'a Contact,
    messages: &'a [StoredMessage],
}

impl<'a> OpenedChat<'a> {
    pub fn new(contact: &'a Contact, messages: &'a [StoredMessage], self_id: &'a &str) -> Self {
        Self {
            self_id,
            contact,
            messages,
        }
    }
}

impl<'a> StatefulWidget for OpenedChat<'a> {
    type State = OpenedChatState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // header
        // ----------
        // messages_area
        // ----------
        // input_area
        let [header, messages_area, input_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .areas(area);

        // contact_name | _ | more
        let [chatname_area, _, more_area] = Layout::horizontal([
            Constraint::Length(20),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(header);

        // if the contact name is longer than 20 characters, truncate it
        let contact_name = if self.contact.name.len() > 20 {
            &self.contact.name[..20]
        } else {
            &self.contact.name
        };

        let header_block = Block::default().borders(Borders::ALL);
        header_block.render(header, buf);

        // render the contact name
        let chat_name =
            Paragraph::new(contact_name).block(Block::default().padding(Padding::new(1, 0, 1, 1)));
        chat_name.render(chatname_area, buf);

        // render the more button
        let more = Paragraph::new("⋮").block(Block::default().padding(Padding::new(0, 1, 1, 1)));
        more.render(more_area, buf);

        // render the messages
        let [messages_list_area, scrollbar_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Length(2)]).areas(messages_area);

        let max_width = messages_list_area.width as usize;

        // select the messages to be displayed
        let sorted_messages = sort_messages(self.messages);

        let mut items = vec![];
        for message in sorted_messages.iter() {
            let lines = convert_message_to_lines(message, (max_width / 2) - 3);
            let mut lines = wrap_message_with_round_border(lines);
            let datetime =
                DateTime::parse_from_rfc3339(&message.timestamp).expect("Failed to convert date");
            lines.insert(0, datetime.format("%d/%m/%Y %H:%M:%S").to_string());
            for line in lines {
                if message.sender_istid == self.self_id {
                    let pad_len = max_width - line.chars().count();
                    let mut string = String::new();
                    for _ in 0..pad_len {
                        string.push(' ');
                    }
                    string.push_str(&line);
                    items.push(ListItem::new(string));
                } else {
                    items.push(ListItem::new(line));
                }
            }
        }
        let scroll_count = items.len().saturating_sub(state.scroll_offset);
        let mut scroll_state = ScrollbarState::new(items.len()).position(scroll_count);
        let mut list_state = ListState::default();
        list_state.select(Some(scroll_count));
        let list = List::new(items);
        StatefulWidget::render(list, messages_list_area, buf, &mut list_state);
        let scrollbar = Scrollbar::default();
        scrollbar.render(scrollbar_area, buf, &mut scroll_state);
        let mut text_box = TextBox::new().block(Block::bordered().border_type(BorderType::Rounded));
        if state.selected_element == SelectedElement::Input {
            text_box = text_box.cursor(Cursor::default());
        }
        text_box.render(input_area, buf, &mut state.input_state);
    }
}

fn sort_messages(messages: &[StoredMessage]) -> VecDeque<StoredMessage> {
    let mut messages: Vec<StoredMessage> = messages.to_vec();
    messages.sort_by(|a, b| {
        a.sent_counter
            .cmp(&b.sent_counter)
            .then_with(|| a.receive_counter.cmp(&b.receive_counter))
            .then_with(|| a.id.cmp(&b.id))
    });
    messages.iter().cloned().collect()
}

fn convert_message_to_lines(message: &StoredMessage, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in message.content.split_whitespace() {
        if current_line.len() + word.len() > max_width {
            lines.push(current_line.clone());
            current_line.clear();
        }
        current_line.push_str(word);
        current_line.push(' ');
    }
    lines.push(current_line);
    lines
}

fn wrap_message_with_round_border(lines: Vec<String>) -> Vec<String> {
    let mut wrapped = Vec::new();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut upper = String::from("╭");
    for _ in 0..(max_len) {
        upper.push('─');
    }
    upper.push('╮');
    wrapped.push(upper);
    for line in lines {
        wrapped.push(format!("│{}│", line));
    }
    let mut down = String::from("╰");
    for _ in 0..(max_len) {
        down.push('─');
    }
    down.push('╯');
    wrapped.push(down);
    wrapped
}

impl EventHandler<Event> for OpenedChatState {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Up => {
                    self.next();
                }
                KeyCode::Down => {
                    self.prev();
                }
                _ => match self.selected_element {
                    SelectedElement::Input => {
                        self.input_state.handle_event(Event::Key(event));
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }
}
