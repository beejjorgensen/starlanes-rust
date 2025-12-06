use crate::company::Company;
use crate::map::{Map, MapCell};
use crate::player::Player;
use rand::Rng;
use rand::prelude::SliceRandom;

const MAX_TURNS: usize = 48;
const DEFAULT_MAX_COMPANY_COUNT: usize = 5;
const CANDIDATE_MOVE_COUNT: usize = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point(pub usize, pub usize);

#[derive(Debug, PartialEq)]
enum GameState {
    PreInit,
    BeginTurn,
    Move,
    EndTurn,
    GameOver,
}

use GameState::*;

#[derive(Debug)]
pub struct StarLanes {
    pub map: Map,

    turn_count: usize,
    turn: usize,
    state: GameState,
    player_count: usize,
    current_player: usize,
    players: Vec<Player>,
    max_company_count: usize,
    companies: Vec<Company>,
    candidate_moves: Vec<Point>,
}

impl Default for StarLanes {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct NeighborCounts {
    spaces: usize,
    stars: usize,
    outposts: usize,
    companies: usize,
    only_space: bool,
}

impl StarLanes {
    pub fn new() -> Self {
        let mut result = StarLanes {
            map: Map::new(),
            state: PreInit,
            player_count: 0,
            current_player: 0,
            turn_count: 0,
            turn: 0,
            players: Vec::new(),
            max_company_count: DEFAULT_MAX_COMPANY_COUNT,
            companies: Vec::new(),
            candidate_moves: Vec::new(),
        };

        for _ in 0..DEFAULT_MAX_COMPANY_COUNT {
            result.companies.push(Company::new());
        }

        result
    }

    pub fn init(&mut self, player_count: usize) {
        let mut rng = rand::rng();

        if !(1..=4).contains(&player_count) {
            panic!("invalid player count");
        }

        self.turn_count = 0;

        self.player_count = player_count;

        self.current_player = rng.random_range(0..self.player_count);

        for _ in 0..player_count {
            self.players.push(Player::new(self.max_company_count));
        }

        self.state = BeginTurn;
    }

    pub fn get_current_player(&self) -> usize {
        self.current_player
    }

    pub fn begin_turn(&mut self) {
        if self.state != BeginTurn {
            panic!("begin_turn: invalid state: {:#?}", self.state);
        }

        self.candidate_moves.clear();

        self.state = Move;
    }

    fn neighbor_count(&self, at_row: usize, at_col: usize) -> NeighborCounts {
        let mut result = NeighborCounts {
            spaces: 0,
            stars: 0,
            outposts: 0,
            companies: 0,
            only_space: false,
        };

        let offsets: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

        for [roffset, coffset] in offsets {
            let row = at_row as i32 + roffset;
            let col = at_col as i32 + coffset;

            if row < 0 || row >= self.map.height as i32 {
                continue;
            }

            if col < 0 || col >= self.map.width as i32 {
                continue;
            }

            match self.map.data[row as usize][col as usize] {
                MapCell::Space => result.spaces += 1,
                MapCell::Star => result.stars += 1,
                MapCell::Outpost => result.outposts += 1,
                MapCell::Company(_) => result.companies += 1,
            }
        }

        result.only_space = result.stars == 0 && result.outposts == 0 && result.companies == 0;

        result
    }

    fn active_company_count(&self) -> usize {
        let mut count: usize = 0;

        for c in &self.companies {
            if c.in_use {
                count += 1;
            }
        }

        count
    }

    fn companies_available(&self) -> bool {
        self.active_company_count() < self.max_company_count
    }

    pub fn get_moves(&mut self) -> Vec<Point> {
        if self.state != Move {
            panic!("get_moves: invalid state: {:#?}", self.state);
        }

        let mut candidates: Vec<Point> = Vec::new();

        // If we've already generated the moves this turn, just return them
        if !self.candidate_moves.is_empty() {
            candidates.extend(self.candidate_moves.iter().cloned());
            return candidates;
        }

        // Loop through map getting candidate moves

        for (r, row) in self.map.data.iter().enumerate() {
            for (c, mapcell) in row.iter().enumerate() {
                if *mapcell != MapCell::Space {
                    continue;
                }

                let neighbors = self.neighbor_count(r, c);

                if !self.companies_available()
                    && neighbors.companies == 0
                    && (neighbors.outposts > 0 || neighbors.stars > 0)
                {
                    continue;
                }

                candidates.push(Point(r, c));
            }
        }

        let mut rng = rand::rng();

        candidates.shuffle(&mut rng);

        // Check if not enough legal moves remaining on board--
        // this would cause an early game-over.
        if candidates.len() < CANDIDATE_MOVE_COUNT {
            candidates.truncate(0);
            self.state = GameOver;
            return candidates;
        }

        candidates.truncate(CANDIDATE_MOVE_COUNT);

        // Keep a copy for us to use later
        self.candidate_moves.extend(candidates.iter().cloned());

        candidates
    }

    pub fn make_move(&mut self, move_point: Point) {
        if self.state != Move {
            panic!("move: invalid state: {:#?}", self.state);
        }

        if !self.candidate_moves.contains(&move_point) {
            panic!("move: invalid move: {:?}", move_point);
        }

        // TODO beef up neighbor_count to get us the info we need.
        let Point(row, col) = move_point;

        let neighbors = self.neighbor_count(row, col);

        // println!("{:#?}", neighbors);

        if neighbors.only_space {
            self.map.set(row, col, MapCell::Outpost);
        }

        self.state = EndTurn;
    }

    pub fn end_turn(&mut self) {
        if self.state != EndTurn {
            panic!("move: invalid state: {:#?}", self.state);
        }

        if self.turn >= MAX_TURNS {
            self.state = GameOver;
            return;
        }

        self.current_player = (self.current_player + 1) % self.player_count;

        self.state = BeginTurn;
    }
}
