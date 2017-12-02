use ncurses::*;
use std;
use utils;

fn print_cmd(cmd: &str, init_y: i32, init_x: i32) {
    mv(init_y, init_x);
    clrtoeol();
    printw(&cmd);
    mv(init_y, init_x + cmd.chars().count() as i32);
}

fn prompt() {
    // TODO: hostname
    let username = std::env::var("USER").unwrap();
    let cwd = utils::cwd();
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

pub fn read_line() -> Result<String, std::io::Error> {
    let mut cx = 0;
    let mut cy = 0;
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
    printw("\n");
    getyx(stdscr(), &mut cy, &mut cx);
    if cy + 1 > max_y - 1 {
        wscrl(stdscr(), 1);
        // why tho
        mv(max_y - 2, 0);
    }
    Ok(cmd)
}
