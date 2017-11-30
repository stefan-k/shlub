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
    // refresh();
}

fn print_cmd(cmd: &str, init_y: i32, init_x: i32) {
    mv(init_y, init_x);
    clrtoeol();
    printw(&cmd);
    mv(init_y, init_x + cmd.chars().count() as i32);
}

fn read_line() -> Result<String, std::io::Error> {
    let mut cmd = String::from("");
    let mut pos: usize = 0;
    let mut init_x = 0;
    let mut init_y = 0;
    getyx(stdscr(), &mut init_y, &mut init_x);
    loop {
        match getch() {
            KEY_ENTER | KEY_BREAK | KEY_EOL | 10 => break,
            KEY_BACKSPACE => {
                pos = if pos > 0 {
                    if pos == cmd.chars().count() {
                        cmd.pop();
                    } else {
                        cmd.remove(pos as usize);
                    }
                    print_cmd(&cmd, init_y, init_x);
                    pos - 1
                } else {
                    0
                };
            }
            c => {
                pos += 1;
                cmd.push(std::char::from_u32(c as u32).unwrap());
                print_cmd(&cmd, init_y, init_x);
            }
        }
    }
    Ok(cmd)
}

fn list_env() {
    for (key, value) in std::env::vars() {
        printw(format!("{}: {}\n", key, value).as_ref());
    }
}

fn push_history(line: &str, history: &mut Vec<String>) {
    history.push(line.to_owned());
}

fn print_history(history: &[String]) {
    for line in history.iter() {
        printw(format!("{}", line).as_ref());
    }
}

fn abort_mission() {
    // TODO: Write History

    // avoids fucked up terminal after shell has been run
    nocbreak();
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

    // allow scrolling
    scrollok(stdscr(), true);

    // print to the back buffer
    printw("shlub\n");

    // update the screen
    refresh();

    let mut cx = 0;
    let mut cy = 0;

    // load history from file!
    // put current date as first
    let mut history: Vec<String> = vec!["BLA".to_owned()];
    loop {
        // borrow checker... really ugly. needs to be cleaned up!
        let home_dir = user_home_dir().into_os_string().into_string().unwrap();

        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        getyx(stdscr(), &mut cy, &mut cx);
        if cy + 1 > max_y - 1 {
            wscrl(stdscr(), 1);
            // why tho
            mv(max_y - 2, 0);
        }

        printw("\n");
        prompt();

        let cmd = read_line().unwrap();
        printw("\n");

        getyx(stdscr(), &mut cy, &mut cx);
        if cy + 1 > max_y - 1 {
            wscrl(stdscr(), 1);
            // why tho
            mv(max_y - 2, 0);
        }

        push_history(&cmd, &mut history);

        // All of this is just for testing right now.
        let cmd_split: Vec<&str> = cmd.split(' ').collect();
        match cmd_split[0] {
            "exit" => break,
            "cwd" | "pwd" => {
                printw(format!("{}", cwd().display()).as_ref());
                ()
            }
            "listenv" => list_env(),
            "printhist" => print_history(&history),
            "cd" => {
                let new_dir = if cmd_split.len() > 1 {
                    cmd_split[1]
                } else {
                    &home_dir
                };
                if let Err(e) = chdir(new_dir) {
                    printw(format!("{}", e).as_ref());
                };
            }
            _ => {
                printw(format!("back: {}", cmd).as_ref());
                ()
            }
        };
        refresh();
    }
    abort_mission();
}
