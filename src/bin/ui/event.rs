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
            }
        }
    }
}
