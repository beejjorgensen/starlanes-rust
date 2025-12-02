use rand::Rng;

#[derive(Debug,PartialEq)]
pub enum MapCell {
    Space,
    Outpost,
    Star,
    Company(u32),
}

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<MapCell>>,

    star_probability: f32,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

const DEFAULT_WIDTH: usize = 12;
const DEFAULT_HEIGHT: usize = 9;
const DEFAULT_STAR_PROBABILITY: f32 = 0.05;

impl Map {
    pub fn new() -> Self {
        Self::new_with_params(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_STAR_PROBABILITY)
    }

    pub fn new_with_params(width: usize, height: usize, star_probability: f32) -> Self {
        let data: Vec<Vec<MapCell>> = Vec::new();

        let mut m = Map {
            width,
            height,
            data,
            star_probability,
        };

        m.regenerate();

        m
    }

    pub fn regenerate(&mut self) {
        let mut rng = rand::rng();

        self.data.clear();

        for _ in 0..self.height {
            let mut row: Vec<MapCell> = Vec::new();

            for _ in 0..self.width {
                let s: f32 = rng.random();

                let new_cell = if s > self.star_probability {
                    MapCell::Space
                } else {
                    MapCell::Star
                };

                row.push(new_cell);
            }

            self.data.push(row);
        }
    }
}
