use std::collections::HashMap;

use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Scrollbar, ScrollbarState, StatefulWidget, Widget},
};
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::{db::structs::StoredMessage, ui::event_handler::EventHandler};

use super::dm_card::DMCard;

pub struct DMListState {
    scrollbar_state: ScrollbarState,
    list_state: ListState,
}

impl DMListState {
    pub fn new(size: usize) -> Self {
        let list_state = ListState::default();
        Self {
            scrollbar_state: ScrollbarState::new(size),
            list_state,
        }
    }
}

impl DMListState {
    pub fn next(&mut self) {
        self.list_state.next();
        self.scrollbar_state.next();
    }

    pub fn prev(&mut self) {
        self.list_state.previous();
        self.scrollbar_state.prev();
    }

    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected
    }
}

pub struct DMList<'a> {
    style: Style,
    block: Option<Block<'a>>,
    inner_block: Option<Block<'a>>,
    scrollbar: bool,
    chats: &'a HashMap<String, Vec<StoredMessage>>,
}

impl<'a> DMList<'a> {
    pub fn new(chats: &'a HashMap<String, Vec<StoredMessage>>) -> Self {
        Self {
            style: Style::default(),
            block: None,
            inner_block: None,
            scrollbar: false,
            chats,
        }
    }

    pub fn inner_block(mut self, block: Block<'a>) -> Self {
        self.inner_block = Some(block);
        self
    }

    pub const fn with_scrollbar(mut self) -> Self {
        self.scrollbar = true;
        self
    }
}

impl<'a> StatefulWidget for DMList<'a> {
    type State = DMListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut DMListState) {
        let area = if let Some(block) = &self.block {
            block.render(area, buf);
            block.inner(area)
        } else {
            area
        };
        let area = if self.scrollbar {
            let [scroll_area, _, area] = Layout::horizontal([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(100),
            ])
            .areas(area);
            let scrollbar = Scrollbar::default();
            scrollbar.render(scroll_area, buf, &mut state.scrollbar_state);
            area
        } else {
            area
        };
        let contacts = self.chats.clone();
        let selected = state.list_state.selected;
        let builder = ListBuilder::new(move |context| {
            let (contact, messages) = contacts
                .iter()
                .nth(context.index)
                .expect("Invalid contact index!");
            let last_message = messages.last();
            let mut card = match last_message {
                Some(last_message) => DMCard::new(contact).message(last_message.content.clone()),
                None => DMCard::new(contact),
            };
            let is_selected = selected.is_some() && selected.unwrap() == context.index;
            if is_selected {
                card = card.block(Block::new().style(Style::new().bg(Color::DarkGray)));
            }
            (card, 3)
        });
        let mut view = ListView::new(builder, self.chats.len())
            .infinite_scrolling(false)
            .style(self.style);
        if let Some(block) = self.inner_block {
            view = view.block(block);
        }
        view.render(area, buf, &mut state.list_state);
    }
}

impl EventHandler<Event> for DMListState {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Up => self.prev(),
                KeyCode::Down => self.next(),
                _ => (),
            },
            _ => (),
        }
    }
}
