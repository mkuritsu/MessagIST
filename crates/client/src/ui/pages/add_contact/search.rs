use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::Line,
    widgets::{Block, BorderType, StatefulWidget, Widget},
};
use reqwest::StatusCode;
use tachyonfx::CenteredShrink;

use crate::{
    app::App,
    ui::{
        event_handler::{AsyncStatefulEventHandler, EventHandler},
        theming::AppTheme,
        widgets::{
            button::{Button, ButtonState},
            cursor::Cursor,
            text_box::{TextBox, TextBoxState},
            title::TitleWidget,
        },
    },
};

pub enum SearchContactResult {
    None,
    Error(String),
    Success(String),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectedElement {
    TextBox,
    Button,
}

pub struct SearchContactState {
    text_box: TextBoxState,
    selected_element: SelectedElement,
    pub active: bool,
    result: SearchContactResult,
}

impl Default for SearchContactState {
    fn default() -> Self {
        Self {
            text_box: TextBoxState::default(),
            selected_element: SelectedElement::TextBox,
            active: false,
            result: SearchContactResult::None,
        }
    }
}

pub struct SearchContactWidget<'a> {
    block: Option<Block<'a>>,
    theme: &'a AppTheme,
}

impl<'a> SearchContactWidget<'a> {
    pub fn new(theme: &'a AppTheme) -> Self {
        Self { block: None, theme }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for SearchContactWidget<'a> {
    type State = SearchContactState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        } else {
            area
        };
        let [header, body] =
            Layout::vertical([Constraint::Length(5), Constraint::Percentage(100)]).areas(area);
        let title = TitleWidget::new("SEARCH CONTACT")
            .block(
                Block::bordered()
                    .border_type(BorderType::Thick)
                    .style(self.theme.text_style().add_modifier(Modifier::BOLD)),
            )
            .style(self.theme.text_style().add_modifier(Modifier::BOLD));
        title.render(header, buf);
        let center = body.inner_centered(body.width - body.width / 5, 10);
        let [warn_area, _, input, _, button] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
        ])
        .areas(center);
        let mut add_button = Button::new("Add contact").colors(self.theme.button_colors());
        let mut text_box = TextBox::new()
            .block(
                Block::bordered()
                    .title("IST ID")
                    .style(self.theme.text_style()),
            )
            .placeholder(" e.g ist1112270")
            .placeholder_style(self.theme.subtext_stye())
            .style(self.theme.text_style());
        if state.active {
            match state.selected_element {
                SelectedElement::TextBox => {
                    text_box = text_box.cursor(Cursor::default().style(self.theme.text_style()))
                }
                SelectedElement::Button => add_button = add_button.state(ButtonState::Selected),
            }
        }
        match &state.result {
            SearchContactResult::None => (),
            SearchContactResult::Error(msg) => {
                let warn = Line::from(msg.to_string())
                    .centered()
                    .style(self.theme.error_style());
                warn.render(warn_area, buf);
            }
            SearchContactResult::Success(msg) => {
                let warn = Line::from(msg.to_string())
                    .centered()
                    .style(self.theme.success_style());
                warn.render(warn_area, buf);
            }
        }
        add_button.render(button, buf);
        text_box.render(input, buf, &mut state.text_box);
    }
}

impl AsyncStatefulEventHandler<Event> for SearchContactState {
    type State = App;

    async fn handle_event(&mut self, event: Event, app: &mut App) {
        if self.selected_element == SelectedElement::TextBox {
            self.text_box.handle_event(event.clone());
        }
        match event {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => self.selected_element = SelectedElement::TextBox,
                    KeyCode::Down => self.selected_element = SelectedElement::Button,
                    _ => {
                        if self.selected_element == SelectedElement::Button {
                            if event.code == KeyCode::Enter {
                                let id = &self.text_box.text;
                                if id.trim().is_empty() {
                                    self.result = SearchContactResult::Error(
                                        "IST ID cannot be empty!".to_string(),
                                    );
                                    return;
                                } else if let Some(user_id) = &app.current_user {
                                    if user_id.id.to_lowercase() == id.to_lowercase() {
                                        self.result = SearchContactResult::Error(
                                            "You cannot add yourself!".to_string(),
                                        );
                                        return;
                                    }
                                }
                                match app.net_client.get_user(id).await {
                                    Ok(response) => {
                                        match app
                                            .add_contact(
                                                &response.id,
                                                &response.name,
                                                &response.public_key,
                                            )
                                            .await
                                        {
                                            Ok(_) => {
                                                self.result = SearchContactResult::Success(
                                                    format!("Contact '{}' has ben added!", id),
                                                );
                                            }
                                            Err(_) => self.result = SearchContactResult::Error(
                                                "Failed to add contact, probably another exists?"
                                                    .to_string(),
                                            ),
                                        };
                                    }
                                    Err(e) => {
                                        if let Some(status) = e.status() {
                                            if status == StatusCode::NOT_FOUND {
                                                self.result = SearchContactResult::Error(format!(
                                                    "Contact with id '{}' not found",
                                                    id
                                                ));
                                                return;
                                            }
                                        }
                                        self.result = SearchContactResult::Error(format!("{}", e));
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
