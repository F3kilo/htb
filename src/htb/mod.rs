use crate::htb::view::View;
pub use event::Event;
use winit::event::{DeviceEvent, WindowEvent};
use winit::window::{Fullscreen, Window};

mod event;
mod menu;
mod view;

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
        match event {
            Event::StartGame => {
                log::info!("Starting game")
            }
            _ => {}
        }
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => Event::Exit.send(),
            _ => self.view.window_event(event),
        }
    }

    pub fn device_event(&mut self, event: DeviceEvent) {
        self.view.device_event(event)
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
