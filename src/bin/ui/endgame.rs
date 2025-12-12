//! Game over functionality.
use crate::UserInterface;
use crate::ui;

impl UserInterface {
    /// Print final stats
    pub(crate) fn final_stats(&self) {
        let companies = self.game.get_companies();

        ui::special_announcement();
        println!("THE GAME IS OVER - HERE ARE THE FINAL STANDINGS\n");

        println!(
            "{:<9}{:<23}{:<17}NET WORTH",
            "PLAYER", "CASH VALUE OF STOCK", "CASH ON HAND"
        );

        for (p, name) in self.names.iter().enumerate() {
            let player = self.game.get_player(p);

            let mut total_stock_value: i64 = 0;
            for (i, c) in companies.iter().enumerate().filter(|(_, c)| c.in_use) {
                let stock_value = player.get_holdings(i) * c.share_price as i64;
                total_stock_value += stock_value;
            }

            let cash = player.get_cash();
            let cash_str = format!("$ {}", cash);

            let net_worth_str = format!("$ {}", total_stock_value + cash);

            println!(
                "{:<9}{:<23}{:<17}{}",
                name, total_stock_value, cash_str, net_worth_str
            );
        }
    }
}

/// Ask for another game
pub fn play_again() -> bool {
    print!("ANOTHER GAME");
    let yn = ui::input();

    yn.starts_with('Y')
}
