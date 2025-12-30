//! Player information, stock holdings and cash.

/// Default cash for original game
const DEFAULT_CASH: i64 = 6000;

/// Player information.
#[derive(Debug)]
pub struct Player {
    /// Cash on-hand. This is signed to potentially allow indebtedness as a game option.
    cash: i64,

    /// Holdings in various companies. This is indexed by company number.
    ///
    /// This really shouldn't be negative ever, but the original game had a bug that allowed for
    /// that.
    holdings: Vec<i64>,

    /// How much cash the player should start with at the beginning of the game.
    starting_cash: i64,
}

impl Player {
    /// Construct a new player.
    pub fn new() -> Self {
        Self::new_with_params(DEFAULT_CASH)
    }

    /// Construct a new player with given parameters.
    pub fn new_with_params(starting_cash: i64) -> Self {
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
    pub fn get_holdings(&self, company_idx: usize) -> i64 {
        *self.holdings.get(company_idx).unwrap_or(&0)
    }

    /// Helper function to grow the holdings vector if necessary.
    fn grow_holdings_vec(&mut self, company_idx: usize) {
        let required_size = company_idx + 1;

        if self.holdings.len() < required_size {
            self.holdings.resize(required_size, 0);
        }
    }

    /// Set player holdings in a particular company.
    pub fn set_holdings(&mut self, company_idx: usize, holdings: i64) {
        self.grow_holdings_vec(company_idx);
        self.holdings[company_idx] = holdings;
    }

    /// Add to player holdings in a particular company.
    pub fn add_holdings(&mut self, company_idx: usize, delta: u64) {
        self.grow_holdings_vec(company_idx);
        self.holdings[company_idx] = self.holdings[company_idx].saturating_add_unsigned(delta);
    }

    /// Change player holding in a particular company.
    pub fn add_holdings_signed(&mut self, company_idx: usize, delta: i64) {
        self.grow_holdings_vec(company_idx);
        self.holdings[company_idx] = self.holdings[company_idx].saturating_add(delta);
    }

    /// Multiply player holdings by some factor.
    pub fn mul_holdings(&mut self, company_idx: usize, factor: i64) {
        self.grow_holdings_vec(company_idx);
        self.holdings[company_idx] *= factor;
    }

    /// Return player cash.
    pub fn get_cash(&self) -> i64 {
        self.cash
    }

    /// Set player cash.
    pub fn set_cash(&mut self, cash: i64) {
        self.cash = cash;
    }

    /// Add to player cash.
    pub fn add_cash(&mut self, delta: i64) -> i64 {
        self.cash = self.cash.saturating_add(delta);
        self.cash
    }
}

impl Default for Player {
    /// Make a new default Player.
    fn default() -> Self {
        Self::new()
    }
}
