use crate::htb::view::Gfx;
use winit::event::DeviceEvent;

#[derive(Default)]
pub struct Game {}

impl Game {
    pub fn device_event(&mut self, event: DeviceEvent) {
        // todo!()
    }

    pub fn render(&self, gfx: &impl Gfx) {
        // todo!()
    }
}
