use super::Rectangle;

pub trait RenderMenu {
    fn render(&self, menu: &MenuView);
}

pub struct MenuView {
    pub cursor: CursorView,
    pub controls: Vec<ControlView>,
    pub hovered: Option<usize>,
    pub frame: FrameView,
}

pub enum ControlView {
    Button(ButtonView),
}

pub struct ButtonView {
    pub place: Rectangle,
    pub text: String,
    pub enabled: bool,
}

pub struct CursorView {
    pub position: glam::Vec2,
}

pub struct FrameView {
    pub place: Rectangle,
}
