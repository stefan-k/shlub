#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate ncurses;

pub mod errors;
pub mod utils;
pub mod history;
pub mod cursor;
pub mod prompt;
