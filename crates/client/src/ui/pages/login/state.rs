use crate::ui::widgets::text_box::TextBoxState;

use super::Layout;

#[derive(PartialEq)]
pub enum FocusedElement {
    Username,
    Password,
    Submit,
}

#[derive(Clone, Copy)]
pub enum LoginResult {
    None,
    UsernameEmpty,
    PasswordEmpty,
    UserNotFound,
    WrongPassword,
    Error,
}

pub struct LoginState {
    pub username_text_box: TextBoxState,
    pub password_text_box: TextBoxState,
    pub layout: Layout,
    pub focused: FocusedElement,
    pub login_result: LoginResult,
}

impl Default for LoginState {
    fn default() -> Self {
        Self {
            username_text_box: TextBoxState::default(),
            password_text_box: TextBoxState::default(),
            layout: Layout::default(),
            focused: FocusedElement::Username,
            login_result: LoginResult::None,
        }
    }
}

impl LoginState {
    pub fn next_focus(&mut self) {
        self.focused = match self.focused {
            FocusedElement::Username => FocusedElement::Password,
            FocusedElement::Password => FocusedElement::Submit,
            FocusedElement::Submit => FocusedElement::Submit,
        }
    }

    pub fn prev_focus(&mut self) {
        self.focused = match self.focused {
            FocusedElement::Username => FocusedElement::Username,
            FocusedElement::Password => FocusedElement::Username,
            FocusedElement::Submit => FocusedElement::Password,
        }
    }
}
