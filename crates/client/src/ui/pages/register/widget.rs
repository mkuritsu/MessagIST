use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::App,
    ui::widgets::{
        button::{Button, ButtonState},
        cursor::Cursor,
        text_box::TextBox,
    },
};

use super::{
    state::{FocusedElement, RegisterResult, RegisterState},
    Layout, RegisterPage,
};

const HEADER: &str = "██████╗ ███████╗ ██████╗ ██╗███████╗████████╗███████╗██████╗
██╔══██╗██╔════╝██╔════╝ ██║██╔════╝╚══██╔══╝██╔════╝██╔══██╗
██████╔╝█████╗  ██║  ███╗██║███████╗   ██║   █████╗  ██████╔╝
██╔══██╗██╔══╝  ██║   ██║██║╚════██║   ██║   ██╔══╝  ██╔══██╗
██║  ██║███████╗╚██████╔╝██║███████║   ██║   ███████╗██║  ██║
╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝";

const FOOTER: &str =
    "Use TAB to switch between fields, ENTER to confirm, ESC to return to previous page";

impl StatefulWidget for &mut RegisterPage {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        self.state.layout = Layout::new(area);
        let header = Paragraph::new(HEADER)
            .alignment(Alignment::Center)
            .style(app.theme.accent_style());
        let footer = Paragraph::new(FOOTER)
            .alignment(Alignment::Center)
            .style(app.theme.subtext_stye());
        if let Some(notification) = create_notificaton(&self.state.register_result, app) {
            notification.render(self.state.layout.notification, buf);
        }
        let (id, name, password, re_password, submit) = create_field_widgets(&self.state, app);
        header.render(self.state.layout.header, buf);
        footer.render(self.state.layout.footer, buf);
        submit.render(self.state.layout.submit, buf);
        id.render(self.state.layout.username, buf, &mut self.state.id_text_box);
        name.render(self.state.layout.name, buf, &mut self.state.name_text_box);
        password.render(
            self.state.layout.password,
            buf,
            &mut self.state.password_text_box,
        );
        re_password.render(
            self.state.layout.re_password,
            buf,
            &mut self.state.re_password_text_box,
        );
    }
}

fn create_notificaton<'a>(result: &RegisterResult, app: &App) -> Option<Paragraph<'a>> {
    let error_style = app.theme.error_style();
    let (text, style) = match result {
        RegisterResult::None => return None,
        RegisterResult::Error(msg) => (msg, error_style),
    };
    Some(Paragraph::new(text.clone()).style(style).centered())
}

fn create_field_widgets<'a>(
    state: &RegisterState,
    app: &App,
) -> (
    TextBox<'a>,
    TextBox<'a>,
    TextBox<'a>,
    TextBox<'a>,
    Button<'a>,
) {
    let base_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(app.theme.text_style());
    let id_block = base_block.clone().title("IST ID");
    let name_block = base_block.clone().title("Name");
    let password_block = base_block.clone().title("Password");
    let re_password_block = base_block.title("Confirm Password");
    let mut id = TextBox::new()
        .block(id_block.clone())
        .style(app.theme.text_style());
    let mut name = TextBox::new()
        .block(name_block.clone())
        .style(app.theme.text_style());
    let mut password = TextBox::new()
        .censored()
        .block(password_block.clone())
        .style(app.theme.text_style());
    let mut re_password = TextBox::new()
        .censored()
        .block(re_password_block.clone())
        .style(app.theme.text_style());
    let mut submit = Button::new("Submit").colors(app.theme.button_colors());
    let cursor = Cursor::default().style(app.theme.text_style());
    match state.focused {
        FocusedElement::Username => {
            id = id
                .cursor(cursor)
                .block(id_block.style(app.theme.accent_style()))
        }
        FocusedElement::Name => {
            name = name
                .cursor(cursor)
                .block(name_block.style(app.theme.accent_style()))
        }
        FocusedElement::Password => {
            password = password
                .cursor(cursor)
                .block(password_block.style(app.theme.accent_style()))
        }
        FocusedElement::ConfirmPassword => {
            re_password = re_password
                .cursor(cursor)
                .block(re_password_block.style(app.theme.accent_style()))
        }
        FocusedElement::Submit => submit = submit.state(ButtonState::Selected),
    }
    (id, name, password, re_password, submit)
}
