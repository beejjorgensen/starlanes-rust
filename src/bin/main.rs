//! Main game running code.

use starlanes::starlanes::{StarLanes, StarLanesOptions};

mod ui;

/// Command line options.
struct UserInterfaceOptions {
    wizard_mode: bool,
}

/// General information for displaying and running the UI.
struct UserInterface {
    wizard_mode: bool,
    player_count: usize,
    names: Vec<String>,
    game: StarLanes,
}

impl UserInterface {
    /// Create a new UserInterface.
    fn new(opts: UserInterfaceOptions) -> Self {
        Self {
            wizard_mode: opts.wizard_mode,
            player_count: 0,
            names: Vec::new(),
            game: StarLanes::new(),
        }
    }

    /// Main game loop.
    pub fn game_loop(&mut self) {
        ui::print_title();

        loop {
            // Play again loop.
            self.get_player_count();

            let game_opts = StarLanesOptions {
                player_count: self.player_count,
                wizard_mode: self.wizard_mode,
            };

            self.game.reset(Some(game_opts));
            ui::prompt_instructions();
            self.get_player_names();
            self.go_first_message();

            loop {
                // Main game loop
                self.wizard_alert();
                self.display_map();
                self.game.begin_turn();
                let candidates = self.game.get_moves();

                // This can happen if there aren't enough moves remaining.
                if self.game.game_is_over() {
                    break;
                }

                let move_point = self.get_move(&candidates);
                let events = self.game.make_move(move_point);
                self.handle_events(events);
                self.trade();
                self.game.end_turn();

                if self.game.game_is_over() {
                    break;
                }
            }

            self.final_stats();
            if !ui::play_again() {
                break;
            }
        }
    }
}

/// Parse the command line.
fn parse_command_line() -> Option<UserInterfaceOptions> {
    let mut options = UserInterfaceOptions { wizard_mode: false };

    for a in std::env::args().skip(1) {
        match a.as_str() {
            "--wizard" | "-w" => {
                options.wizard_mode = true;
            }
            _ => {
                return None;
            }
        }
    }

    Some(options)
}

/// Main.
fn main() {
    let options = if let Some(options) = parse_command_line() {
        options
    } else {
        eprintln!("usage: starlanes [-w|--wizard]");
        std::process::exit(1);
    };

    let mut user_interface = UserInterface::new(options);

    user_interface.game_loop();
}
