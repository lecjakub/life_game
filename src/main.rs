extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Button, MouseCursorEvent};
use piston::window::WindowSettings;
use std::collections::HashMap;
use crate::cgc::{EventSource, EventListener};
use life_game::LifeState;


pub mod cgc;
pub mod utils;
pub mod life_game;
pub mod grid;
pub mod consts;
pub mod drawer;


fn main() {
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [1080, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut rules = HashMap::new();
    rules.insert(0, LifeState::Die);
    rules.insert(1, LifeState::Die);
    rules.insert(2, LifeState::Survive);
    rules.insert(3, LifeState::Revive);
    rules.insert(4, LifeState::Die);
    rules.insert(5, LifeState::Die);
    rules.insert(6, LifeState::Die);
    rules.insert(7, LifeState::Die);
    rules.insert(8, LifeState::Die);

    let mut app = life_game::LifeGame {
        rotation: 0.0,
        grid: grid::Grid::new(40, 40),
        rules: rules,
        in_menu: true,
        paused: false,
        drawer: drawer::Drawer::new(),
    };

    let mut events = Events::new(EventSettings::new());
    let mut cursor_pos: [f64; 2] = [0.0, 0.0];

    // main game loop
    while let Some(e) = events.next(&mut window) {
        match e.render_args() {
            Some(args) => app.render(&args),
            _ => {}
        }
        match e.update_args() {
            Some(args) => app.update(&args),
            _ => {}
        }
        e.mouse_cursor(|pos| {
            cursor_pos = pos;
        });

        match e.press_args() {
            Some(button) => {
                println!("Clicked on X:{} Y:{} ", cursor_pos[0], cursor_pos[1]);
//                match button {
//                    Button::Keyboard(key) => key_value = key.code(),
//                    Button::Mouse(_) => {}
//                    Button::Controller(_) => {}
//                    Button::Hat(_) => {}
//                }
            }
            _ => {}
        }
    }
}
