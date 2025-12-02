use crate::map::{Map, MapCell};
use crate::player::Player;
use rand::Rng;

const MAX_TURNS: usize = 48;

struct Point(usize, usize);

#[derive(Debug)]
enum GameState {
    PreInit,
    BeginTurn,
    GetMoves,
    GameOver,
}

use GameState::*;

#[derive(Debug)]
pub struct StarLanes {
    pub map: Map,

    turn_count: usize,
    state: GameState,
    player_count: usize,
    current_player: usize,
    players: Vec<Player>,
}

impl Default for StarLanes {
    fn default() -> Self {
        Self::new()
    }
}

impl StarLanes {
    pub fn new() -> Self {
        StarLanes {
            map: Map::new(),
            state: PreInit,
            player_count: 0,
            current_player: 0,
            players: Vec::new(),
        }
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
            self.players.push(Player::new());
        }

        self.state = BeginTurn;
    }

    pub fn get_current_player(&self) -> usize {
        self.current_player
    }
    
    pub fn begin_turn(&self) {
        if self.state != BeginTurn {
            panic!("begin_turn: invalid state: {}", self.state);
        }

        if self.turn >= MAX_TURNS {
            self.state = GameOver;
            return;
        }

        self.player = (self.player + 1) % self.player_count;

        self.state = GetMoves;
    }

    pub fn get_moves(&self) {
        if self.state != GetMoves {
            panic!("get_moves: invalid state: {}", self.state);
        }

        // Loop through map getting candidate moves
        let mut candidates: Vec<Point> = Vec::new();

        for (r, row) in self.map.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if map.data[r][c] == MapCell::Space {
                    candidates.push(Point(r, c));
                }
            }
        }

        candidates
    }
}
