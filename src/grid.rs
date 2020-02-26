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
}
