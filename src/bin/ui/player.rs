//! Player-related functions.
use crate::UserInterface;
use crate::ui;

impl UserInterface {
    /// Prompt for and get the player count.
    pub(crate) fn get_player_count(&mut self) {
        print!("HOW MANY PLAYERS (2-4)");
        let count = ui::input();
        self.player_count = count.parse().unwrap()
    }

    /// Get the player names.
    pub(crate) fn get_player_names(&mut self) {
        self.names.clear();

        for i in 1..=self.player_count {
            print!("PLAYER {i} WHAT IS YOUR NAME");
            self.names.push(ui::input())
        }
    }

    /// Return a particular player's name.
    pub(crate) fn get_player_name(&self, n: usize) -> &String {
        &self.names[n]
    }

    /// Return the current player name.
    pub(crate) fn get_current_player_name(&self) -> &String {
        &self.names[self.game.get_current_player_index()]
    }

    /// Print out who goes first.
    ///
    /// The game has already decided this, so it's just informational.
    pub(crate) fn go_first_message(&self) {
        println!("\nNOW I WILL DECIDED WHO GOES FIRST...\n"); // DECIDED sic

        println!(
            "{} IS THE FIRST PLAYER TO MOVE.\n",
            self.get_current_player_name()
        );
    }
}
