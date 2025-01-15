use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::App,
    ui::widgets::button::{Button, ButtonState},
};

use super::{state::FocusedElement, EntryPage, Layout};

const LOGO: &'static str = "
███╗░░░███╗███████╗░██████╗░██████╗░█████╗░░██████╗░██╗░██████╗████████╗
████╗░████║██╔════╝██╔════╝██╔════╝██╔══██╗██╔════╝░██║██╔════╝╚══██╔══╝
██╔████╔██║█████╗░░╚█████╗░╚█████╗░███████║██║░░██╗░██║╚█████╗░░░░██║░░░
██║╚██╔╝██║██╔══╝░░░╚═══██╗░╚═══██╗██╔══██║██║░░╚██╗██║░╚═══██╗░░░██║░░░
██║░╚═╝░██║███████╗██████╔╝██████╔╝██║░░██║╚██████╔╝██║██████╔╝░░░██║░░░
╚═╝░░░░░╚═╝╚══════╝╚═════╝░╚═════╝░╚═╝░░╚═╝░╚═════╝░╚═╝╚═════╝░░░░╚═╝░░░";

const FOOTER: &'static str = "Use ⭠ ⭢ to switch between options, ENTER to confirm, ESC to quit";

impl StatefulWidget for &mut EntryPage {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        app.just_registered = false;
        self.state.layout = Layout::new(area);
        let header = Paragraph::new(LOGO)
            .alignment(Alignment::Center)
            .style(app.theme.accent_style());
        let footer = Paragraph::new(FOOTER)
            .alignment(Alignment::Center)
            .style(app.theme.subtext_stye());
        let mut login = Button::new("Login").colors(app.theme.button_colors());
        let mut register = Button::new("Register").colors(app.theme.button_colors());
        match self.state.focused {
            FocusedElement::Login => login = login.state(ButtonState::Selected),
            FocusedElement::Register => register = register.state(ButtonState::Selected),
        }
        header.render(self.state.layout.header, buf);
        footer.render(self.state.layout.footer, buf);
        login.render(self.state.layout.login, buf);
        register.render(self.state.layout.register, buf);
    }
}
