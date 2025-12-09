//! The game map.
//!
//! Each [`Map`] is a grid of [`MapCell`]s.
//!
//! [`Map`]: Map
//! [`MapCell`]: MapCell

use rand::Rng;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum MapCell {
    Space,
    Outpost,
    Star,
    Company(u32),
}

#[derive(Debug)]
pub struct Map {
    /// The width of the map in cells.
    pub width: usize,
    /// The height of the map in cells.
    pub height: usize,

    /// The [`MapCell`]s themselves.
    /// [`MapCell`]: MapCell
    data: Vec<Vec<MapCell>>,

    /// The probability of there being a star in a particular cell.
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

    pub fn set(&mut self, r: usize, c: usize, v: MapCell) {
        if r >= self.height || c >= self.width {
            panic!("map.set: coordinates out of range: {r},{c}");
        }

        self.data[r][c] = v;
    }

    pub fn get(&self, r: usize, c: usize) -> MapCell {
        if r >= self.height || c >= self.width {
            panic!("map.set: coordinates out of range: {r},{c}");
        }

        self.data[r][c]
    }
}
