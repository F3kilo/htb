use winit::event::DeviceEvent;

#[derive(Default)]
pub struct Game {}

impl Game {
    pub fn device_event(&mut self, event: DeviceEvent) {}
}
