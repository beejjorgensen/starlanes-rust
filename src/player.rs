//! Player information, stock holdings and cash.

/// Number of companies in the original game.
const DEFAULT_COMPANY_COUNT: usize = 5;

/// Player information.
#[derive(Debug)]
pub struct Player {
    /// Holdings in various companies. This is indexed by company number.
    pub holdings: Vec<u64>,
    /// Cash on-hand.
    pub cash: u64,
}

impl Player {
    /// Construct a new player. `company_count` is the number of companies
    /// possible in the game, needed so the player can track holdings.
    /// TODO: make this a lazily allocated vector so we don't need to pass that
    /// in.
    pub fn new(company_count: usize) -> Self {
        Player {
            holdings: vec![0; company_count],
            cash: 0,
        }
    }

    // TODO add reset function
}

impl Default for Player {
    /// Make a new Player for the original game.
    fn default() -> Self {
        Self::new(DEFAULT_COMPANY_COUNT)
    }
}
