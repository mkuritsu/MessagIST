use base64::Engine;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::db::structs::Contact;

const PICTURE: &'static str = r#"
⠀⠀⠀⠀⠀⠀⢀⣀⡀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⣾⣻⣽⣟⣷⡀⠀⠀⠀⠀
⠀⠀⠀⠀⢸⣯⣟⣷⣟⣷⡇⠀⠀⠀⠀
⠀⠀⠀⠀⠈⠳⣟⣷⣟⠗⠁⠀⠀⠀⠀
⠀⠀⠀⢀⣶⡷⣦⣤⡴⣾⢶⡀⠀⠀⠀
⠀⠀⠀⣿⣳⣿⣻⣾⣻⣟⣿⣻⠀⠀⠀
⠀⠀⠀⠻⣽⡾⣿⣞⣯⡿⣾⠛⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠉⠈⠉⠀⠀⠀⠀⠀⠀"#;

pub struct ContactInfo<'a> {
    name: String,
    id: String,
    public_key: String,
    picture_style: Style,
    name_style: Style,
    id_style: Style,
    public_key_style: Style,
    block: Option<Block<'a>>,
}

impl<'a> ContactInfo<'a> {
    pub fn new(name: &str, id: &str, public_key: &str) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            public_key: public_key.to_string(),
            picture_style: Style::default(),
            name_style: Style::default(),
            id_style: Style::default(),
            public_key_style: Style::default(),
            block: None,
        }
    }

    pub fn picture_style(mut self, style: Style) -> Self {
        self.picture_style = style;
        self
    }

    pub fn name_style(mut self, style: Style) -> Self {
        self.name_style = style;
        self
    }

    pub fn id_style(mut self, style: Style) -> Self {
        self.id_style = style;
        self
    }

    pub fn public_key_style(mut self, style: Style) -> Self {
        self.public_key_style = style;
        self
    }
}

impl<'a> Widget for ContactInfo<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let fields = [self.name, format!("@{}", self.id), self.public_key];
        let styles = [self.name_style, self.id_style, self.public_key_style];
        let mut lines = PICTURE
            .split('\n')
            .map(|x| {
                Line::from(x)
                    .alignment(Alignment::Center)
                    .style(self.picture_style)
            })
            .collect::<Vec<Line>>();
        let picture_len = lines.len();
        lines.extend(fields.iter().zip(styles).map(|(f, s)| {
            Line::from(f.to_string())
                .alignment(Alignment::Center)
                .style(s)
        }));
        lines.insert(picture_len, Line::default());
        lines.insert(lines.len() - 1, Line::default());
        let mut p = Paragraph::new(lines).wrap(Wrap { trim: true });
        if let Some(block) = self.block {
            p = p.block(block);
        }
        p.render(area, buf);
    }
}

impl<'a> From<&Contact> for ContactInfo<'a> {
    fn from(value: &Contact) -> Self {
        let b64 = base64::engine::general_purpose::STANDARD.encode(&value.public_key);
        ContactInfo::new(&value.name, &value.id, &b64)
    }
}
