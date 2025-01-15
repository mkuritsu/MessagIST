use ratatui::layout::{self, Constraint, Rect};
use state::RegisterState;

mod handler;
mod state;
mod widget;

#[derive(Default)]
pub struct Layout {
    header: Rect,
    notification: Rect,
    username: Rect,
    name: Rect,
    password: Rect,
    re_password: Rect,
    submit: Rect,
    footer: Rect,
}

impl Layout {
    fn new(area: Rect) -> Self {
        let [_, header, notification, body, bottom] = layout::Layout::vertical([
            Constraint::Min(0),
            Constraint::Max(8),
            Constraint::Length(3),
            Constraint::Length(18),
            Constraint::Min(5),
        ])
        .areas(area);
        let [_, center_body, _] = layout::Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Min(20),
            Constraint::Fill(1),
        ])
        .areas(body);

        let [username, name, password, re_password, submit] = layout::Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .areas(center_body);

        Self {
            header,
            notification,
            username,
            name,
            password,
            re_password,
            submit,
            footer: bottom,
        }
    }
}

pub struct RegisterPage {
    state: RegisterState,
}

impl RegisterPage {
    pub fn new() -> Self {
        Self {
            state: RegisterState::default(),
        }
    }
}
