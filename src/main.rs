// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # shlub

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
extern crate shlub;
extern crate termion;

// #![warn(missing_docs)]
use shlub::{cursor::Cursor, errors::*, history::History, prompt::read_line,
            terminal::termion::Terminal, utils::*};
use std::io::{Stdout, Write};

// fn abort_mission(stdout: &mut Stdout) {
fn abort_mission(stdout: &mut termion::raw::RawTerminal<Stdout>) {
    // TODO: Write History
    stdout.flush().unwrap();
    drop(stdout);
    std::process::exit(0);
}

fn evaluate(
    cmd: &[&str],
    history: &History,
    stdout: &mut termion::raw::RawTerminal<Stdout>,
) -> Result<()> {
    // All of this is just for testing right now.
    match cmd[0] {
        "exit" => abort_mission(stdout),
        "cwd" | "pwd" => {
            println!("{}", cwd().display());
            ()
        }
        "listenv" => list_env(),
        "printhist" => {
            // TODO: needs to take care of cursors...
            println!("{}", &history.get_all());
            ()
        }
        "cd" => {
            // borrow checker... really ugly. needs to be cleaned up!
            let home_dir = user_home_dir().into_os_string().into_string().unwrap();
            let new_dir = if cmd.len() > 1 { cmd[1] } else { &home_dir };
            if let Err(e) = chdir(new_dir) {
                println!("{}", e);
            };
        }
        _ => {
            println!("back: {}", cmd.join(" "));
            ()
        }
    };
    stdout.flush()?;
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("Error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("Backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut term = Terminal::new();
    let mut cursor = Cursor::current_pos(&mut term);

    // load history from file!
    // put current date as first
    let mut history = History::new();
    loop {
        let cmd = read_line(&mut history, &mut cursor, &mut term)?;

        let cmd_split: Vec<&str> = cmd.split(' ').collect();

        evaluate(&cmd_split, &history, &mut term.stdout)?;
    }
}
