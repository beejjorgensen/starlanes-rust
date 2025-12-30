use super::DEFAULT_DIVIDEND_PERCENTAGE;
use super::StarLanes;
use crate::event::{Dividend, Event};

impl StarLanes {
    /// Computes the dividents for the current player.
    ///
    /// This also adds an [`Event`] describing the [`Dividend`] per company that the UI can use to
    /// display the info. (The original game did not display anything.)
    pub(super) fn dividends(&mut self, events: &mut Vec<Event>) {
        let mut dividends: Vec<Dividend> = Vec::new();
        let player = &mut self.players[self.current_player];

        for (idx, c) in self.companies.iter().enumerate() {
            if !c.in_use {
                continue;
            }

            let amount = (DEFAULT_DIVIDEND_PERCENTAGE / 100.0
                * c.share_price as f32
                * player.get_holdings(idx) as f32)
                .round() as i64;

            dividends.push(Dividend {
                company: idx,
                amount,
            });

            player.add_cash(amount);
        }

        if !dividends.is_empty() {
            events.push(Event::Dividends(dividends));
        }
    }
}
