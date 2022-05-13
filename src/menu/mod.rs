use model::Control;

use self::model::Action;

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
        let focused =
            self.places_iter()
                .enumerate()
                .find_map(|(i, rect)| if rect.contains(to) { Some(i) } else { None });
        self.hovered = focused
    }

    pub fn cursor_pressed(&mut self) {
        self.pressed = self.hovered
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
        let left_top = glam::vec2(-Self::BUTTON_WIDTH / 2.0, self.top);
        let right_bot = glam::vec2(Self::BUTTON_WIDTH / 2.0, self.top - Self::BUTTON_HEIGHT);
        self.top -= Self::PADDING + Self::BUTTON_HEIGHT;

        Some(Rectangle {
            left_top,
            right_bot,
        })
    }
}

pub struct Rectangle {
    left_top: glam::Vec2,
    right_bot: glam::Vec2,
}

impl Rectangle {
    pub fn contains(&self, point: glam::Vec2) -> bool {
        point.x > self.left()
            && point.x < self.right()
            && point.y > self.bot()
            && point.y < self.top()
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
