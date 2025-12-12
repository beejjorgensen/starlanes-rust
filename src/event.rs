//! A game event.
//!
//! A vector of `Event`s is returned by [`make_move`], indicating the
//! results of the player move.
//!
//! [`make_move`]: crate::starlanes::StarLanes::make_move

/// A dividend result for the current player.
#[derive(Debug)]
pub struct Dividend {
    /// The company the dividend is from.
    pub company: usize,
    /// The amount of the dividend.
    pub amount: i64,
}

pub enum Event {
    /// A event representating company formation. The field is the company
    /// index number.
    CompanyFormed(usize),

    /// A event representating dividends payouts for the current player.
    Dividends(Vec<Dividend>),
}
