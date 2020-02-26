extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::consts::*;
use crate::grid::Grid;
use graphics::color::BLACK;
use graphics::math::Scalar;
use graphics::rectangle::square;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{
    Button, MouseCursorEvent, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;

pub struct Drawer {
    gl: GlGraphics,
}

impl Drawer {
    pub fn new() -> Drawer {
        Drawer {
            gl: GlGraphics::new(OpenGL::V3_2),
        }
    }
    pub fn draw_grid(&mut self, args: &RenderArgs, grid: &Grid) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            let square = rectangle::square(0.0, 0.0, 50.0);
            let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            draw_menu(c, gl);

            let (tile_width, tile_height) = (10.0, 10.0);

            //drawing vertical lines
            for i in 1..(grid.width) {
                let x1 = i as Scalar * tile_width;
                let y1 = 0.0;
                let x2 = x1;
                let y2 = (tile_height * grid.height as f64) as Scalar;
                line(BLACK, 0.5, [x1, y1, x2, y2], c.transform, gl)
            }
            //drawing horizontal lines
            for i in 1..(grid.width) {
                let x1 = 0.0;
                let y1 = i as Scalar * tile_height;
                let x2 = (tile_width * grid.width as f64) as Scalar;
                let y2 = y1;
                line(BLACK, 0.5, [x1, y1, x2, y2], c.transform, gl)
            }

            let transform = c
                .transform
                .trans(x, y)
                //.rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}

fn draw_menu(c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
    use graphics::*;
    clear(BACKGROUND, gl);
    let transform = c.transform.trans(0.0, 0.0).rot_rad(0.5).trans(-25.0, -25.0);

    // Draw a box rotating around the middle of the screen.
    rectangle(RED, rectangle::square(50.0, 10.0, 50.0), transform, gl);
}
