use crate::Event;
use model::Model;
use winit::event::WindowEvent;

pub mod model;

pub struct Menu {
    model: Model<Event>,
    tab: usize,
    page: usize,
}

impl Menu {
    pub fn new(model: Model<Event>) -> Self {
        let page = model.start_page;
        let tab = model.pages[page].start_tab;
        Self { model, page, tab }
    }

    pub fn window_event(&mut self, _event: WindowEvent) {
        todo!()
    }
}
