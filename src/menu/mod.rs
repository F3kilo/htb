use model::Control;

pub mod model;
pub mod view;

pub struct Page<Event> {
    pub controls: Vec<Control<Event>>,
    pub focused: Option<usize>,
}

impl<Event> Page<Event> {
    pub fn new(controls: Vec<Control<Event>>) -> Self {
        Self {
            controls,
            focused: None,
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
            controls
        }
    }
}

impl<'a, Event: 'a, Inner: Iterator<Item = &'a Control<Event>>> Iterator
    for PlacesIter<'a, Event, Inner>
{
    type Item = Rectangle;

    fn next(&mut self) -> Option<Self::Item> {
        let left_top = glam::vec2(-Self::BUTTON_WIDTH / 2.0, self.top);
        self.top += Self::BUTTON_HEIGHT;
        let right_bot = glam::vec2(Self::BUTTON_WIDTH / 2.0, self.top);
        self.top += Self::PADDING;
        
        Some(Rectangle {
            left_top,
            right_bot,
        })
    }
}

struct Rectangle {
    left_top: glam::Vec2,
    right_bot: glam::Vec2,
}
