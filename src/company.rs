//! A company.

#[derive(Debug)]
pub struct Company {
    /// True if the company is currently in play.
    pub in_use: bool,
    /// The size of the company on the game board (cell count).
    pub size: u64,
    /// The price-per-share of the company.
    pub share_price: u64,
}

impl Company {
    /// Constructs a new company.
    pub fn new() -> Self {
        Company {
            in_use: false,
            size: 0,
            share_price: 100,
        }
    }
}

impl Default for Company {
    fn default() -> Self {
        Self::new()
    }
}
