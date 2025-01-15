use crossterm::event::{Event, KeyCode};

use crate::{
    app::{App, AppEvent, Pages},
    ui::event_handler::AsyncStatefulEventHandler,
};

use super::{state::FocusedElement, EntryPage};

impl AsyncStatefulEventHandler<AppEvent> for EntryPage {
    type State = App;

    async fn handle_event(&mut self, event: AppEvent, app: &mut Self::State) {
        if let AppEvent::Input(event) = event {
            match event {
                Event::Key(event) => match event.code {
                    KeyCode::Enter => match self.state.focused {
                        FocusedElement::Login => app.current_page = Pages::Login,
                        FocusedElement::Register => app.current_page = Pages::Register,
                    },
                    KeyCode::Esc => app.should_quit = true,
                    KeyCode::Left => self.state.focused = FocusedElement::Login,
                    KeyCode::Right => self.state.focused = FocusedElement::Register,
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
