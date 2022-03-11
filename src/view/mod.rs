use crate::view::game::Game;
use crate::view::menu::Menu;
use winit::event::{DeviceEvent, WindowEvent};

mod game;
mod menu;

#[derive(Default)]
pub struct View {
    menu: Option<Menu>,
    game: Game,
}

impl View {
    pub fn window_event(&mut self, event: WindowEvent) {
        todo!()
    }

    pub fn device_event(&mut self, event: DeviceEvent) {
        todo!()
    }
}
