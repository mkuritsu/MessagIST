use crate::ui::widgets::text_box::TextBoxState;

use super::Layout;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FocusedElement {
    Username,
    Name,
    Password,
    ConfirmPassword,
    Submit,
}

#[derive(Clone)]
pub enum RegisterResult {
    None,
    Error(String),
}

pub struct RegisterState {
    pub id_text_box: TextBoxState,
    pub name_text_box: TextBoxState,
    pub password_text_box: TextBoxState,
    pub re_password_text_box: TextBoxState,
    pub layout: Layout,
    pub focused: FocusedElement,
    pub register_result: RegisterResult,
}

impl Default for RegisterState {
    fn default() -> Self {
        Self {
            id_text_box: TextBoxState::default(),
            name_text_box: TextBoxState::default(),
            password_text_box: TextBoxState::default(),
            re_password_text_box: TextBoxState::default(),
            layout: Layout::default(),
            focused: FocusedElement::Username,
            register_result: RegisterResult::None,
        }
    }
}

impl RegisterState {
    pub fn next_focus(&mut self) {
        self.focused = match self.focused {
            FocusedElement::Username => FocusedElement::Name,
            FocusedElement::Name => FocusedElement::Password,
            FocusedElement::Password => FocusedElement::ConfirmPassword,
            FocusedElement::ConfirmPassword => FocusedElement::Submit,
            FocusedElement::Submit => FocusedElement::Submit,
        }
    }

    pub fn prev_focus(&mut self) {
        self.focused = match self.focused {
            FocusedElement::Username => FocusedElement::Username,
            FocusedElement::Name => FocusedElement::Username,
            FocusedElement::Password => FocusedElement::Name,
            FocusedElement::ConfirmPassword => FocusedElement::Password,
            FocusedElement::Submit => FocusedElement::ConfirmPassword,
        }
    }
}
