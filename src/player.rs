//! Player information, stock holdings and cash.

/// Default cash for original game
const DEFAULT_CASH: u64 = 6000;

/// Player information.
#[derive(Debug)]
pub struct Player {
    /// Cash on-hand.
    cash: u64,

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

    /// Reset a player to starting conditions.
    pub fn reset(&mut self) {
        self.holdings.clear();
        self.cash = self.starting_cash;
    }

    /// Return holdings in a particular company.
    pub fn get_holdings(&self, company_idx: usize) -> u64 {
        if let Some(h) = self.holdings.get(company_idx) {
            *h
        } else {
            0
        }
    }

    /// Helper function to grow the holdings vector if necessary.
    fn grow_holdings_vec(&mut self, company_idx: usize) {
        let required_size = company_idx + 1;

        if self.holdings.len() < required_size {
            self.holdings.resize(required_size, 0);
        }
    }

    /// Set player holdings in a particular company.
    pub fn set_holdings(&mut self, company_idx: usize, holdings: u64) {
        self.grow_holdings_vec(company_idx);
        self.holdings[company_idx] = holdings;
    }

    /// Change player holdings in a particular company.
    pub fn add_holdings_signed(&mut self, company_idx: usize, delta: i64) {
        self.grow_holdings_vec(company_idx);
        self.holdings[company_idx] = self.holdings[company_idx].saturating_add_signed(delta);
    }

    /// Return player cash.
    pub fn get_cash(&self) -> u64 {
        self.cash
    }

    /// Set player cash.
    pub fn set_cash(&mut self, cash: u64) {
        self.cash = cash;
    }

    /// Add to player cash.
    pub fn add_cash(&mut self, delta: u64) -> u64 {
        self.cash = self.cash.saturating_add(delta);
        self.cash
    }

    /// Signed add to player cash.
    pub fn add_cash_signed(&mut self, delta: i64) -> u64 {
        self.cash = self.cash.saturating_add_signed(delta);
        self.cash
    }
}

impl Default for Player {
    /// Make a new default Player.
    fn default() -> Self {
        Self::new()
    }
}
