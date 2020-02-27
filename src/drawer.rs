extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::consts::*;
use crate::grid::Grid;
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
            let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

            //draw background
            clear(BACKGROUND, gl);

            let (tile_width, tile_height) = (10.0, 10.0);

            //drawing vertical lines
            for i in 0..(grid.width + 1) {
                let x1 = i as Scalar * tile_width;
                let y1 = 0.0;
                let x2 = x1;
                let y2 = (tile_height * grid.height as f64) as Scalar;
                line(BLACK, 0.1, [x1, y1, x2, y2], c.transform, gl)
            }
            //drawing horizontal lines
            for i in 0..(grid.height+1) {
                let x1 = 0.0 as Scalar;
                let y1 = (i as f64 * tile_height) as Scalar;
                let x2 = (tile_width * grid.width as f64) as Scalar;
                let y2 = y1 as Scalar;

                line(BLACK, 0.1, [x1, y1, x2, y2], c.transform, gl)
            }
        });
    }
}

