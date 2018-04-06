// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Todo

use std;
use termion::{input::{Keys, TermRead}, raw::{IntoRawMode, RawTerminal}};

pub struct Terminal {
    pub stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn new() -> Self {
        let stdout = std::io::stdout();
        // let stdout = stdout.into_raw_mode().unwrap();
        Terminal {
            stdout: stdout.into_raw_mode().unwrap(),
        }
    }

    pub fn keys(&self) -> Keys<std::io::Stdin> {
        std::io::stdin().keys()
    }
}
