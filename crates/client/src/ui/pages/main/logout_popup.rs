use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, Clear, StatefulWidget, Widget},
};
use tachyonfx::{fx, Duration, Effect, EffectTimer, Interpolation, Motion, Shader};

use crate::ui::{
    event_handler::EventHandler,
    theming::AppTheme,
    widgets::button::{Button, ButtonState},
    RENDER_TICKRATE,
};

#[derive(Clone, Copy)]
pub enum SelectedOption {
    Logout,
    Cancel,
}

#[derive(Clone)]
pub struct LogoutPopupState {
    pub selected_option: SelectedOption,
    logout_area: Rect,
    cancel_area: Rect,
    slide_effect: Effect,
}

impl LogoutPopupState {
    pub fn new(color: Color) -> Self {
        let slide_effect = fx::slide_in(
            Motion::UpToDown,
            0,
            0,
            color,
            EffectTimer::new(Duration::from_millis(150), Interpolation::Linear),
        );
        Self {
            selected_option: SelectedOption::Logout,
            logout_area: Rect::default(),
            cancel_area: Rect::default(),
            slide_effect,
        }
    }
}

pub struct LogoutPopup<'a> {
    theme: &'a AppTheme,
}

impl<'a> LogoutPopup<'a> {
    pub fn new(theme: &'a AppTheme) -> Self {
        Self { theme }
    }
}

impl<'a> StatefulWidget for LogoutPopup<'a> {
    type State = LogoutPopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let clear = Clear::default();
        clear.render(area, buf);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(self.theme.background_style());
        let inner = block.inner(area);
        block.render(area, buf);
        let [top, bottom] = Layout::vertical([Constraint::Percentage(50); 2]).areas(inner);
        let [_, left, _, right, _] = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Percentage(40),
            Constraint::Min(0),
            Constraint::Percentage(40),
            Constraint::Min(0),
        ])
        .areas(bottom);
        state.logout_area = left;
        state.cancel_area = right;
        let line = Line::from("Do you want to logout?")
            .centered()
            .style(self.theme.text_style());
        line.render(top, buf);
        let mut logout = Button::new("Logout").colors(self.theme.button_colors());
        let mut cancel = Button::new("Cancel").colors(self.theme.button_colors());
        match state.selected_option {
            SelectedOption::Logout => logout = logout.state(ButtonState::Selected),
            SelectedOption::Cancel => cancel = cancel.state(ButtonState::Selected),
        }
        logout.render(left, buf);
        cancel.render(right, buf);
        state
            .slide_effect
            .process(RENDER_TICKRATE.into(), buf, area);
    }
}

impl EventHandler<Event> for LogoutPopupState {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Left => self.selected_option = SelectedOption::Logout,
                KeyCode::Right => self.selected_option = SelectedOption::Cancel,
                _ => (),
            },
            _ => (),
        }
    }
}
