//! Show holdings UI.
use crate::UserInterface;
use crate::ui;

impl UserInterface {
    /// Show the holdings of the given player.
    pub(crate) fn show_holdings(&self) {
        let player = self.game.get_current_player();
        let companies = self.game.get_companies();

        ui::formfeed();
        println!("\n\n{:<29}{:<19}YOUR HOLDINGS", "STOCK", "PRICE PER SHARE");

        for (i, c) in companies.iter().enumerate() {
            if !c.in_use {
                continue;
            }

            println!(
                "{:<29}{:<19}{}",
                ui::company_name(i),
                c.share_price,
                player.get_holdings(i)
            );
        }
    }
}
