use crate::map::Map;
use crate::player::Player;
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

        self.player_count = player_count;
        self.state = Ready;

        self.current_player = rng.random_range(0..self.player_count);

        for _ in 0..player_count {
            self.players.push(Player::new());
        }
    }

    pub fn get_current_player(&self) -> usize {
        self.current_player
    }
}
