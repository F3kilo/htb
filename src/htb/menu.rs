use crate::Event;
use std::collections::HashMap;
use crate::htb::view::menu::model::{Button, Control, ControlType, Filling, Model, Page, Tab};

pub fn new_main_menu() -> Model<Event> {
    let controls = vec![
        Control {
            tab: 0,
            group: None,
            typ: ControlType::Button(Button {
                filling: Filling::Text("Start".into()),
            }),
            handlers: HashMap::from([]),
        },
        Control {
            tab: 0,
            group: None,
            typ: ControlType::Button(Button {
                filling: Filling::Text("Exit".into()),
            }),
            handlers: HashMap::from([]),
        },
    ];

    let tab = Tab {
        name: "Main menu".into(),
    };

    let main_page = Page {
        tabs: vec![tab],
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
