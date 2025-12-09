use starlanes::company::Company;
use starlanes::player::Player;

use crate::ui;

pub fn show_holdings(player: &Player, companies: &[Company]) {
    ui::formfeed();
    println!("\n\n{:<29}{:<19}YOUR HOLDINGS", "STOCK", "PRICE PER SHARE");

    for (i, c) in companies.iter().enumerate() {
        if !c.in_use {
            continue;
        }

        println!(
            "{:<29}{:<19}{}",
            ui::COMPANY_NAMES[i],
            c.share_price,
            player.holdings[i]
        );
    }
}
