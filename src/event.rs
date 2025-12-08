#[derive(Debug)]
pub struct Dividend {
    pub company: usize,
    pub amount: u64,
}

pub enum Event {
    CompanyFormed(usize),
    Dividends(Vec<Dividend>),
}
