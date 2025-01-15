use ratatui::layout::{self, Constraint, Rect};
use state::EntryState;

mod handler;
mod state;
mod widget;

#[derive(Default, Clone, Copy)]
pub struct Layout {
    header: Rect,
    login: Rect,
    register: Rect,
    footer: Rect,
}

impl Layout {
    pub fn new(area: Rect) -> Self {
        let [_, header, body, _, footer] = layout::Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(13),
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(5),
        ])
        .areas(area);
        let [_, login, _, register, _] =
            layout::Layout::horizontal([Constraint::Percentage(20); 5]).areas(body);
        Self {
            header,
            login,
            register,
            footer,
        }
    }
}

pub struct EntryPage {
    state: EntryState,
}

impl EntryPage {
    pub fn new() -> Self {
        Self {
            state: EntryState::default(),
        }
    }
}
