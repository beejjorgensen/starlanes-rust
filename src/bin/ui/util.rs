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
