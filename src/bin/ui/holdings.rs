//! Show holdings UI.
use crate::UserInterface;
use crate::ui;

impl UserInterface {
    /// Show the holdings of the given player.
    pub(crate) fn show_holdings(&self) {
        let player = self.game.get_current_player();
        let companies = self.game.get_companies();

        ui::formfeed();
        println!("\n\n{:<29}{:<20}YOUR HOLDINGS", "STOCK", "PRICE PER SHARE");

        for (i, c) in companies.iter().enumerate() {
            if !c.in_use {
                continue;
            }

            println!(
                "{:<29}{: <20}{: }",
                ui::company_name(i),
                ui::format_num(c.share_price),
                ui::format_num_signed(player.get_holdings(i))
            );
        }
    }
}
