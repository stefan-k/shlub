#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate ncurses;
extern crate shlub;

// #![warn(missing_docs)]
use ncurses::*;
use shlub::prompt::read_line;
use shlub::utils::*;
use shlub::history::History;
use shlub::errors::*;

fn abort_mission() {
    // TODO: Write History

    // avoids fucked up terminal after shell has been run
    nocbreak();
    endwin();
    std::process::exit(0);
}

fn evaluate(cmd: &[&str], history: &History) {
    // All of this is just for testing right now.
    match cmd[0] {
        "exit" => abort_mission(),
        "cwd" | "pwd" => {
            printw(format!("{}", cwd().display()).as_ref());
            ()
        }
        "listenv" => list_env(),
        "printhist" => {
            printw(&history.get_all());
            ()
        }
        "cd" => {
            // borrow checker... really ugly. needs to be cleaned up!
            let home_dir = user_home_dir().into_os_string().into_string().unwrap();
            let new_dir = if cmd.len() > 1 { cmd[1] } else { &home_dir };
            if let Err(e) = chdir(new_dir) {
                printw(format!("{}", e).as_ref());
            };
        }
        _ => {
            printw(format!("back: {}", cmd.join(" ")).as_ref());
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
    // setlocale(LcCategory::all, "gb_EN.UTF-8");

    // Start ncurses
    initscr();
    // raw();
    cbreak();

    // allow for extended keyboard
    keypad(stdscr(), true);
    noecho();

    // allow scrolling
    scrollok(stdscr(), true);

    // print to the back buffer
    printw("shlub\n");

    // update the screen
    refresh();

    // load history from file!
    // put current date as first
    let mut history = History::new();
    loop {

        let cmd = read_line(&mut history).unwrap();

        let cmd_split: Vec<&str> = cmd.split(' ').collect();

        evaluate(&cmd_split, &history);

        refresh();
    }
}
