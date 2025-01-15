use base64::Engine;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::Line,
    widgets::{Block, BorderType, StatefulWidget, Widget},
};
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

#[derive(Clone, PartialEq, Eq)]
pub enum AddManualResult {
    None,
    Error(String),
    Success(String),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SelectedElement {
    Id,
    Name,
    Key,
    Button,
}

pub struct ManualContactState {
    id_text_box: TextBoxState,
    name_text_box: TextBoxState,
    key_text_box: TextBoxState,
    selected_element: SelectedElement,
    result: AddManualResult,
    pub active: bool,
}

impl ManualContactState {
    pub fn select_next(&mut self) {
        self.selected_element = match self.selected_element {
            SelectedElement::Id => SelectedElement::Name,
            SelectedElement::Name => SelectedElement::Key,
            SelectedElement::Key => SelectedElement::Button,
            SelectedElement::Button => SelectedElement::Button,
        }
    }

    pub fn select_prev(&mut self) {
        self.selected_element = match self.selected_element {
            SelectedElement::Id => SelectedElement::Id,
            SelectedElement::Name => SelectedElement::Id,
            SelectedElement::Key => SelectedElement::Name,
            SelectedElement::Button => SelectedElement::Key,
        }
    }
}

impl Default for ManualContactState {
    fn default() -> Self {
        Self {
            id_text_box: Default::default(),
            name_text_box: Default::default(),
            key_text_box: Default::default(),
            selected_element: SelectedElement::Id,
            result: AddManualResult::None,
            active: false,
        }
    }
}

pub struct ManualContactWidget<'a> {
    block: Option<Block<'a>>,
    theme: &'a AppTheme,
}

impl<'a> ManualContactWidget<'a> {
    pub fn new(theme: &'a AppTheme) -> Self {
        Self { block: None, theme }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for ManualContactWidget<'a> {
    type State = ManualContactState;

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
        let title = TitleWidget::new("ADD MANUALLY")
            .block(
                Block::bordered()
                    .border_type(BorderType::Thick)
                    .style(self.theme.text_style().add_modifier(Modifier::BOLD)),
            )
            .style(self.theme.text_style().add_modifier(Modifier::BOLD));
        title.render(header, buf);
        let center = body.inner_centered(body.width - body.width / 5, 20);
        let [warn_area, _, id_area, _, name_area, _, key_area, _, add_button] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
        ])
        .areas(center);
        let mut id = TextBox::new()
            .block(
                Block::bordered()
                    .title("IST ID")
                    .style(self.theme.text_style()),
            )
            .placeholder(" e.g ist1112270")
            .placeholder_style(self.theme.subtext_stye())
            .style(self.theme.text_style());
        let mut name = TextBox::new()
            .block(
                Block::bordered()
                    .title("Name")
                    .style(self.theme.text_style()),
            )
            .placeholder(" e.g Laura Cunha")
            .placeholder_style(self.theme.subtext_stye())
            .style(self.theme.text_style());
        let mut key = TextBox::new()
            .block(
                Block::bordered()
                    .title("Public key")
                    .style(self.theme.text_style()),
            )
            .placeholder(" base64 encoded rsa public key")
            .placeholder_style(self.theme.subtext_stye())
            .style(self.theme.text_style());
        let mut add = Button::new("Add contact").colors(self.theme.button_colors());
        if state.active {
            match state.selected_element {
                SelectedElement::Id => {
                    id = id.cursor(Cursor::default().style(self.theme.text_style()))
                }
                SelectedElement::Name => {
                    name = name.cursor(Cursor::default().style(self.theme.text_style()))
                }
                SelectedElement::Key => {
                    key = key.cursor(Cursor::default().style(self.theme.text_style()))
                }
                SelectedElement::Button => add = add.state(ButtonState::Selected),
            }
        }
        match &state.result {
            AddManualResult::None => (),
            AddManualResult::Error(msg) | AddManualResult::Success(msg) => {
                let warn = Line::from(msg.to_string())
                    .centered()
                    .style(self.theme.error_style());
                warn.render(warn_area, buf);
            }
        }
        id.render(id_area, buf, &mut state.id_text_box);
        name.render(name_area, buf, &mut state.name_text_box);
        key.render(key_area, buf, &mut state.key_text_box);
        add.render(add_button, buf);
    }
}

impl AsyncStatefulEventHandler<Event> for ManualContactState {
    type State = App;

    async fn handle_event(&mut self, event: Event, app: &mut App) {
        match self.selected_element {
            SelectedElement::Id => self.id_text_box.handle_event(event.clone()),
            SelectedElement::Name => self.name_text_box.handle_event(event.clone()),
            SelectedElement::Key => self.key_text_box.handle_event(event.clone()),
            _ => (),
        }
        match event {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => self.select_prev(),
                    KeyCode::Down => self.select_next(),
                    _ => {
                        if self.selected_element == SelectedElement::Button {
                            if event.code == KeyCode::Enter {
                                let id = &self.id_text_box.text;
                                let name = &self.name_text_box.text;
                                let public_key = &self.key_text_box.text;
                                let error_messages = [
                                    "IST ID cannot be empty",
                                    "Name cannot be empty",
                                    "Public key cannot be empty",
                                ];
                                let fields = [id, name, public_key];
                                if let Some((_, e)) = fields
                                    .iter()
                                    .zip(error_messages)
                                    .filter(|(f, _)| f.trim().is_empty())
                                    .next()
                                {
                                    self.result = AddManualResult::Error(e.to_string());
                                    return;
                                }
                                if let Some(current_user) = &app.current_user {
                                    if current_user.id.to_lowercase() == id.to_lowercase() {
                                        self.result = AddManualResult::Error(
                                            "You cannot add yourself!".to_string(),
                                        );
                                        return;
                                    }
                                }
                                let public_key = match base64::engine::general_purpose::STANDARD
                                    .decode(public_key)
                                {
                                    Ok(v) => v,
                                    Err(_) => {
                                        self.result = AddManualResult::Error(
                                            "Non valid base64 public key".to_string(),
                                        );
                                        return;
                                    }
                                };
                                match app.add_contact(id, name, &public_key).await {
                                Ok(_) => {
                                    self.result = AddManualResult::Success(format!(
                                        "Contact {} has been added!",
                                        id
                                    ))
                                }
                                Err(_) => self.result = AddManualResult::Error("Couldn't add contact to contacts, probably another exists?".to_string()),
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
