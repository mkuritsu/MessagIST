use crossterm::event::{Event, MouseEventKind};
use log::Level;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{
        Block, BorderType, List, ListItem, ListState, Scrollbar, ScrollbarOrientation,
        ScrollbarState, StatefulWidget, Widget,
    },
};

use crate::{app::AppEvent, ui::event_handler::EventHandler};

struct LogLine {
    message: String,
    level: Level,
}

impl LogLine {
    pub fn new(message: String, level: Level) -> Self {
        Self { message, level }
    }
}

#[derive(Default)]
pub struct LoggerWidgetState {
    pub area: Rect,
    logs: Vec<LogLine>,
    selected: usize,
}

#[derive(Default, Clone, Copy)]
pub struct LoggingColors {
    pub trace: Style,
    pub debug: Style,
    pub info: Style,
    pub warn: Style,
    pub error: Style,
}

pub struct LoggerWidget {
    colors: LoggingColors,
}

impl LoggerWidget {
    pub fn new() -> Self {
        Self {
            colors: LoggingColors::default(),
        }
    }

    pub const fn colors(mut self, colors: LoggingColors) -> Self {
        self.colors = colors;
        self
    }
}

impl StatefulWidget for LoggerWidget {
    type State = LoggerWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut LoggerWidgetState)
    where
        Self: Sized,
    {
        state.area = area;
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("MessageIST Logs");
        let inner = block.inner(area);
        block.render(area, buf);
        let [scroll_area, _, list_area] = Layout::horizontal([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Percentage(100),
        ])
        .areas(inner);
        let mut items = Vec::with_capacity(state.logs.len());
        for log in &state.logs {
            let style = match log.level {
                log::Level::Error => self.colors.error,
                log::Level::Warn => self.colors.warn,
                log::Level::Info => self.colors.info,
                log::Level::Debug => self.colors.debug,
                log::Level::Trace => self.colors.trace,
            };
            items.push(ListItem::new(log.message.clone()).style(style));
        }
        let list = List::new(items);
        let mut list_state = ListState::default();
        list_state.select(Some(state.selected));
        StatefulWidget::render(list, list_area, buf, &mut list_state);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalLeft);
        let mut scrollbar_state = ScrollbarState::new(state.logs.len()).position(state.selected);
        scrollbar.render(scroll_area, buf, &mut scrollbar_state);
    }
}

impl EventHandler<AppEvent> for LoggerWidgetState {
    fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Input(event) => match event {
                Event::Mouse(event) => {
                    let kind = event.kind;
                    if kind == MouseEventKind::ScrollUp {
                        self.selected = std::cmp::max(self.selected.saturating_sub(1), 0);
                    } else if kind == MouseEventKind::ScrollDown {
                        self.selected = std::cmp::min(self.selected + 1, self.logs.len() - 1);
                    }
                }
                _ => (),
            },
            AppEvent::Log(record) => {
                let should_scroll = self.selected == self.logs.len().saturating_sub(1);
                record
                    .message
                    .lines()
                    .enumerate()
                    .map(|(i, l)| {
                        if i == 0 {
                            let time = record.time.format("%H:%M:%S");
                            let message = format!("[{}] {} > {}", time, record.level, l);
                            LogLine::new(message, record.level)
                        } else {
                            let message = format!("    {}", l);
                            LogLine::new(message, record.level)
                        }
                    })
                    .for_each(|r| self.logs.push(r));
                if should_scroll {
                    self.selected = self.logs.len().saturating_sub(1);
                }
            }
        }
    }
}
