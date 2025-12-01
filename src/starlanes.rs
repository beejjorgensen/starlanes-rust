use crate::map::Map;
use rand::Rng;

#[derive(Debug)]
enum GameState {
    PreInit,
    Ready,
}

use GameState::*;

#[derive(Debug)]
pub struct StarLanes {
    pub map: Map,

    state: GameState,
    player_count: usize,
    current_player: usize,
}

impl StarLanes {
    pub fn new() -> Self {
        StarLanes {
            map: Map::new(),
            state: PreInit,
            player_count: 0,
            current_player: 0,
        }
    }

    pub fn init(&mut self, player_count: usize) {
        let mut rng = rand::rng();

        if player_count < 1 || player_count > 4 {
            panic!("invalid player count");
        }

        self.player_count = player_count;
        self.state = Ready;

        self.current_player = rng.random_range(0..self.player_count);
    }
        
    pub fn get_current_player(&self) -> usize {
        self.current_player
    }
}
