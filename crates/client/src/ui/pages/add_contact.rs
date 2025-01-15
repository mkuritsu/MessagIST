use crossterm::event::{Event, KeyCode};
use manual::{ManualContactState, ManualContactWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, StatefulWidget, Widget},
};
use search::{SearchContactState, SearchContactWidget};
use tachyonfx::{fx, CenteredShrink, Duration, Effect, EffectTimer, Interpolation, Motion, Shader};

use crate::{
    app::{App, AppEvent, Pages},
    ui::event_handler::AsyncStatefulEventHandler,
};

mod manual;
mod search;

pub enum SelectedOption {
    Search,
    Manual,
}

pub struct AddContactState {
    selected_option: SelectedOption,
    search_state: SearchContactState,
    manual_state: ManualContactState,
    slide_effect: Effect,
}

impl AddContactState {
    pub fn new(color: Color) -> Self {
        let slide_effect = fx::slide_in(
            Motion::UpToDown,
            0,
            0,
            color,
            EffectTimer::new(Duration::from_millis(300), Interpolation::QuadIn),
        );
        Self {
            selected_option: SelectedOption::Search,
            search_state: SearchContactState::default(),
            manual_state: ManualContactState::default(),
            slide_effect,
        }
    }
}

pub struct AddContactPage {
    state: AddContactState,
}

impl AddContactPage {
    pub fn new(color: Color) -> Self {
        Self {
            state: AddContactState::new(color),
        }
    }
}

impl StatefulWidget for &mut AddContactPage {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut App) {
        let [body, _, footer, _] = Layout::vertical([
            Constraint::Percentage(100),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .areas(area);
        let [left, right] = Layout::horizontal([Constraint::Percentage(50); 2]).areas(body);
        let mut search = SearchContactWidget::new(&app.theme).block(
            Block::bordered()
                .border_type(BorderType::Thick)
                .style(app.theme.text_style()),
        );
        let mut manual = ManualContactWidget::new(&app.theme).block(
            Block::bordered()
                .border_type(BorderType::Thick)
                .style(app.theme.text_style()),
        );
        let selected_block = Block::bordered()
            .border_type(BorderType::Thick)
            .style(app.theme.accent_style());
        match self.state.selected_option {
            SelectedOption::Search => {
                search = search.block(selected_block);
                self.state.search_state.active = true;
                self.state.manual_state.active = false;
            }
            SelectedOption::Manual => {
                manual = manual.block(selected_block);
                self.state.search_state.active = false;
                self.state.manual_state.active = true;
            }
        }
        search.render(left, buf, &mut self.state.search_state);
        manual.render(right, buf, &mut self.state.manual_state);
        let help = Line::from("TAB to cycle, arrow keys for navigation and ESC to go back")
            .centered()
            .style(app.theme.subtext_stye());
        help.render(footer, buf);
        let mut middle = body.inner_centered(4, 3);
        if body.width % 2 > 0 {
            middle.x += 1;
        }
        let block = Block::bordered()
            .border_type(BorderType::Thick)
            .style(app.theme.text_style());
        let inner = block.inner(middle);
        block.render(middle, buf);
        let middle_text = Line::from("OR").centered().style(app.theme.text_style());
        middle_text.render(inner, buf);
        self.state
            .slide_effect
            .process(app.frame_duration.into(), buf, area);
    }
}

impl AsyncStatefulEventHandler<AppEvent> for AddContactPage {
    type State = App;

    async fn handle_event(&mut self, event: AppEvent, app: &mut Self::State) {
        if let AppEvent::Input(event) = event {
            if let Event::Key(event) = event {
                if event.code == KeyCode::Esc {
                    app.current_page = Pages::Main;
                    return;
                }
                if event.code == KeyCode::Tab || event.code == KeyCode::BackTab {
                    self.state.selected_option = match self.state.selected_option {
                        SelectedOption::Search => SelectedOption::Manual,
                        SelectedOption::Manual => SelectedOption::Search,
                    };
                    return;
                }
            }
            match self.state.selected_option {
                SelectedOption::Search => self.state.search_state.handle_event(event, app).await,
                SelectedOption::Manual => self.state.manual_state.handle_event(event, app).await,
            }
        }
    }
}
