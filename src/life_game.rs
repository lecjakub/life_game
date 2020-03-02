extern crate opengl_graphics;
extern crate piston;

use crate::consts::*;
use crate::drawer::Drawer;
use crate::grid::Grid;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{
    Button, MouseCursorEvent, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
// use crate::drawer::Drawer;

#[derive(Copy, Clone)]
pub enum LifeState {
    Revive,
    Survive,
    Die,
    None,
}

pub type LivingNeighbours = u32;
pub type Rules = HashMap<LivingNeighbours, LifeState>;

pub struct LifeGame {
    pub drawer: crate::drawer::Drawer,
    pub grid: Grid,
    pub rules: Rules,
    pub in_menu: bool,
    pub paused: bool,
    max_round_time: f32,
    current_round_time: f32,
    last_round_update_time: Instant,
    step_mode: bool,
}

impl LifeGame {
    pub fn new(
        grid_width: u32,
        grid_height: u32,
        rules: HashMap<LivingNeighbours, LifeState>,
        in_menu: bool,
        paused: bool,
    ) -> LifeGame {
        LifeGame {
            grid: Grid::new(grid_width, grid_height),
            rules,
            in_menu,
            paused,
            drawer: Drawer::new(),
            max_round_time: 0.4,
            current_round_time: 0.0,
            last_round_update_time: std::time::Instant::now(),
            step_mode: false,
        }
    }
    pub fn render(&mut self, args: &RenderArgs) {
        self.drawer.draw_grid(args, &self.grid);
    }
    pub fn update(&mut self, args: &UpdateArgs) {
        if !self.paused && !self.step_mode {
            let break_time = self.last_round_update_time.elapsed().as_secs_f32();

            if break_time >= self.max_round_time {
                self.grid.update(&self.rules);
                self.last_round_update_time = Instant::now();
            }
        }
    }

    pub fn step(&mut self) {
        if !self.step_mode {
            panic!("trying to make step while not in step mode!!!!");
        }
        self.grid.update(&self.rules);
    }

    pub fn toggle_on_step_mode(&mut self) {
        self.step_mode = true;
    }

    pub fn toggle_off_step_mode(&mut self) {
        self.step_mode = false;
    }

    pub fn in_step_mode(&self) -> bool {
        self.step_mode
    }
    
}
