use crate::menu::model;
use crate::Event;
use std::collections::HashMap;

pub fn new_main_menu() -> model::Model<Event> {
    let controls = vec![
        model::Control {
            tab: 0,
            group: None,
            typ: model::ControlType::Button(model::Button {
                filling: model::Filling::Text("Start".into()),
            }),
            handlers: HashMap::from([]),
        },
        model::Control {
            tab: 0,
            group: None,
            typ: model::ControlType::Button(model::Button {
                filling: model::Filling::Text("Exit".into()),
            }),
            handlers: HashMap::from([]),
        },
    ];

    let main_page = model::Page {
        start_tab: 0,
        groups: vec![],
        controls,
    };

    let pages = vec![main_page];

    Model {
        pages,
        start_page: 0,
    }
}
