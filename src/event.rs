//! A game event.
//!
//! A vector of `Event`s is returned by [`make_move`], indicating the
//! results of the player move.
//!
//! [`make_move`]: crate::starlanes::StarLanes::make_move

#[derive(Debug)]
pub struct Dividend {
    pub company: usize,
    pub amount: u64,
}

pub enum Event {
    /// A event representating company formation. The field is the company
    /// index number.
    CompanyFormed(usize),

    /// A event representating dividends payouts for the current player,
    Dividends(Vec<Dividend>),
}
