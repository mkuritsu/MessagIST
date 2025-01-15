use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::{Block, BorderType, StatefulWidget, Widget},
};

use crate::{
    app::App,
    ui::{
        event_handler::EventHandler,
        widgets::{
            contact_info::ContactInfo,
            contact_list::{ContactList, ContactListState},
            title::TitleWidget,
        },
    },
};

pub struct ContactsTabState {
    list_state: ContactListState,
}

impl ContactsTabState {
    pub fn new(size: usize) -> Self {
        Self {
            list_state: ContactListState::new(size),
        }
    }
}

pub struct ContactsTab<'a> {
    app: &'a mut App,
}

impl<'a> ContactsTab<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> StatefulWidget for ContactsTab<'a> {
    type State = ContactsTabState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [side_bar, info_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(area);
        let [header_area, list_area] =
            Layout::vertical([Constraint::Length(5), Constraint::Min(0)]).areas(side_bar);
        let [_, info_area] =
            Layout::vertical([Constraint::Length(5), Constraint::Min(0)]).areas(info_area);
        let title = TitleWidget::new("MY CONTACTS")
            .block(
                Block::bordered()
                    .border_type(BorderType::Thick)
                    .style(self.app.theme.text_style()),
            )
            .style(self.app.theme.text_style().add_modifier(Modifier::BOLD));
        title.render(header_area, buf);
        let list = ContactList::new(&self.app.contacts)
            .with_scrollbar()
            .selected_prefix(" >> ")
            .selected_style(self.app.theme.accent_style().add_modifier(Modifier::BOLD))
            .inner_block(Block::bordered().style(self.app.theme.text_style()));
        list.render(list_area, buf, &mut state.list_state);
        if let Some(selected) = state.list_state.selected() {
            if let Some(contact) = self.app.contacts.get(selected) {
                let contact_info = ContactInfo::from(contact)
                    .picture_style(self.app.theme.accent_style())
                    .name_style(self.app.theme.warn_style())
                    .id_style(self.app.theme.subtext_stye())
                    .public_key_style(self.app.theme.text_style());
                contact_info.render(info_area, buf);
            }
        }
    }
}

impl EventHandler<Event> for ContactsTabState {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Up => self.list_state.prev(),
                KeyCode::Down => self.list_state.next(),
                _ => (),
            },
            _ => (),
        }
    }
}
