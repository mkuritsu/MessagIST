use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::Widget,
};

#[derive(Default, Clone, Copy)]
pub struct ButtonColors {
    pub text: Color,
    pub shadow: Color,
    pub background: Color,
    pub highlight: Color,
}

pub struct Button<'a> {
    label: Line<'a>,
    state: ButtonState,
    colors: ButtonColors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Selected,
}

impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        Button {
            label: label.into(),
            state: ButtonState::Normal,
            colors: ButtonColors::default(),
        }
    }

    pub const fn state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    pub const fn colors(mut self, colors: ButtonColors) -> Self {
        self.colors = colors;
        self
    }

    const fn parse_colors(&self) -> (Color, Color, Color, Color) {
        match self.state {
            ButtonState::Normal => (
                self.colors.background,
                self.colors.text,
                self.colors.shadow,
                self.colors.highlight,
            ),
            ButtonState::Selected => (
                self.colors.highlight,
                self.colors.text,
                self.colors.shadow,
                self.colors.highlight,
            ),
        }
    }
}

impl<'a> Widget for Button<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (background, text, shadow, highlight) = self.parse_colors();
        buf.set_style(area, Style::new().bg(background).fg(text));

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new().fg(shadow).bg(background),
            );
        }
        // render label centered
        buf.set_line(
            area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}
