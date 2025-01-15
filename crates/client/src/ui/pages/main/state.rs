use std::cmp;

use crate::db::structs::Contact;

use super::{
    logout_popup::LogoutPopupState,
    profile_popup::ProfilePopupState,
    tabs::{contacts::ContactsTabState, messages::MessagesTabState},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SelectedTab {
    Messages(MessagesTab),
    Contacts,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MessagesTab {
    Chatlist,
    ChatMessages,
}

#[derive(Clone)]
pub enum ShowingPopup {
    None,
    Logout(LogoutPopupState),
    Profile(ProfilePopupState),
}

pub struct MainState {
    pub selected_tab: SelectedTab,
    pub contacts_state: ContactsTabState,
    pub messages_state: MessagesTabState,
    pub poup: ShowingPopup,
}

impl MainState {
    pub fn switch_tab(&mut self) {
        self.selected_tab = match self.selected_tab {
            SelectedTab::Messages(_) => SelectedTab::Contacts,
            SelectedTab::Contacts => SelectedTab::Messages(MessagesTab::Chatlist),
        };
    }

    pub fn switch_msglist(&mut self) {
        match self.selected_tab {
            SelectedTab::Messages(MessagesTab::ChatMessages) => {
                self.selected_tab = SelectedTab::Messages(MessagesTab::Chatlist);
            }
            _ => (),
        }
    }

    pub fn switch_msgchat(&mut self) {
        match self.selected_tab {
            SelectedTab::Messages(MessagesTab::Chatlist) => {
                self.selected_tab = SelectedTab::Messages(MessagesTab::ChatMessages);
            }
            _ => (),
        }
    }
}

impl MainState {
    pub fn new(contacts: Vec<Contact>, chats: Vec<String>) -> Self {
        let n_contacts = cmp::max(contacts.len(), 1);
        Self {
            selected_tab: SelectedTab::Messages(MessagesTab::Chatlist),
            contacts_state: ContactsTabState::new(n_contacts),
            messages_state: MessagesTabState::new(chats.len()),
            poup: ShowingPopup::None,
        }
    }
}
