use base64::Engine as _;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Tabs, Widget},
};
use tachyonfx::CenteredShrink;

use crate::{app::App, ui::widgets::messagist_text::MessageISTText};

use super::{
    logout_popup::LogoutPopup,
    profile_popup::ProfilePopup,
    state::{SelectedTab, ShowingPopup},
    tabs::{contacts::ContactsTab, messages::MessagesTab},
    Layout, MainPage,
};

impl StatefulWidget for &mut MainPage {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, app: &mut Self::State) {
        let layout = Layout::new(area);
        let title = MessageISTText::new();
        let footer = Paragraph::new("Use TAB to cycle between tabs, arrow keys for navigation, '+' to add a contact, 'u' to check your profile.")
            .centered()
            .style(app.theme.subtext_stye());
        title.render(layout.title, buf);
        footer.render(layout.footer, buf);
        let block = Block::bordered().border_type(BorderType::Thick);
        let inner = block.inner(layout.body);
        block.render(layout.body, buf);
        let mut tabs = Tabs::new(["  Messages  ", "  Contacts  "])
            .padding("  ", "  ")
            .divider("<->")
            .style(app.theme.text_style())
            .highlight_style(app.theme.accent_style().add_modifier(Modifier::BOLD));
        match self.state.selected_tab {
            SelectedTab::Messages(_) => {
                let widget = MessagesTab::new(app);
                widget.render(inner, buf, &mut self.state.messages_state);
                tabs = tabs.select(0);
            }
            SelectedTab::Contacts => {
                let widget = ContactsTab::new(app);
                widget.render(inner, buf, &mut self.state.contacts_state);
                tabs = tabs.select(1);
            }
        };
        match &self.state.poup {
            ShowingPopup::None => (),
            ShowingPopup::Logout(state) => {
                let mut state = state.clone();
                let area = area.inner_centered(area.width / 5, area.height / 5);
                let widget = LogoutPopup::new(&app.theme);
                widget.render(area, buf, &mut state);
                self.state.poup = ShowingPopup::Logout(state);
            }
            ShowingPopup::Profile(state) => {
                let mut state = state.clone();
                let area = area.inner_centered(area.width / 3, area.height - area.height / 10);
                let user = app.current_user.as_ref().expect("No user");

                let pub_key_bytes = cryptolib::utils::public_key_to_bytes(user.public_key.clone())
                    .expect("Failed to transform key to bytes");
                let pub_key = base64::engine::general_purpose::STANDARD.encode(&pub_key_bytes);

                let widget = ProfilePopup::new(&user.name, &user.id, &pub_key, &app.theme);
                widget.render(area, buf, &mut state);
                self.state.poup = ShowingPopup::Profile(state);
            }
        }
        tabs.render(layout.tabs, buf);
    }
}
