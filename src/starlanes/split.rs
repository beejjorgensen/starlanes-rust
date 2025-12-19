use super::StarLanes;
use super::{DEFAULT_STOCK_SPLIT_FACTOR, DEFAULT_STOCK_SPLIT_LIMIT};
use crate::event::Event;

impl StarLanes {
    /// Do a stock split if necessary.
    pub(super) fn stock_split(&mut self, co_num: usize, events: &mut Vec<Event>) {
        let company = &mut self.companies[co_num];

        // In the original game, the stock could only split once per
        // move or per merge event. This was mathematical; the companies
        // had a maximum stock price of 3000 before splitting, so
        // joining two would make for a price of 6000, which would then
        // split 2-for-1 to get back to 3000.
        //
        // However, if we allow tuning of the parameters, it could be
        // that the stock would split repeatedly before it got below the
        // limit. As such, this is a while loop instead of an if
        // statement.
        while company.share_price > DEFAULT_STOCK_SPLIT_LIMIT {
            // Price is halved
            company.share_price /= DEFAULT_STOCK_SPLIT_FACTOR as u64;

            // Player's shares are doubled
            for p in &mut self.players {
                p.mul_holdings(co_num, DEFAULT_STOCK_SPLIT_FACTOR);
            }

            // Add stock split event
            events.push(Event::Split(co_num, DEFAULT_STOCK_SPLIT_FACTOR));
        }
    }
}
