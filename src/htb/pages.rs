use crate::menu::{model, Page};
use crate::Event;
use std::collections::HashMap;

pub fn main_menu() -> Page<Event> {
    let controls = vec![
        model::Control {
            typ: model::ControlType::Button(model::Button {
                enabled: true,
                text: String::from("Start game"),
            }),
            handlers: HashMap::from([(model::Action::Click, Event::StartGame)]),
        },
        model::Control {
            typ: model::ControlType::Button(model::Button {
                enabled: true,
                text: String::from("Exit"),
            }),
            handlers: HashMap::from([(model::Action::Click, Event::Exit)]),
        },
    ];

    Page::new(controls)
}
