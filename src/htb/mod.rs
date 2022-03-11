use crate::view::View;
use winit::event::{DeviceEvent, WindowEvent};
use winit::window::{Fullscreen, Window};
use crate::Event;

pub mod event;

pub struct App {
    settings: Settings,
    window: Window,
    view: View,
}

impl App {
    pub fn new(settings: Settings, window: Window) -> Self {
        Self {
            settings,
            window,
            view: View::default(),
        }
    }

    pub fn app_event(&mut self, event: Event) {
        todo!()
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        todo!()
    }

    pub fn device_event(&mut self, event: DeviceEvent) {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn draw(&mut self) {
        todo!()
    }
}

#[derive(Default)]
pub struct Settings {
    pub screen: ScreenSettings,
}

impl Settings {
    pub fn load() -> Option<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ScreenSettings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: Option<Fullscreen>,
}

impl Default for ScreenSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fullscreen: None,
        }
    }
}
