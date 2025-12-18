//! Main game running code.

use starlanes::starlanes::StarLanes;

mod ui;

/// Command line options.
struct UserInterfaceOptions {
    wizard_mode: Option<bool>,
    max_turns: Option<usize>,
}

impl UserInterfaceOptions {
    fn new() -> Self {
        Self {
            wizard_mode: None,
            max_turns: None,
        }
    }
}

/// General information for displaying and running the UI.
struct UserInterface {
    player_count: usize,
    names: Vec<String>,
    game: StarLanes,
    options: UserInterfaceOptions,
}

impl UserInterface {
    /// Create a new UserInterface.
    fn new(options: UserInterfaceOptions) -> Self {
        Self {
            player_count: 0,
            names: Vec::new(),
            game: StarLanes::new(),
            options,
        }
    }

    /// Wizard mode test.
    fn wizard_mode(&self) -> bool {
        self.options.wizard_mode.unwrap_or(false)
    }

    /// Main game loop.
    pub fn game_loop(&mut self) {
        ui::print_title();

        self.game.options.wizard_mode = self.wizard_mode();
        if let Some(max_turns) = self.options.max_turns {
            self.game.options.max_turns = max_turns;
        }

        loop {
            // Play again loop.
            self.get_player_count();
            self.game.reset();
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

/// Print usage.
fn usage() {
    eprintln!("usage: starlanes [options]\n");
    eprintln!("       -w   --wizard        Start in wizard mode");
    eprintln!("       -t n --max-turns n   Set maximum game turns");
    eprintln!();
}

/// Parse the command line.
fn parse_command_line() -> Option<UserInterfaceOptions> {
    let mut options = UserInterfaceOptions::new();

    let mut args = std::env::args().skip(1);

    while let Some(a) = args.next() {
        match a.as_str() {
            "--help" | "-h" => {
                usage();
                std::process::exit(0);
            }
            "--wizard" | "-w" => {
                options.wizard_mode = Some(true);
            }
            "-t" | "--max-turns" => {
                let value = args.next();
                options.max_turns = Some(value?.parse().unwrap());
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
        usage();
        std::process::exit(1);
    };

    let mut user_interface = UserInterface::new(options);

    user_interface.game_loop();
}
