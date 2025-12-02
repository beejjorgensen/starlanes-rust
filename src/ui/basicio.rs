use std::io;
use std::io::Write;

/// Simulates a formfeed.
///
/// This would eject a sheet of paper on a teletype, but it's annoying
/// on a screen unless it's a clear. So it's been reduced here.
pub fn formfeed() {
    const LINEFEEDS_PER_FORMFEED: usize = 5;

    for _ in 0..LINEFEEDS_PER_FORMFEED {
        println!();
    }
}

/// Moves the cursor to the nth column to the right.
///
/// This simulates the BASIC `TAB` function. Effectively it prints n-1
/// spaces so the the next character will be in the nth column.
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

