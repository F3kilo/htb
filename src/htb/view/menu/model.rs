use std::collections::HashMap;

#[derive(Default)]
pub struct Model<Event> {
    pub pages: Vec<Page<Event>>,
    pub start_page: usize,
}

pub struct Page<Event> {
    pub tabs: Vec<Tab>,
    pub start_tab: usize,
    pub groups: Vec<Group>,
    pub controls: Vec<Control<Event>>,
}

pub struct Tab {
    pub name: String,
}

pub struct Group {
    pub name: String,
}

pub struct Control<Event> {
    pub tab: usize,
    pub group: Option<usize>,
    pub typ: ControlType,
    pub handlers: HashMap<Action, Event>,
}

pub enum ControlType {
    Button(Button),
}

pub struct Button {
    pub filling: Filling,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Action {
    Click,
}

pub enum Filling {
    Text(String),
    Image(String),
}
