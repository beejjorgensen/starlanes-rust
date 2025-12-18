//! Miscellaneous UI functions.

use crate::UserInterface;
use crate::ui;

/// Print the game title
pub fn print_title() {
    ui::formfeed();
    println!(
        "\n\n\n{}* S * T * A * R ** L * A * N * E * S *",
        ui::tab(10)
    );
}

/// Special Announcement
pub fn special_announcement() {
    print!("\x07"); // bell
    println!("{}SPECIAL ANNOUNCEMENT!!\n", ui::tab(22));
}

impl UserInterface {
    /// Narc on wizards.
    pub(crate) fn wizard_alert(&self) {
        if self.wizard_mode() {
            println!("\n*******************");
            println!("*** WIZARD MODE ***");
            println!("*******************\n");
        }
    }
}
