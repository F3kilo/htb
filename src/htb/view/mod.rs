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
    game: Game,
}

impl View {
    pub fn with_menu(menu_model: Model<Event>) -> Self {
        Self {
            menu: Some(Menu::new(menu_model)),
            game: Game::default(),
        }
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        if let Some(menu) = &mut self.menu {
            menu.window_event(event)
        }
    }

    pub fn device_event(&mut self, _event: DeviceEvent) {
       // todo!()
    }

    pub fn draw(&self, gfx: &impl Gfx) {
        if let Some(menu) = &self.menu {
            menu.render(gfx);
        } else {
            self.game.render(gfx);
        }

    }
}

pub trait Gfx {
    fn draw_rect(&self, top_left: glam::Vec2, bot_right: glam::Vec2);
    fn present(&self);
}