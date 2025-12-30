//! Star Lanes game library.
//!
//! [`starlanes`] is the main module.
//!
//! This library is very unforgiving to invalid use. If you try to call a function when you're not
//! supposed to or make an invalid move, it will simply panic. This is by design so that the UI
//! driver code doesn't have to error-check everything. Also, it should be easy to write and test
//! UI code that doesn't make any invalid moves.
//!
//! [More info on the game can be found
//! here](https://github.com/beejjorgensen/starlanes-info/tree/main).
//!
//! [`starlanes`]: crate::starlanes

pub mod company;
pub mod event;
pub mod map;
pub mod player;
pub mod starlanes;
