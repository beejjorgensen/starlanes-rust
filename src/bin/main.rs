use starlanes::starlanes::{Point, StarLanes};

mod ui;

/// Print the game title
fn print_title() {
    ui::formfeed();
    println!(
        "\n\n\n{}* S * T * A * R ** L * A * N * E * S *",
        ui::tab(10)
    );
}

/// Prompt for and get the player count
fn get_player_count() -> usize {
    print!("HOW MANY PLAYERS (2-4)");
    let count = ui::input();
    count.parse().unwrap()
}

/// Prompt for and display instructions
fn instructions() {
    print!("DOES ANY PLAYER NEED INSTRUCTIONS");
    let yn = ui::input();

    if &yn[..1] == "Y" {
        println!("\n   STAR LANES IS A GAME OF INTERSTELLAR TRADING.");
        println!("THE OBJECT OF THE GAME IS TO AMASS THE GREATEST AMOUNT");
        println!("OF MONEY. THIS IS ACCOMPLISHED BY ESTABLISHING VAST,");
        println!("INTERSTELLAR SHIPPING LANES, AND PURCHASING STOCK IN");
        println!("THE COMPANIES THAT CONTROL THOSE TRADE ROUTES. DURING");
        println!("THE COURSE OF THE GAME, STOCK APPRECIATES IN VALUE AS");
        println!("THE SHIPPING COMPANIES BECOME LARGER. ALSO, SMALLER");
        println!("COMPANIES CAN BE MERGED INTO LARGER ONES, AND STOCK");
        println!("IN THE SMALLER FIRM IS CONVERTED INTO STOCK IN THE ");
        println!("LARGER ONE AS DESCRIBED BELOW.\n");

        println!("   EACH TURN, THE COMPUTER WILL PRESENT THE PLAYER WITH");
        println!("FIVE PROSPECTIVE SPACES TO OCCUPY ON A 9X12 MATRIX");
        println!("(ROWS 1-9, COLUMNS A-L). THE PLAYER, AFTER EXAMINING");
        println!("THE MAP OF THE GALAXY TO DECIDE WHICH SPACE HE WISHES");
        println!("TO OCCUPY, RESPONDS WITH THE ROW AND COLUMN OF THAT");
        println!("SPACE, I.E., 7E, 8A, ETC. THERE ARE FOUR POSSIBLE");
        println!("MOVES A PLAYER CAN MAKE.\n");

        println!("   1. HE CAN ESTABLISH AN UNATTACHED OUTPOST- IF HE");
        println!("SELECTS A SPACE THAT IS NOT ADJACENT TO A STAR, ANOTHER");
        println!("UNATTACHED OUTPOST, OR AN EXISTING SHIPPING LANE, THIS");
        println!("SPACE WILL BE DESIGNATED WITH A '+'. HE WILL THEN PROCEED");
        println!("WITH STOCK TRANSACTIONS, AS LISTED BELOW.\n");

        println!("   2. HE CAN ADD TO AN EXISTING LANE- IF HE SELECTS A SPACE");
        println!("WHICH IS ADJACENT TO ONE - AND ONLY ONE EXISTING SHIPPING");
        println!("LANE, THE SPACE HE SELECTS WILL BE ADDED TO THAT SHIPPING");
        println!("LANE, AND WILL BE DESIGNATED WITH THE FIRST LETTER OF ");
        println!("THE COMPANY THAT OWNS THAT LANE. IF THERE ARE ANY STARS");
        println!("OR UNATTACHED OUTPOSTS ALSO ADJACENT TO THE SELECTED SPACE,");
        println!("THEY, TOO, WILL BE INCORPORATED INTO THE EXISTING LANE.");
        println!("EACH NEW SQUARE ADJACENT TO A STAR ADDS $500 PER SHARE, AND");
        println!("EACH NEW OUTPOST ADDS $100 PER SHARE TO THE MARKET VALUE");
        println!("OF THE STOCK OF THAT SHIPPING COMPANY.\n");

        println!("   3. HE MAY ESTABLISH A NEW SHIPPING LANE- IF THERE");
        println!("ARE FIVE OR LESS EXISTING SHIPPING LANES ESTABLISHED,");
        println!("THE PLAYER MAY, GIVEN THE PROPER SPACE TO PLAY, ESTABLISH");
        println!("A NEW SHIPPING LANE. HE MAY DO THIS BY OCCUPYING A SPACE");
        println!("ADJACENT TO A STAR OR ANOTHER UNATTACHED OUTPOST, BUT ");
        println!("NOT ADJACENT TO AN EXISTING SHIPPING LANE. IF HE ");
        println!("ESTABLISHES A NEW SHIPPING LANE, HE IS AUTOMATICALLY");
        println!("ISSUED 5 SHARES IN THE NEW COMPANY AS A REWARD. HE");
        println!("MAY THEN PROCEED TO BUY STOCK IN THAT COMPANY, OR ANY");
        println!("OTHER ACTIVE COMPANY, AS DESCRIBED BELOW. THE MARKET ");
        println!("VALUE OF THE NEW STOCK IS ESTABLISHED BY THE NUMBER OF");
        println!("STARS AND OCCUPIED SPACES AS DESCRIBED IN #2 ABOVE.\n");

        println!("   4. HE MAY MERGE TWO EXISTING COMPANIES- IF PLAYER");
        println!("SELECTS A SPACE ADJACENT TO TWO EXISTING SHIPPING");
        println!("LANES, A MERGER OCCURS. THE LARGER COMPANY TAKES OVER");
        println!("THE SMALLER COMPANY, THE STOCK OF THE LARGER COMPANY IS");
        println!("INCREASED IN VALUE ACCORDING TO THE NUMBER OF SPACES AND");
        println!("STARS ADDED TO ITS LANE, EACH PLAYER'S STOCK IN THE");
        println!("SMALLER COMPANY IS EXCHANGED FOR SHARES IN THE LARGER");
        println!("ON A RATIO OF 2 SHARES OF THE SMALLER = 1 SHARE OF THE");
        println!("LARGER. ALSO, EACH PLAYER IS PAID A CASH BONUS PROPORTIONAL");
        println!("TO THE PERCENTAGE OF OUTSTANDING STOCK HE HELD IN THE");
        println!("SMALLER COMPANY. NOTE: AFTER A COMPANY BECOMES DEFUNCT");
        println!("THROUGH THIS MERGER PROCESS, IT CAN REAPPEAR ELSEWHERE");
        println!("ON THE BOARD IF A NEW COMPANY IS ESTABLISHED (SEE #3 ABOVE)\n");

        println!("   NEXT THE COMPUTER ADDS STOCK DIVIDENDS TO THE");
        println!("PLAYER'S CASH ON HAND (5% OF THE MARKET VALUE OF THE ");
        println!("STOCK IN HIS POSSESSION), AND OFFERS HIM THE OPPORTUNITY TO");
        println!("PURCHASE STOCK IN ANY OF THE ACTIVE COMPANIES ON THE");
        println!("BOARD. STOCK MAY NOT BE SOLD, BUT THE MARKET VALUES OF");
        println!("EACH PLAYER'S STOCK IS TAKEN INTO ACCOUNT AT THE END");
        println!("OF THE GAME TO DETERMINE THE WINNER. IF THE MARKET VALUE");
        println!("OF A GIVEN STOCK EXCEEDS $3000 AT ANY TIME DURING THE ");
        println!("GAME, THAT STOCK SPLITS TWO FOR ONE. THE PRICE IS CUT");
        println!("IN HALF, AND THE NUMBER OF SHARES OWNED BY EACH PLAYER");
        println!("IS DOUBLED.\n");

        println!("NOTE: THE PLAYER MAY LOOK AT HIS PORTFOLIO AT ANY TIME");
        println!("DURING THE COURSE OF HIS TURN BY RESPONDING WITH 'STOCK'");
        println!("TO AN INPUT STATEMENT. LIKEWISE, HE CAN REVIEW THE MAP");
        println!("OF THE GALAXY BY TYPING 'MAP' TO AN INPUT STATEMENT.\n");

        println!("GAME ENDS AFTER 48 MOVES. PLAYER WITH THE GREATEST");
        println!("NET WORTH AT THAT POINT IS THE WINNER.\n\n");
    }
}

