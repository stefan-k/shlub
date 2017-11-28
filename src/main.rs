#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate ncurses;

// #![warn(missing_docs)]
// use std::io::Write;
use ncurses::*;

fn cwd() -> std::path::PathBuf {
    std::env::current_dir().unwrap()
}

fn user_home_dir() -> std::path::PathBuf {
    match std::env::home_dir() {
        Some(path) => path,
        None => cwd(),
    }
}

fn prompt() {
    // TODO: hostname
    let username = std::env::var("USER").unwrap();
    let cwd = cwd();
    let prompt = format!(
        "{} | {} > ",
        username,
        cwd.into_iter()
            .map(|a| a.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(" > ")
    );

    printw(&prompt);
    refresh();
}

fn read_line() -> Result<String, std::io::Error> {
    let mut cmd = String::from("");
    let mut pos: u32 = 0;
    loop {
        // match std::char::from_u32(getch() as u32) {
        //     Some('\n') => break,
        //     Some('\u{0008}') => {
        //         printw("fu");
        //         // pos -= 1;
        //     }
        //     Some(c) => {
        //         pos += 1;
        //         printw(&c.to_string());
        //         cmd.push(c);
        //     }
        //     None => {}
        // }
        match getch() {
            KEY_ENTER | KEY_BREAK | KEY_EOL | 10 => break,
            KEY_BACKSPACE => {
                printw("fu");
                // pos -= 1;
            }
            c => {
                pos += 1;
                // printw(format!("|{}|", c).as_ref());
                printw(&std::char::from_u32(c as u32).unwrap().to_string());
                cmd.push(std::char::from_u32(c as u32).unwrap());
            }
        }
    }
    Ok(cmd)
}

fn list_env() {
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}

fn push_history(line: &str, history: &mut Vec<String>) {
    history.push(line.to_owned());
}

fn print_history(history: &[String]) {
    for line in history.iter() {
        println!("{}", line);
    }
}

fn abort_mission() {
    // TODO: Write History
    endwin();
    std::process::exit(0);
}

fn chdir(dir: &str) -> Result<(), &'static str> {
    let path = std::path::Path::new(dir);

    if !path.exists() {
        return Err("Directory doesn't exist.");
    }

    if !path.is_dir() {
        return Err("Directory is not a directory. wut?");
    }
    if path.is_file() {
        return Err("Not a directory.");
    }

    std::env::set_current_dir(&path).is_ok();
    Ok(())
}

fn main() {
    // setlocale(LcCategory::all, "gb_EN.UTF-8");

    // Start ncurses
    initscr();
    // raw();
    cbreak();

    // allow for extended keyboard
    keypad(stdscr(), true);
    noecho();

    // print to the back buffer
    printw("Hello, world!\n");

    // update the screen
    refresh();

    // load history from file!
    // put current date as first
    let mut history: Vec<String> = vec!["BLA".to_owned()];
    loop {
        // borrow checker... really ugly. needs to be cleaned up!
        let home_dir = user_home_dir().into_os_string().into_string().unwrap();

        prompt();

        let cmd = read_line().unwrap();

        push_history(&cmd, &mut history);

        // All of this is just for testing right now.
        let cmd_split: Vec<&str> = cmd.split(' ').collect();
        match cmd_split[0] {
            "exit" => break,
            "cwd" | "pwd" => println!("{}", cwd().display()),
            "listenv" => list_env(),
            "printhist" => print_history(&history),
            "cd" => {
                let new_dir = if cmd_split.len() > 1 {
                    cmd_split[1]
                } else {
                    &home_dir
                };
                if let Err(e) = chdir(new_dir) {
                    println!("{}", e);
                };
            }
            _ => println!("back: {}", cmd),
        };
    }
    abort_mission();
}
