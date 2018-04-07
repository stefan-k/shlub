// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Todo

use std;
use std::io::Write;
use termion::{clear::AfterCursor, cursor::{DetectCursorPos, Goto}, input::{Keys, TermRead},
              raw::{IntoRawMode, RawTerminal}};

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

    pub fn cursor_pos(&mut self) -> (u16, u16) {
        self.stdout.cursor_pos().unwrap()
    }

    pub fn write(&mut self, text: &String) {
        write!(self.stdout, "{}", text).unwrap();
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) {
        write!(self.stdout, "{}", Goto(x, y)).unwrap();
    }

    pub fn clear_after_cursor(&mut self) {
        write!(self.stdout, "{}", AfterCursor).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn newline(&mut self) {
        write!(self.stdout, "\n").unwrap();
    }
}
