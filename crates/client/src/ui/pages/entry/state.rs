use super::Layout;

pub enum FocusedElement {
    Login,
    Register,
}

pub struct EntryState {
    pub layout: Layout,
    pub focused: FocusedElement,
}

impl Default for EntryState {
    fn default() -> Self {
        Self {
            layout: Layout::default(),
            focused: FocusedElement::Login,
        }
    }
}
