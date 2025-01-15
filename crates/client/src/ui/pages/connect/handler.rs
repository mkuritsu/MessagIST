use crossterm::event::{Event, KeyCode};

use crate::{
    app::{App, AppEvent, Pages},
    ui::event_handler::{AsyncStatefulEventHandler, EventHandler},
};

use super::{ConnectPage, FocusedElement};

impl AsyncStatefulEventHandler<AppEvent> for ConnectPage {
    type State = App;

    async fn handle_event(&mut self, event: AppEvent, app: &mut Self::State) {
        if let AppEvent::Input(event) = event {
            if self.state.selected_element == FocusedElement::TextBox {
                self.state.text_box_state.handle_event(event.clone());
                if let Event::Key(event) = event {
                    if let KeyCode::Char(_) = event.code {
                        self.state.conn_result = None
                    }
                }
            }
            match event {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => app.should_quit = true,
                    KeyCode::Up => self.state.selected_element = FocusedElement::TextBox,
                    KeyCode::Down => self.state.selected_element = FocusedElement::Button,
                    KeyCode::Enter => {
                        if self.state.selected_element == FocusedElement::Button {
                            let url = &self.state.text_box_state.text;
                            match app.net_client.connect(url).await {
                                Ok(_) => app.current_page = Pages::Entry,
                                Err(e) => {
                                    self.state.conn_result =
                                        Some(format!("Error connecting to the server: {}", e))
                                }
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
