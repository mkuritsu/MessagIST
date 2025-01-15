use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, StatefulWidget, Widget},
};

use crate::ui::event_handler::EventHandler;

use super::cursor::Cursor;

#[derive(Default, Clone)]
pub struct TextBoxState {
    pub cursor_pos: u16,
    pub text: String,
}

impl TextBoxState {
    pub fn move_cursor(&mut self, movement: CursorMovement) {
        match movement {
            CursorMovement::Forward => {
                self.cursor_pos = std::cmp::min(self.cursor_pos + 1, self.text.len() as u16);
            }
            CursorMovement::Backwards => {
                self.cursor_pos = self.cursor_pos.saturating_sub(1);
            }
            CursorMovement::End => self.cursor_pos = self.text.len() as u16,
            CursorMovement::Start => self.cursor_pos = 0,
        }
    }

    pub fn write_char(&mut self, ch: char) {
        self.text.insert(self.cursor_pos as usize, ch);
        self.cursor_pos += 1;
    }

    pub fn erase_char(&mut self) {
        if self.cursor_pos > 0 {
            self.text.remove((self.cursor_pos - 1) as usize);
            self.cursor_pos -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_pos = 0;
    }
}

pub enum CursorMovement {
    Forward,
    Backwards,
    End,
    Start,
}

#[derive(Default)]
pub struct TextBox<'a> {
    cursor: Option<Cursor<'a>>,
    block: Option<Block<'a>>,
    style: Style,
    placeholder: Option<String>,
    placeholder_style: Style,
    censored: bool,
}

impl<'a> TextBox<'a> {
    pub fn new() -> Self {
        Self {
            cursor: None,
            block: None,
            style: Style::default(),
            placeholder: None,
            placeholder_style: Style::new().fg(Color::Gray),
            censored: false,
        }
    }

    pub const fn cursor(mut self, cursor: Cursor<'a>) -> Self {
        self.cursor = Some(cursor);
        self
    }

    pub fn block(mut self, b: Block<'a>) -> Self {
        self.block = Some(b);
        self
    }

    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub const fn placeholder_style(mut self, style: Style) -> Self {
        self.placeholder_style = style;
        self
    }

    pub fn placeholder(mut self, text: &str) -> Self {
        self.placeholder = Some(text.to_string());
        self
    }

    pub fn censored(mut self) -> Self {
        self.censored = true;
        self
    }
}

impl<'a> StatefulWidget for TextBox<'a> {
    type State = TextBoxState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TextBoxState)
    where
        Self: Sized,
    {
        let inner = if let Some(block) = &self.block {
            block.render(area, buf);
            block.inner(area)
        } else {
            area
        };
        let y = inner.y + inner.height / 2;
        if state.text.len() > 0 {
            if self.censored {
                let text = std::iter::repeat("*")
                    .take(state.text.len())
                    .collect::<String>();
                buf.set_string(inner.x, y, text, self.style);
            } else {
                buf.set_string(inner.x, y, &state.text, self.style);
            }
        } else if let Some(placeholder) = &self.placeholder {
            buf.set_string(inner.x, y, placeholder, self.placeholder_style);
        }
        if let Some(cursor) = self.cursor {
            let x = inner.x + state.cursor_pos;
            let area = Rect::new(x, y, 1, 1);
            cursor.render(area, buf);
        }
    }
}

impl EventHandler<Event> for TextBoxState {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Backspace => self.erase_char(),
                KeyCode::Left => self.move_cursor(CursorMovement::Backwards),
                KeyCode::Right => self.move_cursor(CursorMovement::Forward),
                KeyCode::End => self.move_cursor(CursorMovement::End),
                KeyCode::Home => self.move_cursor(CursorMovement::Start),
                KeyCode::Char(ch) => self.write_char(ch),
                _ => (),
            },
            Event::Paste(text) => text.chars().into_iter().for_each(|ch| self.write_char(ch)),
            _ => (),
        }
    }
}
