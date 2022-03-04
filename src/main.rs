use std::path::PathBuf;
use winit::dpi::PhysicalSize;
use winit::event::{Event, StartCause};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> anyhow::Result<()> {
    let env_path = load_env();
    init_logger()?;
    log::info!("Environment loaded from: {env_path:?}.");

    let event_loop = EventLoop::new();

    let window_size = PhysicalSize::new(800u32, 600);
    let _window = WindowBuilder::default()
        .with_title("Hold the broadcast.")
        .with_inner_size(window_size)
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, flow| {
        *flow = ControlFlow::Poll;
        match event {
            Event::NewEvents(start_cause) => match start_cause {
                StartCause::Init => {
                    log::info!("Event loop initialized.");
                }
                _ => {}
            }
            Event::WindowEvent { event, .. } => {
                log::trace!("Got window event: {event:?}.");
            }
            Event::DeviceEvent { device_id, event } => {
                log::trace!("Got device event: {device_id:?}, {event:?}.");
            }
            Event::MainEventsCleared => {
                log::trace!("Events cleared. Updating...");
                log::trace!("Updated. Rendering...");
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
