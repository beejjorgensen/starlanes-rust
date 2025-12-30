//! This is the main game module.
//!
//! An external UI driver will run through the following steps.
//!
//! 1. Create a new [`StarLanes`] object.
//! 2. Call [`reset`] on that object.
//! 3. Call [`begin_turn`].
//! 4. Call [`get_moves`].
//! 5. Test [`game_is_over`].
//! 6. Call [`make_move`].
//! 7. Call [`end_turn`].
//! 8. Test [`game_is_over`].
//! 9. `GOTO` step 3.
//!
//! After [`get_moves`] or [`end_turn`], the UI should check if the game is over and act
//! accordingly.
//!
//! [`reset`]: StarLanes::reset
//! [`begin_turn`]: StarLanes::begin_turn
//! [`get_moves`]: StarLanes::get_moves
//! [`game_is_over`]: StarLanes::game_is_over
//! [`make_move`]: StarLanes::make_move
//! [`end_turn`]: StarLanes::end_turn

use crate::company::Company;
use crate::event::Event;
use crate::map::{Map, MapCell, Point};
use crate::player::Player;
use rand::Rng;
use rand::prelude::SliceRandom;
use std::collections::HashMap;

mod dividends;
mod merge;
mod split;

const DEFAULT_MAX_TURNS: usize = 48;
const DEFAULT_MAX_COMPANY_COUNT: usize = 5;
const DEFAULT_CANDIDATE_MOVE_COUNT: usize = 5;

const DEFAULT_STAR_PRICE_BOOST: u64 = 500;
const DEFAULT_GROWTH_PRICE_BOOST: u64 = 100;
const DEFAULT_OUTPOST_PRICE_BOOST: u64 = 100;
// Modifying the stock split limit and factor can have unintended consequences during a merge;
// multiple stock splits could occur from a single merge.
const DEFAULT_STOCK_SPLIT_LIMIT: u64 = 3000;
const DEFAULT_STOCK_SPLIT_FACTOR: i64 = 2;
const DEFAULT_DIVIDEND_PERCENTAGE: f32 = 5.0; // percent
const DEFAULT_FOUNDER_SHARES: i64 = 5;
const DEFAULT_MERGE_SHARE_CONVERSION: i64 = 2; // divisor
const DEFAULT_MERGE_BONUS_FACTOR: i64 = 10;

/// Trade Error. This happens when trying to do bad trades.
#[derive(Debug)]
pub enum TradeError {
    /// Player doesn't have enough cash to buy.
    TooLittleCash,
    /// Player doesn't have enough stock to sell.
    TooLittleStock,
}

impl std::fmt::Display for TradeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeError::TooLittleCash => write!(f, "Not enough cash"),
            TradeError::TooLittleStock => write!(f, "Not enough stock"),
        }
    }
}

impl std::error::Error for TradeError {}

/// Game state representation. The game state is moved by calling various methods.
#[derive(Debug, PartialEq)]
enum GameState {
    /// Before the game has begun. Game is ready for an [`StarLanes::init`] call.
    PreInit,

    /// Player is beginning their turn. Game is ready for a [`StarLanes::begin_turn`] call.
    BeginTurn,

    /// Player is moving. Game is ready for [`StarLanes::get_moves`] and [`StarLanes::make_move`]
    /// calls.
    Move,

    /// Player is trading in a specific company. The original game only allowed you to trade
    /// companies in order. Game is ready for [`StarLanes::trade`] call.
    Trade(usize),

    /// Player is trading arbitrary companies in any order. Game is ready for [`StarLanes::trade`]
    /// call.
    //FreeTrade,

    /// Player has completed their turn. Game is ready for an [`StarLanes::end_turn`] call.
    EndTurn,

    /// Game is over.
    GameOver,
}

use GameState::*;

/// Main game structure.
#[derive(Debug)]
pub struct StarLanes {
    /// The game map.
    pub map: Map,

    /// Current game turn count. When it reaches a limit, the game is over.
    turn_number: usize,

    /// Current game state.
    state: GameState,

    /// Current player's number.
    current_player: usize,

    /// List of player information in the game.
    players: Vec<Player>,

    /// The maximum number of companies that can exist in this game.
    max_company_count: usize,

    /// A list of companies, both extant and not.
    companies: Vec<Company>,

    /// Potential moves the current player can make this turn.
    candidate_moves: Vec<Point>,

    /// Various game options
    pub options: StarLanesOptions,
}

impl Default for StarLanes {
    /// Creates a new default game.
    fn default() -> Self {
        Self::new()
    }
}

