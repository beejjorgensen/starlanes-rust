#[derive(Debug)]
pub struct Dividend {
    pub company: usize,
    pub amount: usize,
}

pub enum Event {
    CompanyFormed(usize),
    Dividends(Vec<Dividend>),
}
