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
    state::{FocusedElement, LoginResult, LoginState},
    Layout, LoginPage,
};

const HEADER: &'static str = "██╗      ██████╗  ██████╗ ██╗███╗   ██╗
██║     ██╔═══██╗██╔════╝ ██║████╗  ██║
██║     ██║   ██║██║  ███╗██║██╔██╗ ██║
██║     ██║   ██║██║   ██║██║██║╚██╗██║
███████╗╚██████╔╝╚██████╔╝██║██║ ╚████║
╚══════╝ ╚═════╝  ╚═════╝ ╚═╝╚═╝  ╚═══╝";

const FOOTER: &'static str =
    "Use ⭡ ⭣ to switch between fields, ENTER to confirm, ESC to return to previous page";

impl StatefulWidget for &mut LoginPage {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        self.state.layout = Layout::new(area);
        let header = Paragraph::new(HEADER)
            .alignment(Alignment::Center)
            .style(app.theme.accent_style());
        let footer = Paragraph::new(FOOTER)
            .alignment(Alignment::Center)
            .style(app.theme.subtext_stye());
        if let Some(notification) = create_notificaton(self.state.login_result, app) {
            notification.render(self.state.layout.notification, buf);
        }
        let (username, password, submit) = create_field_widgets(&self.state, app);
        header.render(self.state.layout.header, buf);
        footer.render(self.state.layout.footer, buf);
        submit.render(self.state.layout.submit, buf);
        username.render(
            self.state.layout.username,
            buf,
            &mut self.state.username_text_box,
        );
        password.render(
            self.state.layout.password,
            buf,
            &mut self.state.password_text_box,
        );
    }
}

fn create_notificaton<'a>(result: LoginResult, app: &mut App) -> Option<Paragraph<'a>> {
    let error_style = app.theme.error_style();
    let (text, style) = match result {
        LoginResult::None => {
            if app.just_registered {
                (
                    "Successfully registered, you can now login!",
                    app.theme.success_style(),
                )
            } else {
                return None;
            }
        }
        LoginResult::UsernameEmpty => ("IST ID is required!", error_style),
        LoginResult::PasswordEmpty => ("Password is required!", error_style),
        LoginResult::UserNotFound => ("IST ID not registered!", error_style),
        LoginResult::WrongPassword => ("Wrong password", error_style),
        LoginResult::Error => ("An error ocurred processing your request!", error_style),
    };
    Some(Paragraph::new(text).style(style).centered())
}

fn create_field_widgets<'a>(
    state: &LoginState,
    app: &mut App,
) -> (TextBox<'a>, TextBox<'a>, Button<'a>) {
    let base_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(app.theme.text_style());
    let id_block = base_block.clone().title("IST ID");
    let password_block = base_block.title("Password");
    let mut submit = Button::new("Submit").colors(app.theme.button_colors());
    let mut username = TextBox::new()
        .style(app.theme.text_style())
        .block(id_block.clone());
    let mut password = TextBox::new()
        .censored()
        .style(app.theme.text_style())
        .block(password_block.clone());
    match state.focused {
        FocusedElement::Username => {
            username = username
                .cursor(Cursor::default().style(app.theme.text_style()))
                .block(id_block.style(app.theme.accent_style()))
        }
        FocusedElement::Password => {
            password = password
                .cursor(Cursor::default().style(app.theme.text_style()))
                .block(password_block.style(app.theme.accent_style()))
        }
        FocusedElement::Submit => submit = submit.state(ButtonState::Selected),
    }
    (username, password, submit)
}
