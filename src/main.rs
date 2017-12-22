#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate termion;
extern crate shlub;

// #![warn(missing_docs)]
use shlub::prompt::read_line;
use shlub::utils::*;
use shlub::history::History;
use shlub::errors::*;
use termion::raw::IntoRawMode;
use std::io::{Write};

fn abort_mission() {
    // TODO: Write History

    // avoids fucked up terminal after shell has been run
    std::process::exit(0);
}

fn evaluate(cmd: &[&str], history: &History) {
    // All of this is just for testing right now.
    match cmd[0] {
        "exit" => abort_mission(),
        "cwd" | "pwd" => {
            println!("{}", cwd().display());
            ()
        }
        "listenv" => list_env(),
        "printhist" => {
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

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1))?;
    stdout.flush()?;

    // load history from file!
    // put current date as first
    let mut history = History::new();
    loop {

        let cmd = read_line(&mut history, &mut stdout).unwrap();

        let cmd_split: Vec<&str> = cmd.split(' ').collect();

        evaluate(&cmd_split, &history);
        stdout.flush()?;

    }
}
