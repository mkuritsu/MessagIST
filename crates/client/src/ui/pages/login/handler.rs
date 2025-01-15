use crossterm::event::{Event, KeyCode};

use crate::ui::event_handler::{EventHandler, StatefulEventHandler};

use super::state::{FocusedElement, LoginState};

pub struct LoginHandler;

impl LoginHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulEventHandler<Event> for LoginHandler {
    type State = LoginState;

    fn handle_event(&mut self, event: Event, state: &mut Self::State) {
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Down | KeyCode::Tab => {
                    state.next_focus();
                    return;
                }
                KeyCode::Up | KeyCode::BackTab => {
                    state.prev_focus();
                    return;
                }
                _ => (),
            }
        }
        match state.focused {
            FocusedElement::Username => state.username_text_box.handle_event(event),
            FocusedElement::Password => state.password_text_box.handle_event(event),
            _ => (),
        }
    }
}
