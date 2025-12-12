//! Getting and making moves.
use crate::UserInterface;
use crate::ui;
use starlanes::map::Point;

impl UserInterface {
    /// Have the user select a move from a group of candidates.
    pub(crate) fn get_move(&self, candidates: &[Point]) -> Point {
        // There is a bug in the original source where the name wasn't printed
        // again if a 'M'ap or 'S'tocks were requested. This horrid thing
        // recreates that bug.
        let mut bug_first = true;
        let mut show_error = false;

        let name = self.get_current_player_name();

        loop {
            if show_error {
                println!("THAT SPACE WAS NOT INCLUDED IN THE LIST...");
                show_error = false;
            } else {
                if bug_first {
                    print!("\n{name}");
                    bug_first = false;
                }
                println!(", HERE ARE YOUR LEGAL MOVES FOR THIS TURN:");

                for &Point(r, c) in candidates {
                    print!(" {} {} /", r + 1, (b'A' + (c as u8)) as char);
                }
                println!();
            }

            print!("WHAT IS YOUR MOVE");

            let input = ui::input();

            if input.is_empty() {
                show_error = true;
                continue;
            }

            if input.starts_with('M') {
                self.display_map();
                continue;
            }

            if input.starts_with('S') {
                self.show_holdings();
                continue;
            }

            if input.len() < 2 {
                show_error = true;
                continue;
            }

            let (row_str, col_str) = input.split_at(input.len() - 1);

            let selrow = match row_str.trim().parse::<usize>() {
                Ok(n) => n - 1,
                _ => {
                    show_error = true;
                    continue;
                }
            };

            let col_char = col_str.chars().next().unwrap();
            let selcol = match col_char {
                'A'..='Z' => (col_char as u8 - b'A') as usize,
                _ => {
                    show_error = true;
                    continue;
                }
            };

            let selpoint = Point(selrow, selcol);

            if self.wizard_mode || candidates.contains(&selpoint) {
                return selpoint;
            }

            show_error = true;
        }
    }
}
