// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Todo

use termion;
use terminal::termion::Terminal;
use errors::*;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
    max_x: u16,
    max_y: u16,
}

fn get_screen_max() -> Result<(u16, u16)> {
    // I need to move on. fix this later
    Ok(termion::terminal_size().unwrap())
}

impl Cursor {
    pub fn current_pos(term: &mut Terminal) -> Self {
        let (max_x, max_y) = get_screen_max().unwrap();
        // This eats the first byte, see #101 and #136 in termion repository
        let (cx, cy) = term.cursor_pos();
        Cursor {
            x: cx,
            y: cy,
            max_x: max_x,
            max_y: max_y,
        }
    }

    pub fn set(&mut self, x: u16, y: u16) -> &mut Self {
        self.update_max();
        self.y = y + (x - (x % self.max_x)) / self.max_x;
        self.x = x % self.max_x;
        self
    }

    pub fn update_max(&mut self) -> &mut Self {
        let (max_x, max_y) = get_screen_max().unwrap();
        self.max_y = max_y;
        self.max_x = max_x;
        self
    }

    pub fn left(&mut self) -> &mut Self {
        self.update_max();
        if self.x > 1 {
            self.x -= 1;
        } else {
            self.x = self.max_x;
            self.up();
        }
        self
    }

    pub fn right(&mut self) -> &mut Self {
        self.update_max();
        if self.x < self.max_x {
            self.x += 1;
        } else {
            self.x = 1;
            self.down();
        }
        self
    }

    pub fn up(&mut self) -> &mut Self {
        self.y -= 1;
        self
    }

    pub fn down(&mut self) -> &mut Self {
        if self.y + 1 > self.max_y {
            print!("{}", termion::scroll::Down(1));
        } else {
            self.y += 1;
        }
        self
    }

    pub fn pos_0(&mut self) -> &mut Self {
        self.x = 0;
        self
    }
}
