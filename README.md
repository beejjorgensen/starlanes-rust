# Star Lanes, Rust Port

This is a port of the classic BASIC game [Star
Lanes](https://github.com/beejjorgensen/starlanes-info) to Rust.

## Usage

This is a library and binary package.

To run the game with the classic interface with just a few minor changes:

```
cargo run
```

## The Library

The library portion has been written with the intent that front-ends are
relatively easy to bolt on.

`main.rs` is an example.

## License

All my code is Unlicensed. However, the game itself contains text that
is copyrighted by the original author. I plead fair use for that, but if
you're still out there, [Steven
Faber](https://www.mobygames.com/person/635973/steven-faber/credits/),
drop me a line.

```
THE GAME OF STAR LANES - AN INTERSTELLAR COMMERCE GAME
FOR 2-4 PLAYERS - COPYRIGHT 1977 BY STEVEN FABER
WRITTEN IN ALTAIR BASIC 12/17/76
```

## TODO

### For reimplementation

* Pass config object to `StarLanes` constructor
  * Bug flags
  * Everything tuneable
* Check for other fields that should be private

### Additional features

* Any-order trading
* General standings page (was this in the Osborne version and not the
  original?)
* Black holes
* Variable map sizes
* More companies
* Tunable parameters (all those `const`s)

### Later, in other packages

* [Ratatui](https://crates.io/crates/ratatui) front-end
* Web front-end for a WASM build

## Author

Brian "Beej Jorgensen" Hall\
[beej.us](https://beej.us/)\
[beej@beej.us](mailto:beej@beej.us)

