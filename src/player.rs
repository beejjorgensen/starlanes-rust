//! Player information, stock holdings and cash.

/// Default cash for original game
const DEFAULT_CASH: u64 = 6000;

/// Player information.
#[derive(Debug)]
pub struct Player {
    /// Cash on-hand.
    pub cash: u64,

    /// Holdings in various companies. This is indexed by company number.
    holdings: Vec<u64>,

    /// How much cash the player should start with at the beginning of the
    /// game.
    starting_cash: u64,
}

impl Player {
    /// Construct a new player.
    pub fn new() -> Self {
        Self::new_with_params(DEFAULT_CASH)
    }

    /// Construct a new player with given parameters.
    pub fn new_with_params(starting_cash: u64) -> Self {
        Player {
            holdings: Vec::new(),
            cash: starting_cash,
            starting_cash,
        }
    }

    /// Return holdings in a particular company.
    pub fn get_holdings(&self, company_idx: usize) -> u64 {
        if let Some(h) = self.holdings.get(company_idx) {
            *h
        } else {
            0
        }
    }

    /// Set player holdings in a particular company.
    pub fn set_holdings(&mut self, company_idx: usize, holdings: u64) {
        let required_size = company_idx + 1;

        if self.holdings.len() < required_size {
            self.holdings.resize(required_size, 0);
        }

        self.holdings[company_idx] = holdings;
    }

    /// Change player holdings in a particular company.
    pub fn change_holdings(&mut self, company_idx: usize, delta: i64) {
        let required_size = company_idx + 1;

        if self.holdings.len() < required_size {
            self.holdings.resize(required_size, 0);
        }

        self.holdings[company_idx] = holdings;
    }

    /// Reset a player to starting conditions.
    pub fn reset(&mut self) {
        self.holdings.clear();
        self.cash = self.starting_cash;
    }
}

impl Default for Player {
    /// Make a new default Player.
    fn default() -> Self {
        Self::new()
    }
}