/// Get the player names.
fn get_player_names(count: usize) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    for i in 1..=count {
        print!("PLAYER {i} WHAT IS YOUR NAME");
        names.push(ui::input())
    }

    names
}

/// Print out who goes first
///
/// The game has already decided this, so it's just informational.
fn go_first_message(game: &StarLanes, names: &[String]) {
    println!("\nNOW I WILL DECIDED WHO GOES FIRST...\n"); // DECIDED sic

    let current_player = game.get_current_player();

    println!("{} IS THE FIRST PLAYER TO MOVE.\n", names[current_player]);
}

fn get_move(game: &StarLanes, name: &String, candidates: &Vec<Point>) -> usize {
    let mut bad = false;
    let mut candidate_index: usize = 9999;

    // There is a bug in the original source where the name wasn't printed
    // again if a 'M'ap or 'S'tocks were requested. This horrid thing
    // recreates that bug.
    let mut bug_first = true;

    loop {
        if bad {
            println!("THAT SPACE WAS NOT INCLUDED IN THE LIST...");
            bad = false;
        } else {
            println!();
            if bug_first {
                print!("{name}");
                bug_first = false;
            }
            println!(", HERE ARE YOUR LEGAL MOVES FOR THIS TURN:");

            for Point(r, c) in candidates {
                print!(" {} {} /", r + 1, (b'A' + (*c as u8)) as char);
            }
        }

        print!("\nWHAT IS YOUR MOVE");

        let player_move_str = ui::input();

        if player_move_str.is_empty() {
            bad = true;
            continue;
        }

        if player_move_str.starts_with('M') {
            ui::display_map(&game.map);
            continue;
        }

        if player_move_str.len() < 2 {
            bad = true;
            continue;
        }

        let mut selrow: usize = 9999;
        let mut selcol: usize = 9999;

        // Unwrap safe here since the string is at least 2 long
        let lchars = player_move_str.get(0..player_move_str.len() - 1).unwrap();

        if let Ok(s) = lchars.trim().parse::<usize>() {
            selrow = s - 1;
        }

        // Unwrap safe here since the string is at least len-1 long
        let rchars = player_move_str.get(player_move_str.len() - 1..).unwrap();
        let rchar = rchars.chars().next().unwrap();

        if rchar as u8 >= b'A' {
            selcol = (rchar as u8 - b'A') as usize;
        }

        if let Some(i) = candidates.iter().position(|p| p == &Point(selrow, selcol)) {
            candidate_index = i;
        } else {
            bad = true;
        }

        if !bad {
            break;
        }
    }

    candidate_index
}

/// Main
fn main() {
    let mut game = StarLanes::new();

    print_title();

    let player_count = get_player_count();

    game.init(player_count);

    instructions();

    let names = get_player_names(player_count);

    go_first_message(&game, &names);

    ui::display_map(&game.map);

    game.begin_turn();

    let candidates = game.get_moves();

    let candidate_index = get_move(&game, &names[game.get_current_player()], &candidates);

    println!(
        "{} [{},{}]",
        candidate_index, candidates[candidate_index].0, candidates[candidate_index].1
    );

    //for c in candidates { print!("[{},{}] ", c.0, c.1); }
    //println!();
}
