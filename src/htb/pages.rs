use crate::menu::{model, Page};
use crate::Event;
use std::collections::HashMap;

pub fn main_menu() -> Page<Event> {
    let controls = vec![
        model::Control {
            enabled: true,
            typ: model::ControlType::Button(model::Button {
                text: String::from("Start game"),
            }),
            handlers: HashMap::from([(model::Action::Click, Event::StartGame)]),
        },
        model::Control {
            enabled: true,
            typ: model::ControlType::Button(model::Button {
                text: String::from("Exit"),
            }),
            handlers: HashMap::from([(model::Action::Click, Event::Exit)]),
        },
    ];

    Page::new(controls)
}
