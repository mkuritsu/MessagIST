use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, Clear, StatefulWidget, Widget},
};
use tachyonfx::{fx, Duration, Effect, EffectTimer, Interpolation, Motion, Shader};

use crate::ui::{theming::AppTheme, widgets::contact_info::ContactInfo, RENDER_TICKRATE};

#[derive(Clone)]
pub struct ProfilePopupState {
    slide_effect: Effect,
}

impl ProfilePopupState {
    pub fn new(color: Color) -> Self {
        let slide_effect = fx::slide_in(
            Motion::UpToDown,
            0,
            0,
            color,
            EffectTimer::new(Duration::from_millis(200), Interpolation::Linear),
        );
        Self { slide_effect }
    }
}

pub struct ProfilePopup<'a> {
    name: &'a str,
    id: &'a str,
    public_key: &'a str,
    theme: &'a AppTheme,
}

impl<'a> ProfilePopup<'a> {
    pub fn new(name: &'a str, id: &'a str, public_key: &'a str, theme: &'a AppTheme) -> Self {
        Self {
            name,
            id,
            public_key,
            theme,
        }
    }
}

impl<'a> StatefulWidget for ProfilePopup<'a> {
    type State = ProfilePopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ProfilePopupState)
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
        let [top, center, bottom] = build_layout(inner);
        let card = ContactInfo::new(self.name, self.id, self.public_key)
            .picture_style(self.theme.accent_style())
            .name_style(self.theme.warn_style())
            .id_style(self.theme.subtext_stye())
            .public_key_style(self.theme.text_style());
        card.render(center, buf);
        let footer = Line::from("Pree ESC to close this dialog")
            .centered()
            .style(self.theme.subtext_stye());
        footer.render(bottom, buf);
        let header = Line::from("Your profile")
            .centered()
            .style(self.theme.text_style());
        header.render(top, buf);
        state
            .slide_effect
            .process(RENDER_TICKRATE.into(), buf, area);
    }
}

fn build_layout(area: Rect) -> [Rect; 3] {
    const SIDE_PADDING: u16 = 4;
    const TOP_PADDING: u16 = 2;
    const BOTTOM_PADDING: u16 = 2;
    let [_, center, _] = Layout::vertical([
        Constraint::Length(TOP_PADDING * 2 + 1),
        Constraint::Percentage(100),
        Constraint::Percentage(BOTTOM_PADDING * 2 + 1),
    ])
    .areas(area);
    let [_, center, _] = Layout::horizontal([
        Constraint::Length(SIDE_PADDING),
        Constraint::Percentage(100),
        Constraint::Length(SIDE_PADDING),
    ])
    .areas(center);
    let [_, bottom, _] = Layout::vertical([
        Constraint::Percentage(100),
        Constraint::Length(1),
        Constraint::Length(BOTTOM_PADDING),
    ])
    .areas(area);
    let [_, top, _] = Layout::vertical([
        Constraint::Length(TOP_PADDING),
        Constraint::Length(1),
        Constraint::Percentage(100),
    ])
    .areas(area);
    [top, center, bottom]
}
