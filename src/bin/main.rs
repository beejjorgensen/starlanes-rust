use starlanes::event::Event;
use starlanes::starlanes::{Point, StarLanes};

mod ui;

/// Command line options
struct UserInterfaceOptions {
    wizard_mode: bool,
}

struct UserInterface {
    wizard_mode: bool,
    player_count: usize,
    names: Vec<String>,
    game: StarLanes,
}

impl UserInterface {
    /// Create a new UserInterface
    fn new(opts: UserInterfaceOptions) -> Self {
        Self {
            wizard_mode: opts.wizard_mode,
            player_count: 0,
            names: Vec::new(),
            game: StarLanes::new(),
        }
    }

    /// Prompt for and get the player count
    fn get_player_count(&mut self) {
        print!("HOW MANY PLAYERS (2-4)");
        let count = ui::input();
        self.player_count = count.parse().unwrap()
    }

    /// Prompt for and display instructions
    fn instructions(&self) {
        print!("DOES ANY PLAYER NEED INSTRUCTIONS");
        let yn = ui::input();

        if &yn[..1] == "Y" {
            ui::show_instructions();
        }
    }

    /// Get the player names
    fn get_player_names(&mut self) {
        self.names.clear();

        for i in 1..=self.player_count {
            print!("PLAYER {i} WHAT IS YOUR NAME");
            self.names.push(ui::input())
        }
    }

    /// Return the current player name
    fn get_current_player_name(&self) -> &String {
        &self.names[self.game.get_current_player_index()]
    }

    /// Print out who goes first
    ///
    /// The game has already decided this, so it's just informational.
    fn go_first_message(&self) {
        println!("\nNOW I WILL DECIDED WHO GOES FIRST...\n"); // DECIDED sic

        println!(
            "{} IS THE FIRST PLAYER TO MOVE.\n",
            self.get_current_player_name()
        );
    }

    fn get_move(&self, candidates: &[Point]) -> Point {
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
                ui::display_map(&self.game.map);
                continue;
            }

            if input.starts_with('S') {
                ui::show_holdings(self.game.get_current_player(), self.game.get_companies());
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

    /// Narc on wizards
    fn wizard_alert(&self) {
        if self.wizard_mode {
            println!("\n*******************");
            println!("*** WIZARD MODE ***");
            println!("*******************\n");
        }
    }

    /// Loop through post-move events
    fn handle_events(&self, events: Vec<Event>) {
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
            }
        }
    }

    /// Main game loop
    pub fn game_loop(&mut self) {
        ui::print_title();

        self.get_player_count();

        self.game.init(self.player_count, self.wizard_mode);

        self.instructions();

        self.get_player_names();

        self.go_first_message();

        loop {
            self.wizard_alert();

            ui::display_map(&self.game.map);

            self.game.begin_turn();

            let candidates = self.game.get_moves();

            let move_point = self.get_move(&candidates);

            let events = self.game.make_move(move_point);

            self.handle_events(events);

            self.game.end_turn();
        }
    }
}

/// Parse the command line
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

/// Main
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
