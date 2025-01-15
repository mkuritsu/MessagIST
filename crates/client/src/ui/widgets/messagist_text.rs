use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};

const LOGO_PARTS: [&'static str; 6] = ["      ", " ╭╮", "Messag", "I│T", "      ", "╰╯ "];
const STYLE1: Style = Style::new()
    .fg(Color::White)
    .bg(Color::Rgb(0, 158, 226))
    .add_modifier(Modifier::BOLD);
const STYLE2: Style = Style::new()
    .fg(Color::Rgb(0, 158, 226))
    .add_modifier(Modifier::BOLD);
const STYLE3: Style = Style::new()
    .fg(Color::White)
    .bg(Color::Rgb(0, 158, 226))
    .add_modifier(Modifier::BOLD);
const STYLE5: Style = Style::new()
    .fg(Color::White)
    .bg(Color::Rgb(0, 158, 226))
    .add_modifier(Modifier::BOLD);

pub struct MessageISTText;

impl MessageISTText {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for MessageISTText {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let part0 = Span::from(LOGO_PARTS[0]);
        let part1 = Span::styled(LOGO_PARTS[1], STYLE1);
        let part2 = Span::styled(LOGO_PARTS[2], STYLE2);
        let part3 = Span::styled(LOGO_PARTS[3], STYLE3);
        let part4 = Span::from(LOGO_PARTS[4]);
        let part5 = Span::styled(LOGO_PARTS[5], STYLE5);
        let line0 = Line::from(vec![part0, part1]);
        let line1 = Line::from(vec![part2, part3]);
        let line2 = Line::from(vec![part4, part5]);
        let text = Text::from(vec![line0, line1, line2]);
        let p = Paragraph::new(text);
        p.render(area, buf);
    }
}
