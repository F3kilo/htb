use crate::htb::view::View;
pub use event::Event;
use render::{Render, RenderSettings};
use winit::event::{DeviceEvent, WindowEvent};
use winit::window::{Fullscreen, Window};

mod event;
mod menu;
mod render;
mod view;

pub struct App {
    settings: Settings,
    window: Window,
    view: View,
    render: Render,
}

impl App {
    pub fn new(mut settings: Settings, window: Window) -> Self {
        let render = Render::with_settings(&settings.render, &window).unwrap_or_else(|| {
            log::warn!(
                "Can't initialize render with settings: {:?}. Trying with default...",
                settings.render
            );
            let default_render_settings = RenderSettings::default();
            let render = Render::with_settings(&default_render_settings, &window);
            if render.is_none() {
                log::error!("Render initialization failed.");
            } else {
                log::info!("Render initialized with default settings.");
                settings.render = default_render_settings;
                settings.save();
            }
            render.expect("Render initialization failed.")
        });

        Self {
            settings,
            window,
            view: View::default(),
            render,
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
        // todo!()
    }

    pub fn draw(&self) {
        self.view.draw(&self.render);
    }
}

#[derive(Default)]
pub struct Settings {
    pub screen: ScreenSettings,
    pub render: RenderSettings,
}

impl Settings {
    pub fn load() -> Option<Self> {
        // todo!()
        None
    }

    pub fn save(&self) {
        // todo!()
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