/// Holds options for the game.
#[derive(Debug)]
pub struct StarLanesOptions {
    /// Number of players in the game.
    pub player_count: usize,

    /// Wizard (cheating/debugging) mode.
    pub wizard_mode: bool,

    /// Maximum number of turns in a game.
    pub max_turns: usize,
}

impl StarLanesOptions {
    /// Construct a new StarLanesOptions object.
    pub fn new() -> Self {
        Self {
            player_count: 0,
            wizard_mode: false,
            max_turns: DEFAULT_MAX_TURNS,
        }
    }
}

impl Default for StarLanesOptions {
    /// Creates a new default set of options.
    fn default() -> Self {
        Self::new()
    }
}

/// Information about the neighbors of a particular map cell. This is used when coming up with
/// candidate moves and determining the results of a particular player move.
///
/// Neighbors are orthogonal from the given spot. Out-of-bounds cells are not considered.
#[derive(Debug)]
pub(crate) struct NeighborCounts {
    /// How many neighbors are empty space.
    spaces: usize,

    /// How many neighbors are stars.
    stars: usize,

    /// A coordinate list of neighboring unaffiliated outposts.
    outposts: Vec<Point>,

    /// A coordinate list of neighboring companies.
    companies: Vec<Point>,

    /// How many different companies are neighbors.
    discrete_companies: usize,

    /// True if there is only empty space around the cell.
    only_space: bool,

    /// True if there are only stars, outposts, or empty space around the cell.
    only_stars_outposts: bool,
}

impl StarLanes {
    /// Create a new partially-initialized game object. See [`reset`] for completing
    /// initialization.
    ///
    /// [`reset`]: Self::reset
    pub fn new() -> Self {
        Self::new_with_options(StarLanesOptions::new())
    }

    /// Create a new partially-initialized game object with options. See [`reset`] for completing
    /// initialization.
    ///
    /// [`reset`]: Self::reset
    pub fn new_with_options(options: StarLanesOptions) -> Self {
        StarLanes {
            map: Map::new(),
            state: PreInit,
            current_player: 0,
            turn_number: 0,
            players: Vec::new(),
            max_company_count: DEFAULT_MAX_COMPANY_COUNT,
            companies: Vec::new(),
            candidate_moves: Vec::new(),
            options,
        }
    }

    /// Reset this game object to the start of the game.
    pub fn reset(&mut self) {
        let mut rng = rand::rng();

        if self.state != PreInit && self.state != GameOver {
            panic!("init: invalid state: {:#?}", self.state);
        }

        self.map.regenerate();

        if !(1..=4).contains(&self.options.player_count) {
            panic!("invalid player count");
        }

        self.turn_number = 0;

        self.current_player = rng.random_range(0..self.options.player_count);
        self.players.clear();
        for _ in 0..self.options.player_count {
            self.players.push(Player::new());
        }

        self.companies.clear();

        for _ in 0..DEFAULT_MAX_COMPANY_COUNT {
            let mut c = Company::new();
            c.init();
            self.companies.push(c);
        }

        self.state = BeginTurn;
    }

    /// Returns the index of the current player.
    pub fn get_current_player_index(&self) -> usize {
        self.current_player
    }

    /// Return a particular player at an index.
    pub fn get_player(&self, player_num: usize) -> &Player {
        &self.players[player_num]
    }

    /// Returns a reference to the current player object.
    pub fn get_current_player(&self) -> &Player {
        self.get_player(self.current_player)
    }

    /// Returns a reference to a company.
    pub fn get_company(&self, co_num: usize) -> &Company {
        &self.companies[co_num]
    }

    /// Returns a reference to all the companies.
    pub fn get_companies(&self) -> &Vec<Company> {
        &self.companies
    }

    /// Start the turn. This should be called from the UI.
    pub fn begin_turn(&mut self) {
        if self.state != BeginTurn {
            panic!("begin_turn: invalid state: {:#?}", self.state);
        }

        self.candidate_moves.clear();

        self.state = Move;
    }

