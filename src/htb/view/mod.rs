use super::event::Event;
use crate::htb::view::menu::model::Model;
use game::Game;
use menu::Menu;
use winit::event::{DeviceEvent, WindowEvent};

mod game;
pub mod menu;

#[derive(Default)]
pub struct View {
    menu: Option<Menu>,
    _game: Game,
}

impl View {
    pub fn with_menu(menu_model: Model<Event>) -> Self {
        Self {
            menu: Some(Menu::new(menu_model)),
            _game: Game::default(),
        }
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        if let Some(menu) = &mut self.menu {
            menu.window_event(event)
        }
    }

    pub fn device_event(&mut self, _event: DeviceEvent) {
        todo!()
    }
}
