use crate::menu::{Page, model};
use crate::Event;
use std::collections::HashMap;

pub fn main_menu() -> Page<Event> {
    let controls = vec![
        model::Control {
            typ: model::ControlType::Button(model::Button {
                enabled: true,
                text: String::from("Start game"),
            }),
            handlers: HashMap::from([]),
        },
        model::Control {
            typ: model::ControlType::Button(model::Button {
                enabled: true,
                text: String::from("Exit"),
            }),
            handlers: HashMap::from([]),
        },
    ];

    Page {
        controls,
        focused: Some(0)
    }
}
