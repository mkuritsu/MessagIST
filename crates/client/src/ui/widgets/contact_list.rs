use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{
        Block, List, ListItem, ListState, Scrollbar, ScrollbarState, StatefulWidget, Widget,
    },
};

use crate::db::structs::Contact;

#[derive(Default)]
pub struct ContactListState {
    scroll_state: ScrollbarState,
    list_state: ListState,
}

impl ContactListState {
    pub fn new(count: usize) -> Self {
        let mut list = ListState::default();
        list.select_first();
        Self {
            scroll_state: ScrollbarState::new(count),
            list_state: list,
        }
    }

    pub fn next(&mut self) {
        self.scroll_state.next();
        self.list_state.select_next();
    }

    pub fn prev(&mut self) {
        self.scroll_state.prev();
        self.list_state.select_previous();
    }

    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected()
    }
}

pub struct ContactList<'a> {
    contacts: &'a Vec<Contact>,
    style: Style,
    item_style: Style,
    selected_style: Style,
    selected_prefix: Option<String>,
    block: Option<Block<'a>>,
    inner_block: Option<Block<'a>>,
    scrollbar: bool,
}

impl<'a> ContactList<'a> {
    pub fn new(contacts: &'a Vec<Contact>) -> Self {
        Self {
            contacts,
            style: Style::default(),
            item_style: Style::default(),
            selected_style: Style::default(),
            selected_prefix: None,
            block: None,
            inner_block: None,
            scrollbar: false,
        }
    }

    pub fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn selected_prefix<T: Into<String>>(mut self, prefix: T) -> Self {
        self.selected_prefix = Some(prefix.into());
        self
    }

    pub fn inner_block(mut self, block: Block<'a>) -> Self {
        self.inner_block = Some(block);
        self
    }

    pub fn with_scrollbar(mut self) -> Self {
        self.scrollbar = true;
        self
    }
}

impl<'a> StatefulWidget for ContactList<'a> {
    type State = ContactListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items = self.contacts.iter().enumerate().into_iter().map(|(i, c)| {
            let (text, style) = if let Some(selected) = state.list_state.selected() {
                if i == selected {
                    if let Some(prefix) = &self.selected_prefix {
                        (format!("{}{}", prefix, c.name), self.selected_style)
                    } else {
                        (c.name.to_string(), self.selected_style)
                    }
                } else {
                    (c.name.to_string(), self.item_style)
                }
            } else {
                (c.name.to_string(), self.item_style)
            };
            ListItem::new(text).style(style)
        });
        let area = if let Some(block) = self.block {
            let area = block.inner(area);
            block.render(area, buf);
            area
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
            scrollbar.render(scroll_area, buf, &mut state.scroll_state);
            area
        } else {
            area
        };
        let mut list = List::new(items).style(self.style);
        if let Some(block) = self.inner_block {
            list = list.block(block);
        }
        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
