use rand::Rng;
use crate::company;

#[derive(Debug)]
pub enum MapCell {
    Space,
    Outpost,
    Star,
    Company(company::Company),
}

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<MapCell>>,
}

const DEFAULT_WIDTH: usize = 13;
const DEFAULT_HEIGHT: usize = 9;
const DEFAULT_STAR_PROBABILITY: f32 = 0.05;

impl Map {
    pub fn new() -> Self {
        Self::new_with_params(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_STAR_PROBABILITY)
    }
    
    pub fn new_with_params(width: usize, height: usize, starp: f32) -> Self {
        let mut data: Vec<Vec<MapCell>> = Vec::new();
        let mut rng = rand::rng();

        for _ in 0..height {
            let mut row: Vec<MapCell> = Vec::new();

            for _ in 0..width {
                let s: f32 = rng.random();

                let new_cell = if s > starp {
                    MapCell::Space
                } else {
                    MapCell::Star
                };

                row.push(new_cell);
            }

            data.push(row);
        }

        Map {
            width,
            height,
            data,
        }
    }
}
