#![allow(dead_code, unused_variables)]

use self::htb::Settings;
use crate::htb::App;
use htb::Event;
use std::path::PathBuf;
use winit::dpi::PhysicalSize;
use winit::event::Event as SysEvent;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod htb;
pub mod menu;

fn main() -> anyhow::Result<()> {
    let env_path = load_env();
    init_logger()?;
    log::info!("Environment loaded from: {env_path:?}.");

    let event_loop = EventLoop::with_user_event();
    Event::register_event_proxy(event_loop.create_proxy());

    let settings = Settings::load().unwrap_or_default();
    let screen = &settings.screen;
    let window_size: PhysicalSize<u32> = (screen.width, screen.height).into();

    let window = WindowBuilder::default()
        .with_title("Hold the broadcast")
        .with_inner_size(window_size)
        .with_fullscreen(screen.fullscreen.clone())
        .build(&event_loop)
        .expect("Can't create window.");

    let mut app = App::new(settings, window);

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;
        match event {
            SysEvent::WindowEvent { event, .. } => {
                log::trace!("Got window event: {event:?}.");
                app.window_event(event);
            }
            SysEvent::DeviceEvent { device_id, event } => {
                log::trace!("Got device event: {device_id:?}, {event:?}.");
                app.device_event(event);
            }
            SysEvent::UserEvent(event) => {
                log::trace!("Got user event: {event:?}.");
                if let Event::Exit = event {
                    *flow = ControlFlow::Exit;
                }
                app.app_event(event)
            }
            SysEvent::MainEventsCleared => {
                log::trace!("Updated. Rendering...");
                app.draw();
            }
            _ => {}
        }
    });
}

fn load_env() -> Option<PathBuf> {
    dotenv::dotenv().ok()
}

fn init_logger() -> anyhow::Result<()> {
    env_logger::init();
    log::info!("Logger initialized.");
    Ok(())
}
