extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::cgc::{EventListener, EventSource};
use crate::life_game::LifeState;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use piston::input::MouseButton;
use piston::input::{
    Button, MouseCursorEvent, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use std::collections::HashMap;

pub mod cell;
pub mod cgc;
pub mod consts;
pub mod drawer;
pub mod grid;
pub mod life_game;
pub mod utils;

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

    //creating our game
    let mut app = life_game::LifeGame::new(40, 40, rules, false, false);

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
                match button {
                    Button::Keyboard(key) => {
                        println!("Key : {:?}", key);
                        match key {
                            Key::Space => {
                                if !app.in_step_mode() {
                                    app.toggle_on_step_mode();
                                } else {
                                    app.toggle_off_step_mode();
                                }
                            }
                            _ => {}
                        }
                    }
                    Button::Mouse(key) => match key {
                        MouseButton::Left => {
                            if app.in_step_mode() {
                                app.step();
                            }
                        }
                        _ => {}
                    },
                    //                    Button::Controller(_) => {}
                    //                    Button::Hat(_) => {}
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
