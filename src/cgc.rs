//custom graphic components

extern crate piston;
//mod utils;

use crate::utils::Point2d;
use piston::input::UpdateArgs;

pub trait EventListener {
    fn handle_event(&mut self, e: &piston::Event);
}

pub trait EventSource {
    fn add_event_listener(&mut self, lis: Box<dyn EventListener>);
}

pub trait GraphicComponent {
    fn draw(gl: &mut opengl_graphics::GlGraphics);

    fn update(args: &UpdateArgs);
}

pub struct Button {
    location: Point2d,
    text: String,
    listeners: Box<dyn EventListener>,
}

impl EventSource for Button {
    fn add_event_listener(&mut self, lis: Box<dyn EventListener>) {}
}