    /// Assess the neighbors of a particular location on the map.
    fn neighbor_count(&self, at_row: usize, at_col: usize) -> NeighborCounts {
        let mut result = NeighborCounts {
            spaces: 0,
            stars: 0,
            outposts: Vec::new(),
            companies: Vec::new(),
            discrete_companies: 0,
            only_space: false,
            only_stars_outposts: false,
        };

        // To match the original game for merge tie resolution, this MUST be in the order N, S, E,
        // W:
        let offsets: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, 1], [0, -1]];

        let mut company_count: HashMap<MapCell, usize> = HashMap::new();

        for [roffset, coffset] in offsets {
            let row = at_row as i32 + roffset;
            let col = at_col as i32 + coffset;

            if row < 0 || row >= self.map.height as i32 {
                continue;
            }

            if col < 0 || col >= self.map.width as i32 {
                continue;
            }

            match self.map.get(row as usize, col as usize) {
                MapCell::Space => result.spaces += 1,
                MapCell::Star => result.stars += 1,
                MapCell::Outpost => result.outposts.push(Point(row as usize, col as usize)),
                MapCell::Company(i) => {
                    *company_count.entry(MapCell::Company(i)).or_insert(0) += 1;
                    result.companies.push(Point(row as usize, col as usize));
                }
            }
        }

        result.discrete_companies = company_count.len();
        result.only_space =
            result.stars == 0 && result.outposts.is_empty() && result.companies.is_empty();
        result.only_stars_outposts =
            (result.stars > 0 || !result.outposts.is_empty()) && result.companies.is_empty();

        result
    }

    /// Return the number of companies that are currently active in the game.
    fn active_company_count(&self) -> usize {
        let mut count: usize = 0;

        for c in &self.companies {
            if c.in_use {
                count += 1;
            }
        }

        count
    }

    /// Return true if there are inactive companies available to be formed.
    fn companies_available(&self) -> bool {
        self.active_company_count() < self.max_company_count
    }

    /// Return true if the game is over.
    pub fn game_is_over(&self) -> bool {
        self.state == GameOver
    }

    /// Get the candidate moves for a particular player.
    ///
    /// In the standard game, it's incredibly probable that there will be enough moves available
    /// (i.e. there aren't too many filled spots to find enough valid moves).
    ///
    /// **However**, if enough candidate moves cannot be found, the game will be over. This must be
    /// checked by the UI via [`game_is_over`].
    ///
    /// [`game_is_over`]: Self::game_is_over
    pub fn get_moves(&mut self) -> Vec<Point> {
        if self.state != Move {
            panic!("get_moves: invalid state: {:#?}", self.state);
        }

        let mut candidates: Vec<Point> = Vec::new();

        // If we've already generated the moves this turn, just return them
        if !self.candidate_moves.is_empty() {
            candidates.extend(self.candidate_moves.iter().cloned());
            return candidates;
        }

        // Loop through map getting candidate moves

        for r in 0..self.map.height {
            for c in 0..self.map.width {
                let mapcell = self.map.get(r, c);

                if mapcell != MapCell::Space {
                    continue;
                }

                let neighbors = self.neighbor_count(r, c);

                if !self.companies_available()
                    && neighbors.companies.is_empty()
                    && (!neighbors.outposts.is_empty() || neighbors.stars > 0)
                {
                    continue;
                }

                candidates.push(Point(r, c));
            }
        }

        let mut rng = rand::rng();

        candidates.shuffle(&mut rng);

        // Check if not enough legal moves remaining on board-- this would cause an early
        // game-over.
        if candidates.len() < DEFAULT_CANDIDATE_MOVE_COUNT {
            candidates.truncate(0);
            self.state = GameOver;
            return candidates;
        }

        candidates.truncate(DEFAULT_CANDIDATE_MOVE_COUNT);

        // Keep a copy for us to use later
        self.candidate_moves.extend(candidates.iter().cloned());

        candidates
    }

    /// Form and initialize a new company. This function assumes there are companies available.
    fn form_company(&mut self) -> usize {
        let company_opt = self
            .companies
            .iter_mut()
            .enumerate()
            .find(|(_, c)| !c.in_use);

        let (co_num, company) =
            company_opt.expect("trying to form a company but all companies are in use!");

        company.in_use = true;
        company.size = 1;
        company.share_price = DEFAULT_GROWTH_PRICE_BOOST;

        // Set all player holdings to 0, except the founding player.
        for (i, p) in self.players.iter_mut().enumerate() {
            // Award shares to founding player
            let holdings = if i == self.current_player {
                DEFAULT_FOUNDER_SHARES
            } else {
                0
            };
            p.set_holdings(co_num, holdings);
        }

        co_num
    }

    /// Grow a particular company by one space.
    fn grow_company(&mut self, co_num: usize) {
        let company = &mut self.companies[co_num];

        company.size += 1;
        company.share_price += DEFAULT_GROWTH_PRICE_BOOST;
    }

    /// Do cleanup after forming or growing a company.
    ///
    /// This figures out the stock price increases due to neighboring stars, and absorbs nearby
    /// outposts.
    ///
    /// It also checks if a stock split occurs.
    ///
    /// This should **not** be called if there was a merge; merging handles its own cleanup.
    fn tidy_company(
        &mut self,
        co_num: usize,
        move_point: Point,
        neighbors: &NeighborCounts,
        events: &mut Vec<Event>,
    ) {
        let company = &mut self.companies[co_num];

        company.share_price += DEFAULT_STAR_PRICE_BOOST * neighbors.stars as u64;

        company.share_price += DEFAULT_OUTPOST_PRICE_BOOST * neighbors.outposts.len() as u64;
        for Point(row, col) in &neighbors.outposts {
            self.map.set(*row, *col, MapCell::Company(co_num as u32));
        }

        let Point(row, col) = move_point;

        self.map.set(row, col, MapCell::Company(co_num as u32));

        self.stock_split(co_num, events);
    }

    /// Called by the player to make their move at a given point. This is validated against the
    /// move list to make sure the move is valid unless [wizard mode has been set](Self::reset). If
    /// wizard mode is set, this will panic if a move is made off the map.
    pub fn make_move(&mut self, move_point: Point) -> Vec<Event> {
        if self.state != Move {
            panic!("move: invalid state: {:#?}", self.state);
        }

        if !self.options.wizard_mode && !self.candidate_moves.contains(&move_point) {
            panic!("move: invalid move: {:?}", move_point);
        }

        let mut events: Vec<Event> = Vec::new();

        let Point(row, col) = move_point;

        let neighbors = self.neighbor_count(row, col);

        // println!("{:#?}", neighbors);

        if neighbors.only_space {
            self.map.set(row, col, MapCell::Outpost);
        } else if neighbors.discrete_companies > 1 {
            self.merge(move_point, &neighbors, &mut events);
        } else if neighbors.discrete_companies == 1 {
            let Some(&Point(row, col)) = neighbors.companies.first() else {
                panic!("expected there to be neighbor companies");
            };

            let MapCell::Company(n) = self.map.get(row, col) else {
                panic!("expected to find a company at {},{}", row, col);
            };

            let co_num = n as usize;

            self.grow_company(co_num);
            self.tidy_company(co_num, move_point, &neighbors, &mut events);
        } else if neighbors.only_stars_outposts {
            let co_num = self.form_company();
            self.tidy_company(co_num, move_point, &neighbors, &mut events);

            events.push(Event::CompanyFormed(co_num));
        }

        self.state = self.get_next_trade_state(0);

        self.dividends(&mut events);

        events
    }

    /// Return the next company Trade state from the current one. This is for the classic game
    /// which only allowed you to trade stocks in alphabetical order.
    fn get_next_trade_state(&self, from: usize) -> GameState {
        for (i, c) in self.companies.iter().enumerate() {
            if i >= from && c.in_use {
                return Trade(i);
            }
        }

        EndTurn
    }

    /// Trade stock in a particular company. `amount` is the number of shares, negative to sell.
    pub fn trade(&mut self, co_num: usize, amount: i64) -> Result<(), TradeError> {
        // The original game didn't check for negative values on the purchase. If this is true,
        // this game will not check, either.
        const BUG_OVERSELL: bool = true;

        if self.state != Trade(co_num) {
            panic!(
                "trade: invalid state for trading company {}: {:#?}",
                co_num, self.state
            );
        }

        let player = &mut self.players[self.current_player];
        let holdings = player.get_holdings(co_num);
        let cash = player.get_cash();

        if !BUG_OVERSELL && amount < 0 && amount.abs() > holdings {
            return Err(TradeError::TooLittleStock);
        }

        // Is there a safer way to do this?
        let cost: i64 = amount * self.companies[co_num].share_price as i64;

        if cost > 0 && cost > cash {
            return Err(TradeError::TooLittleCash);
        }

        player.add_holdings_signed(co_num, amount);
        player.add_cash(-cost);

        self.state = self.get_next_trade_state(co_num + 1);

        Ok(())
    }

    /// Called to wrap up the current player's turn.
    pub fn end_turn(&mut self) {
        if !matches!(self.state, EndTurn | Trade(_) /*| FreeTrade*/) {
            panic!("end_turn: invalid state: {:#?}", self.state);
        }

        self.turn_number += 1;

        if self.turn_number >= self.options.max_turns {
            self.state = GameOver;
            return;
        }

        self.current_player = (self.current_player + 1) % self.options.player_count;

        self.state = BeginTurn;
    }
}
