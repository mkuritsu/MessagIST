use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

pub struct DMCard<'a> {
    contact_name: String,
    name_style: Style,
    message: Option<String>,
    message_style: Style,
    block: Option<Block<'a>>,
}

impl<'a> DMCard<'a> {
    pub fn new<T: Into<String>>(contact_name: T) -> Self {
        Self {
            contact_name: contact_name.into(),
            name_style: Style::default(),
            message: None,
            message_style: Style::default(),
            block: None,
        }
    }

    pub fn message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> Widget for DMCard<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
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
        let [name_area, msg_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);
        let name = Line::from(self.contact_name).style(self.name_style);
        name.render(name_area, buf);
        let message = match &self.message {
            Some(v) => v,
            None => "*No messages recorded*",
        };
        let message = Line::from(message).style(self.message_style);
        message.render(msg_area, buf);
    }
}
