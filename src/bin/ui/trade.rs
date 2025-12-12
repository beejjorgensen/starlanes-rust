//! Trading functions.
use crate::UserInterface;
use crate::ui;
use starlanes::starlanes::TradeError;

impl UserInterface {
    /// Trade stocks.
    pub(crate) fn trade(&mut self) {
        // Get a list of the company numbers that are available to trade.
        let trade_companies: Vec<usize> = self
            .game
            .get_companies()
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if c.in_use { Some(i) } else { None })
            .collect();

        // Trade all currently in-use companies, if any.
        for i in trade_companies {
            let co_name = ui::company_name(i);
            let share_price = self.game.get_company(i).share_price;

            let player = self.game.get_current_player();

            let holdings = player.get_holdings(i);
            let cash = player.get_cash();

            println!("YOUR CURRENT CASH= $ {}", cash);

            loop {
                println!("BUY HOW MANY SHARES OF {} AT $ {}", co_name, share_price);
                print!("{}YOU NOW OWN {} ", ui::tab(5), holdings);
                let to_buy = ui::input();

                if to_buy.starts_with('M') {
                    self.display_map();
                    continue;
                }

                if to_buy.starts_with('S') {
                    self.show_holdings();
                    continue;
                }

                let to_buy = to_buy.parse::<i64>().unwrap_or(0);

                match self.game.trade(i, to_buy) {
                    Err(TradeError::TooLittleCash) => {
                        println!("YOU ONLY HAVE $ {} - TRY AGAIN", cash);
                        continue;
                    }

                    Err(TradeError::TooLittleStock) => {
                        // This does not exist in the original game.
                        println!("YOU ONLY HAVE {} SHARES - TRY AGAIN", holdings);
                        continue;
                    }

                    Ok(_) => (),
                }

                break;
            }
        }
    }
}
