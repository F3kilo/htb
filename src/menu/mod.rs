use model::Control;

use self::model::{Action, ControlType};
use self::view::{ButtonView, ControlView, FrameView, PageView};

pub mod model;
pub mod view;

pub struct Page<E> {
    controls: Vec<Control<E>>,
    hovered: Option<usize>,
    pressed: Option<usize>,
}

impl<E: MenuEvent> Page<E> {
    pub fn new(controls: Vec<Control<E>>) -> Self {
        Self {
            controls,
            hovered: None,
            pressed: None,
        }
    }

    pub fn cursor_moved(&mut self, to: glam::Vec2) {
        let hovered =
            self.places_iter()
                .enumerate()
                .find_map(|(i, rect)| if rect.contains(to) { Some(i) } else { None });

        if let Some(hovered) = hovered {
            if self.controls[hovered].enabled {
                self.hovered = Some(hovered);
            }
        } else {
            self.hovered = None
        }
    }

    pub fn cursor_pressed(&mut self) {
        self.pressed = self.hovered;
    }

    pub fn cursor_released(&mut self) {
        let clicked = if self.hovered == self.pressed {
            self.pressed
        } else {
            None
        };

        if let Some(i) = clicked {
            self.controls[i].action(Action::Click)
        }

        self.pressed = None
    }

    pub fn view(&self) -> PageView {
        let control_place_iter = self.controls.iter().zip(self.places_iter());
        let controls = control_place_iter
            .map(|(control, place)| match &control.typ {
                ControlType::Button(btn) => {
                    let button = ButtonView {
                        enabled: control.enabled,
                        place,
                        text: btn.text.clone(),
                    };
                    ControlView::Button(button)
                }
            })
            .collect();

        let place = Rectangle {
            left_top: glam::vec2(-0.9, 0.9),
            right_bot: glam::vec2(0.9, -0.9),
        };
        let frame = FrameView { place };

        PageView {
            controls,
            hovered: self.hovered,
            pressed: self.pressed,
            frame,
        }
    }

    fn places_iter(&self) -> impl Iterator<Item = Rectangle> + '_ {
        PlacesIter::new(self.controls.iter())
    }
}

struct PlacesIter<'a, Event: 'a, Inner: Iterator<Item = &'a Control<Event>>> {
    top: f32,
    controls: Inner,
}

impl<'a, Event: 'a, Inner: Iterator<Item = &'a Control<Event>>> PlacesIter<'a, Event, Inner> {
    const INITIAL_TOP: f32 = 0.25;
    const BUTTON_WIDTH: f32 = 0.25;
    const BUTTON_HEIGHT: f32 = 0.1;
    const PADDING: f32 = 0.05;

    pub fn new(controls: Inner) -> Self {
        Self {
            top: Self::INITIAL_TOP,
            controls,
        }
    }
}

impl<'a, Event: 'a, Inner: Iterator<Item = &'a Control<Event>>> Iterator
    for PlacesIter<'a, Event, Inner>
{
    type Item = Rectangle;

    fn next(&mut self) -> Option<Self::Item> {
        self.controls.next().map(|_| {
            let left_top = glam::vec2(-Self::BUTTON_WIDTH / 2.0, self.top);
            let right_bot = glam::vec2(Self::BUTTON_WIDTH / 2.0, self.top - Self::BUTTON_HEIGHT);
            self.top -= Self::PADDING + Self::BUTTON_HEIGHT;

            Rectangle {
                left_top,
                right_bot,
            }
        })
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub left_top: glam::Vec2,
    pub right_bot: glam::Vec2,
}

impl Rectangle {
    pub fn contains(&self, point: glam::Vec2) -> bool {
        point.x > self.left()
            && point.x < self.right()
            && point.y > self.bot()
            && point.y < self.top()
    }

    pub fn center(&self) -> glam::Vec2 {
        (self.left_top + self.right_bot) / 2.0
    }

    pub fn top(&self) -> f32 {
        self.left_top.y
    }
    pub fn bot(&self) -> f32 {
        self.right_bot.y
    }
    pub fn left(&self) -> f32 {
        self.left_top.x
    }
    pub fn right(&self) -> f32 {
        self.right_bot.x
    }
}

pub trait MenuEvent {
    fn send(&self);
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::model::{Action, Button, Control};
    use super::{MenuEvent, Page, Rectangle};

    #[test]
    fn rectangle_contains() {
        let rect = Rectangle {
            left_top: glam::vec2(-1.0, 1.0),
            right_bot: glam::vec2(1.0, -1.0),
        };

        let point_inside = glam::vec2(-0.75, 0.75);
        assert!(rect.contains(point_inside));

        let point_outside = glam::vec2(-1.25, 0.75);
        assert!(!rect.contains(point_outside));
    }

    #[derive(Default)]
    struct Event {
        pub sent: Cell<bool>,
    }

    impl MenuEvent for &Event {
        fn send(&self) {
            self.sent.set(true)
        }
    }

    #[test]
    fn button_clicked() {
        let button = Button { text: "".into() };
        let event = Event::default();
        let control = Control {
            typ: super::model::ControlType::Button(button),
            enabled: true,
            handlers: [(Action::Click, &event)].into(),
        };
        let mut page = Page::new(vec![control]);

        let point_inside_button = glam::vec2(0.0, 0.2);
        page.cursor_moved(point_inside_button);
        page.cursor_pressed();
        page.cursor_released();
        assert!(event.sent.get());

        event.sent.set(false);
        let point_outside_button = glam::vec2(0.0, 0.3);
        page.cursor_moved(point_outside_button);
        page.cursor_pressed();
        page.cursor_released();
        assert!(!event.sent.get());
    }
}
