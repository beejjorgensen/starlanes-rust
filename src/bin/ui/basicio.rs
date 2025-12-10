//! I/O functions that tend to ease porting from BASIC.
use std::io;
use std::io::Write;

/// Prints a formfeed.
pub fn formfeed() {
    print!("{}", 12 as char);
}

/// Moves the cursor to the nth column to the right.
///
/// This simulates the BASIC `TAB` function. Effectively it prints n-1
/// spaces so the the next character will be in the nth column.
///
/// WARNING: This function only works for the first tab--it doesn't actually
/// do any position tracking at all.
pub fn tab(n: usize) -> String {
    format!("{:>width$}", "", width = n - 1)
}

/// Read a line of text from stdin.
///
/// This simulates the BASIC `INPUT` statement to a degree, the main
/// difference being that it always returns a string. It's up to the
/// caller to convert to other types as needed.
pub fn input() -> String {
    print!("? ");
    _ = io::stdout().flush();

    let mut input = String::new();
    _ = io::stdin().read_line(&mut input);

    // Original game only allowed uppercase input, but we'll take this
    // liberty to keep the user from going insane.
    input.to_uppercase().trim().to_string()
}
