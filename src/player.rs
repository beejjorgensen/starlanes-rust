#[derive(Debug)]
pub struct Player {
    holdings: Vec<u64>,
    cash: u64,
}

impl Player {
    pub fn new(company_count: usize) -> Self {
        Player {
            holdings: vec![0; company_count],
            cash: 0,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new(5)
    }
}
