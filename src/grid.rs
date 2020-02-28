extern crate rand;

use self::rand::{random, Rng};
use crate::cell::Cell;
use crate::life_game::Rules;
use rand::prelude::thread_rng;

macro_rules! get_cell {
    ($gr:ident,$x:ident,$y:ident) => {
        $gr.grid[$x as usize][$y as usize]
    };
}

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub grid: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let mut rng = thread_rng();
        let mut grid: Vec<Vec<Cell>> = vec![];
        for _ in 0..width {
            let mut col=vec![];

            for _ in 0..height{
                col.push(Cell::new(random()));
            }
            grid.push(col);
        }
        Grid {
            width,
            height,
            grid,
        }
    }

    pub fn update(&mut self, rules: &Rules) {
        //checking neighbours
        for i in 0..self.width {
            for j in 0..self.height {
                let neighbours_number = self.count_neighbours(i, j);
                //let mut cell = self.get_cell(i, j);
                get_cell!(self, i, j).set_next_state(rules[&neighbours_number]);
            }
        }

        //after setting next round states we can execute them
        for i in 0..self.width {
            for j in 0..self.height {
                get_cell!(self, i, j).update_state();
            }
        }
    }
    fn count_neighbours(&self, x: u32, y: u32) -> u32 {
        if !in_bounds(x, y, self.width, self.height) {
            panic!("coordinate out of bounds!");
        }
        let mut count = 0;
        let mut begin_x: u32 = x;
        let mut begin_y: u32 = y;

        if x > 0 {
            begin_x -= 1;
        }

        if y > 0 {
            begin_y -= 1;
        }

        //count all neighbours (max 8)
        for i in begin_x..x + 2 {
            for j in begin_y..y + 2 {
                if in_bounds(i, j, self.width, self.height) && !coords_equal(i, j, x, y) {
                    if self.grid[i as usize][j as usize].is_alive() {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn get_cell_ref<'a>(&'a self, x: u32, y: u32) -> &'a Cell {
        &self.grid[x as usize][y as usize]
    }
}

fn in_bounds(x: u32, y: u32, x_bound: u32, y_bound: u32) -> bool {
    x < x_bound && x >= 0 && y < y_bound && y >= 0
}

fn coords_equal(x1: u32, y1: u32, x2: u32, y2: u32) -> bool {
    x1 == x2 && y1 == y2
}

//test submodule
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn coords_equal_test() {
        assert!(coords_equal(2, 2, 2, 2));
        assert_eq!(coords_equal(2, 3, 2, 4), false);
    }

    #[test]
    fn count_neighbours_test() {
        let mut grid = Grid::new(3, 3);
        let gr = vec![
            vec![Cell::new(false), Cell::new(true), Cell::new(false)],
            vec![Cell::new(false), Cell::new(false), Cell::new(false)],
            vec![Cell::new(false), Cell::new(false), Cell::new(false)],
        ];
        grid.grid = gr;
        assert_eq!(grid.count_neighbours(1, 1), 1);
        assert_eq!(grid.count_neighbours(0, 0), 1);

        grid.grid[1][1].set_life(true);
        assert_eq!(grid.count_neighbours(1, 1), 1);

        grid.grid[2][1].set_life(true);
        assert_eq!(grid.count_neighbours(1, 1), 2);
    }
}
