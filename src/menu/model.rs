use std::collections::HashMap;

use super::MenuEvent;

pub struct Control<E> {
    pub typ: ControlType,
    pub handlers: HashMap<Action, E>,
}

impl<E: MenuEvent> Control<E> {
    pub fn action(&self, action: Action) {
        if let Some(event) = self.handlers.get(&action) {
            event.send()
        }
    }
}

pub enum ControlType {
    Button(Button),
}

pub struct Button {
    pub text: String,
    pub enabled: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Action {
    Click,
}
