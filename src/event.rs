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

/// A merge result for a single player.
#[derive(Debug)]
pub struct MergeInfo {
    /// The stock held in the absorbed company.
    pub old_stock: i64,

    /// The old stock amount convered to new stock.
    pub new_stock: i64,

    /// Cash bonus paid out.
    pub bonus_paid: i64,
}

pub enum Event {
    /// A event representating company formation. The field is the company
    /// index number.
    CompanyFormed(usize),

    /// A event representating dividends payouts for the current player.
    Dividends(Vec<Dividend>),

    /// Stock has split. Fields: company index, split factor.
    Split(usize, i64),

    /// Companies have merged. Fields: remaining company, absorbed
    /// company, and a set of MergeInfo objects for each player.
    Merge(usize, usize, Vec<MergeInfo>),
}
