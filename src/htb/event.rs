use once_cell::sync::OnceCell;
use std::sync::Mutex;
use winit::event_loop::EventLoopProxy;

use crate::menu::MenuEvent;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    StartGame,
    Exit,
}

impl Event {
    pub fn send(self) {
        let proxy = EVENT_LOOP_PROXY
            .get()
            .expect("Try to get uninitialized event loop proxy.")
            .lock()
            .unwrap();
        if let Err(e) = proxy.send_event(self) {
            let event = e.0;
            log::info!("Event sent after event loop destroyed: {event:?}.");
        }
    }

    pub fn register_event_proxy(proxy: EventLoopProxy<Event>) {
        EVENT_LOOP_PROXY
            .set(Mutex::new(proxy))
            .expect("Global event loop proxy initialized twice.");
    }
}

impl MenuEvent for Event {
    fn send(&self) {
        Event::send(*self)
    }
}

static EVENT_LOOP_PROXY: OnceCell<Mutex<EventLoopProxy<Event>>> = OnceCell::new();
