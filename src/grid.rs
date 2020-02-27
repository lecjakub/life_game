 pub struct Grid {
        pub width: u32,
        pub height: u32,
        pub grid: Vec<Vec<bool>>,
    }

    impl Grid {
        pub fn new(width: u32, height: u32) -> Grid {
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

        pub fn update(&mut self) {
            for i in &self.grid {
                for j in i {}
            }
        }
        fn count_neighbours(&self, x: u32, y: u32) -> u32 {
            if !in_bounds(x, y, self.width, self.height) {
                panic!("coordinate out of bounds!");
            }
            let mut count = 0;
            //count all neighbours (max 8)
            for i in x - 1..x + 2 {
                for j in y - 1..y + 2 {
                    if in_bounds(i, j, self.width, self.height) && !coords_equal(i, j, x, y) {
                        if self.grid[i as usize][j as usize] {
                            count += 1;
                        }
                    }
                }
            }
            count
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
     fn coords_equal_test(){
         assert!(coords_equal(2,2,2,2));
         assert_eq!(coords_equal(2,3,2,4),false);
     }

     #[test]
     fn count_neighbours_test(){
         let mut grid = Grid::new(3,3);
             let gr = vec!
         [
             vec![false,true,false],
             vec![false,false,false],
             vec![false,false,false],
         ];
         grid.grid = gr;
         assert_eq!(grid.count_neighbours(1,1),1);

         grid.grid[1][1] = true;
         assert_eq!(grid.count_neighbours(1,1),1);

         grid.grid[2][1] = true;
         assert_eq!(grid.count_neighbours(1,1),2);
     }
 }