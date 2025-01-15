use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

pub struct TitleWidget<'a> {
    block: Option<Block<'a>>,
    style: Style,
    text: String,
}

impl<'a> TitleWidget<'a> {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            block: None,
            style: Style::default(),
            text: text.into(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for TitleWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let area = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        } else {
            area
        };
        let y = area.y + area.height / 2;
        let line = Line::from(self.text).centered().style(self.style);
        let title_area = Rect::new(area.x, y, area.width, area.height);
        line.render(title_area, buf);
    }
}
