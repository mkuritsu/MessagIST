use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::Widget,
};

pub struct Cursor<'a> {
    style: Style,
    symbol: &'a str,
}

impl<'a> Cursor<'a> {
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Default for Cursor<'a> {
    fn default() -> Self {
        Self {
            style: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::SLOW_BLINK),
            symbol: symbols::block::FULL,
        }
    }
}

impl<'a> Widget for Cursor<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let span = Span::from(self.symbol).style(self.style);
        span.render(area, buf);
    }
}
