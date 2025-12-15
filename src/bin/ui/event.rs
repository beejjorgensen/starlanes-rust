//! Handle events.
use crate::UserInterface;
use crate::ui;
use starlanes::event::Event;

impl UserInterface {
    /// Loop through post-move events.
    pub(crate) fn handle_events(&self, events: Vec<Event>) {
        for event in &events {
            match event {
                Event::CompanyFormed(c) => {
                    ui::special_announcement();
                    ui::company_formed(*c);
                }

                Event::Dividends(_) => {
                    // Original game didn't display any UI for receiving
                    // dividends
                    //println("Dividends:\n{:#?}", d);
                }

                Event::Split(co_num, factor) => {
                    ui::special_announcement();
                    println!(
                        "THE STOCK OF {} HAS SPLIT {} FOR 1!",
                        ui::company_name(*co_num),
                        *factor
                    );
                    println!("\n\n\n\n");
                }

                Event::Merge(remaining_co, absorbed_co, merge_info) => {
                    ui::special_announcement();
                    println!(
                        "{} HAS JUST BEEN MERGED INTO {}!",
                        ui::company_name(*absorbed_co),
                        ui::company_name(*remaining_co)
                    );
                    println!("PLEASE NOTE THE FOLLOWING TRANSACTIONS.\n");
                    println!(
                        "{:9}{:12}{:12}{:19}BONUS PAID\n",
                        "PLAYER", "OLD STOCK", "NEW STOCK", "TOTAL HOLDINGS"
                    );

                    for (i, info) in merge_info.iter().enumerate() {
                        let player = self.game.get_player(i);
                        println!(
                            "{:9}{:12}{:12}{:19} ${}\n",
                            self.get_player_name(i),
                            ui::format_num_signed(info.old_stock),
                            ui::format_num_signed(info.new_stock),
                            ui::format_num_signed(player.get_holdings(*remaining_co)),
                            ui::format_num_signed(info.bonus_paid)
                        );
                    }
                }
            }
        }
    }
}
