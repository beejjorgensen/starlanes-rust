use crate::company::COMPANY_COUNT;

#[derive(Debug)]
pub struct Player {
    holdings: [u64; COMPANY_COUNT],
    cash: u64,
}

impl Player {
    pub fn new() -> Self {
        //let mut holdings:[u64: COMPANY_COUNT] = [0, COMPANY_COUNT]
        Player {
            holdings: [0; COMPANY_COUNT],
            cash: 0,
        }
    }
}
