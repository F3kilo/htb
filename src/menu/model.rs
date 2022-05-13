use std::collections::HashMap;

pub struct Control<Event> {
    pub typ: ControlType,
    pub handlers: HashMap<Action, Event>,
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