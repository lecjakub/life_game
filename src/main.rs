extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub mod cgc;
pub mod utils;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use std::collections::HashMap;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const background: [f32; 4] = [0.780, 0.964, 0.568, 1.0];

#[derive(Clone)]
enum LifeState {
    Revive,
    Survive,
    Die,
}
type living_neighbours = u32;

struct Grid {
    width: u32,
    height: u32,
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Grid {
        let mut vec2d: Vec<Vec<bool>> = vec![];
        for _ in 0..height {
            vec2d.push(vec![false; width as usize]);
        }
        Grid {
            width: width,
            height: height,
            grid: vec2d,
        }
    }
}

struct LifeGame {
    gl: GlGraphics,
    grid: Grid,
    rules: HashMap<living_neighbours, LifeState>,
    rotation: f64,
    in_menu: bool,
    paused: bool,
}
fn draw_menu(c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
    use graphics::*;
    clear(background, gl);

    let transform = c.transform.trans(0.0, 0.0).rot_rad(0.5).trans(-25.0, -25.0);

    // Draw a box rotating around the middle of the screen.
    rectangle(RED, rectangle::square(0.0, 0.0, 50.0), transform, gl);
}

impl LifeGame {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        // const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        // let background: [f32; 4] = [0.780, 0.964, 0.568, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            draw_menu(c, gl);
            //Clear the screen.

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
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

    let mut app = LifeGame {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        grid: Grid::new(10, 10),
        rules: rules,
        in_menu: true,
        paused: false,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        //if let Some(args) =

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.press_args() {}
    }
}
