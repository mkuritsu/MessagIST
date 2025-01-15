use crate::ui::widgets::text_box::TextBoxState;

mod handler;
mod widget;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FocusedElement {
    TextBox,
    Button,
}

pub struct ConnectState {
    text_box_state: TextBoxState,
    conn_result: Option<String>,
    selected_element: FocusedElement,
}

impl Default for ConnectState {
    fn default() -> Self {
        Self {
            text_box_state: Default::default(),
            conn_result: None,
            selected_element: FocusedElement::TextBox,
        }
    }
}

pub struct ConnectPage {
    state: ConnectState,
}

impl ConnectPage {
    pub fn new() -> Self {
        Self {
            state: ConnectState::default(),
        }
    }
}
