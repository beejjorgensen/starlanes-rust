//! The game map.
//!
//! Each [`Map`] is a grid of [`MapCell`]s. Cell 0,0 is the upper left, row
//! zero, columm zero.
//!
//! [`Map`]: Map
//! [`MapCell`]: MapCell

use rand::Rng;

/// A row, column point on the map.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point(pub usize, pub usize);

/// All the things that can appear in a map cell.
#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum MapCell {
    // Empty space.
    Space,
    // An unaffiliated outpost.
    Outpost,
    // A star.
    Star,
    // A company, identified by the `u32` field.
    Company(u32),
}

/// The map data.
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

/// Width of the map in the original game.
const DEFAULT_WIDTH: usize = 12;

/// Height of the map in the original game.
const DEFAULT_HEIGHT: usize = 9;

/// Star probability in the original game.
const DEFAULT_STAR_PROBABILITY: f32 = 0.05;

impl Map {
    /// Construct a new map with the original game parameters.
    pub fn new() -> Self {
        Self::new_with_params(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_STAR_PROBABILITY)
    }

    /// Construct a new map with custom parameters.
    ///
    /// The `star_probability` is the probability of any cell being a star,
    /// e.g. `0.05`.
    pub fn new_with_params(width: usize, height: usize, star_probability: f32) -> Self {
        let data: Vec<Vec<MapCell>> = Vec::new();

        Map {
            width,
            height,
            data,
            star_probability,
        }
    }

    /// Regenerate the map, overwriting any previous map data.
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

    /// Set a row, column map cell to a particular value.
    pub fn set(&mut self, r: usize, c: usize, v: MapCell) {
        if r >= self.height || c >= self.width {
            panic!("map.set: coordinates out of range: {r},{c}");
        }

        self.data[r][c] = v;
    }

    /// Get a map cell at a particular row, column.
    pub fn get(&self, r: usize, c: usize) -> MapCell {
        if r >= self.height || c >= self.width {
            panic!("map.set: coordinates out of range: {r},{c}");
        }

        self.data[r][c]
    }

    /// Convert all companies from one type to another. Doesn't change
    /// anything other than the map.
    pub fn convert(&mut self, conv_from: usize, conv_to: usize) {
        for r in 0..self.height {
            for c in 0..self.width {
                if let MapCell::Company(existing) = self.get(r, c)
                    && existing as usize == conv_from
                {
                    self.set(r, c, MapCell::Company(conv_to as u32));
                }
            }
        }
    }
}
