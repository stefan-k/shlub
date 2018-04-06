// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Todo

#![feature(slice_patterns)]
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate termion;

pub mod errors;
pub mod utils;
pub mod history;
pub mod cursor;
pub mod prompt;
