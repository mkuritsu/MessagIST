use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, BorderType, StatefulWidget, Widget},
};
use tachyonfx::CenteredShrink;

use crate::{
    app::App,
    ui::widgets::{
        button::{Button, ButtonState},
        cursor::Cursor,
        text_box::TextBox,
    },
};

use super::{ConnectPage, FocusedElement};

const FOOTER: &'static str = "Use ⭡ ⭣ to switch between fields, ENTER to confirm, ESC to quit";

impl StatefulWidget for &mut ConnectPage {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        let [warn, _, tbox_area, _, btn_area, _, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .areas(area.inner_centered(area.width / 2, 13));
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .style(app.theme.text_style())
            .title("Server Address");
        let mut text_box = TextBox::new()
            .block(block.clone())
            .style(app.theme.text_style())
            .placeholder(" https://localhost:8000")
            .placeholder_style(app.theme.subtext_stye());
        let mut button = Button::new("Connect").colors(app.theme.button_colors());
        match self.state.selected_element {
            FocusedElement::TextBox => {
                text_box = text_box
                    .cursor(Cursor::default().style(app.theme.text_style()))
                    .block(block.clone().style(app.theme.accent_style()))
            }
            FocusedElement::Button => button = button.state(ButtonState::Selected),
        }
        if let Some(msg) = &self.state.conn_result {
            let line = Line::from(msg.clone())
                .centered()
                .style(app.theme.error_style());
            line.render(warn, buf);
            text_box = text_box.block(block.style(app.theme.error_style()))
        }
        let footer = Line::from(FOOTER)
            .centered()
            .style(app.theme.subtext_stye());
        footer.render(footer_area, buf);
        text_box.render(tbox_area, buf, &mut self.state.text_box_state);
        button.render(btn_area, buf);
    }
}
