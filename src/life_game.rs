extern crate opengl_graphics;
extern crate piston;

use crate::consts::*;
use crate::grid::Grid;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{
    Button, MouseCursorEvent, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use std::collections::HashMap;
// use crate::drawer::Drawer;

#[derive(Clone)]
pub enum LifeState {
    Revive,
    Survive,
    Die,
}

type LivingNeighbours = u32;

pub struct LifeGame {
    pub drawer: crate::drawer::Drawer,
    pub grid: Grid,
    pub rules: HashMap<LivingNeighbours, LifeState>,
    pub rotation: f64,
    pub in_menu: bool,
    pub paused: bool,
}

impl LifeGame {
    pub fn render(&mut self, args: &RenderArgs) {
        self.drawer.draw_grid(args, &self.grid);
    }
    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}
