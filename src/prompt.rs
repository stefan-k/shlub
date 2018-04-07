// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Todo

use std;
use utils;
use termion;
use termion::event::Key;
use terminal::termion::Terminal;
use std::io::Write;
use cursor::Cursor;
use history::History;
use errors::*;

#[derive(PartialEq, Clone)]
enum State {
    NORMAL,
    INSERT,
    // REPLACE,
}

struct Prompt {
    left: String,
    right: String,
    pos_left: u16,
    pos_right: u16,
}

impl Prompt {
    pub fn new() -> Self {
        Prompt {
            left: "".to_owned(),
            right: "".to_owned(),
            pos_left: 0,
            pos_right: 0,
        }
    }

    pub fn update(&mut self) -> &mut Self {
        let username = std::env::var("USER").unwrap();
        let cwd = utils::cwd();
        let prompt_left = format!(
            "{} | {} > ",
            username,
            cwd.into_iter()
                .map(|a| a.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join("/")
        );
        let prompt_right = "fu".to_owned();
        self.left = prompt_left.to_owned();
        self.right = prompt_right.to_owned();
        self.pos_left = (prompt_left.chars().count() + 1) as u16;
        self.pos_right = (prompt_right.chars().count() + 1) as u16;
        self
    }
}

struct Command {
    cmd: String,
    len: u16,
    pos: u16,
}

impl Command {
    pub fn new() -> Self {
        Command {
            cmd: "".to_owned(),
            len: 0,
            pos: 0,
        }
    }

    pub fn set(&mut self, cmd: String) -> &mut Self {
        self.cmd = cmd;
        let cmd_len = self.cmd.chars().count() as u16;
        self.pos = cmd_len;
        self.len = cmd_len;
        self
    }

    pub fn left(&mut self) -> &mut Self {
        if self.pos > 0 {
            self.pos -= 1;
        }
        self
    }

    pub fn right(&mut self) -> &mut Self {
        if self.pos < self.len {
            self.pos += 1;
        }
        self
    }

    pub fn to_end(&mut self) -> &mut Self {
        self.pos = self.len;
        self
    }

    pub fn insert(&mut self, c: char) -> &mut Self {
        self.cmd.insert(self.pos as usize, c);
        self.len += 1;
        self.pos += 1;
        self
    }

    pub fn remove(&mut self) -> &mut Self {
        if self.pos > 0 {
            if self.pos == self.len {
                self.cmd.pop();
            } else {
                self.cmd.remove(self.pos as usize);
            }
            self.len -= 1;
            self.pos -= 1;
        }
        self
    }
}

fn print_all(
    cur_line: u16,
    prompt: &mut Prompt,
    cmd: &Command,
    cursor: &mut Cursor,
    term: &mut Terminal,
) {
    // TODO: Print right prompt and adapt drawing of command
    prompt.update();
    write!(
        term.stdout,
        "{}{}{}{}{}",
        termion::cursor::Goto(1, cur_line),
        termion::clear::AfterCursor,
        &prompt.left,
        termion::cursor::Goto(prompt.pos_left, cur_line),
        &cmd.cmd
    ).unwrap();
    cursor.set(prompt.pos_left + cmd.pos, cur_line);
    write!(term.stdout, "{}", termion::cursor::Goto(cursor.x, cursor.y)).unwrap();
    term.stdout.flush().unwrap();
}

pub fn read_line(
    history: &mut History,
    cursor: &mut Cursor,
    term: &mut Terminal,
) -> Result<String> {
    cursor.update_pos(term);
    let mut cmd = Command::new();
    let mut prompt = Prompt::new();

    let mut state = State::INSERT;

    let mut stack = vec![];

    print_all(cursor.y, &mut prompt, &cmd, cursor, term);

    for ch in term.keys() {
        let c = ch.unwrap();
        if let Key::Char(cc) = c {
            stack.push(cc);
        }
        match (state.clone(), c, stack.as_slice()) {
            (State::INSERT, Key::Esc, _) => {
                stack.clear();
                state = State::NORMAL;
                ()
            }
            (_, Key::Char('\n'), _) | (_, _, &['\n']) => {
                // in case the cursor is not at the end of the line when pressing return, the
                // cursor has to be moved to the end of the command and the command needs to be
                // printed again. otherwise, everything after the cursor will vanish.
                cmd.to_end();
                stack.clear();
                break;
            }
            (_, Key::Backspace, _) | (_, Key::Ctrl('h'), _) => {
                cmd.remove();
                cursor.left();
                stack.clear();
            }
            (_, Key::Left, _) | (State::NORMAL, _, &['h']) => {
                cmd.left();
                cursor.left();
                stack.clear();
            }
            (_, Key::Right, _) | (State::NORMAL, _, &['l']) => {
                cmd.right();
                cursor.right();
                stack.clear();
            }
            (_, Key::Up, _) | (State::NORMAL, _, &['k']) => {
                // TODO: Stash the previous command!
                if let Some(s) = history.backwards() {
                    cmd.set(s);
                };
                stack.clear();
            }
            (_, Key::Down, _) | (State::NORMAL, _, &['j']) => {
                match history.forward() {
                    Some(s) => cmd.set(s),
                    None => cmd.set("".to_owned()),
                };
                stack.clear();
            }
            (State::NORMAL, _, &['i']) => {
                state = State::INSERT;
                stack.clear();
            }
            (State::NORMAL, _, &['a']) => {
                state = State::INSERT;
                cursor.right();
                cmd.right();
                stack.clear();
            }
            (State::NORMAL, _, &[c]) => {
                stack.push(c);
            }
            (State::NORMAL, _, &[]) => {}
            (State::NORMAL, _, &[_, _..]) => {}
            (State::INSERT, Key::Char(c), _) => {
                cmd.insert(c);
                cursor.right();
                stack.clear();
            }
            (_, _, _) => {}
        }
        print_all(cursor.y, &mut prompt, &cmd, cursor, term);
    }

    // print again to avoid printing \n in the middle of a command
    print_all(cursor.y, &mut prompt, &cmd, cursor, term);

    cursor.pos_0();
    write!(
        term.stdout,
        "{}\n",
        termion::cursor::Goto(cursor.x, cursor.y)
    )?;
    term.stdout.flush()?;
    history.push(&cmd.cmd);
    Ok(cmd.cmd)
}
