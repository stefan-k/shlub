use ncurses::*;
use std;
use utils;
use cursor::Cursor;

struct Prompt {
    left: String,
    right: String,
    pos_left: i32,
    pos_right: i32,
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
                .join(" > ")
        );
        let prompt_right = "fu".to_owned();
        self.left = prompt_left.to_owned();
        self.right = prompt_right.to_owned();
        self.pos_left = (prompt_left.chars().count() + 1) as i32;
        self.pos_right = (prompt_right.chars().count() + 1) as i32;
        self
    }
}

struct Command {
    cmd: String,
    len: i32,
    pos: i32,
}

impl Command {
    pub fn new() -> Self {
        Command {
            cmd: "".to_owned(),
            len: 0,
            pos: 0,
        }
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

    pub fn insert(&mut self, c: i32) -> &mut Self {
        self.cmd.insert(
            self.pos as usize,
            std::char::from_u32(c as u32).unwrap(),
        );
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

fn print_all(cur_line: i32, prompt: &mut Prompt, cmd: &Command, cursor: &mut Cursor) {
    // TODO: Print right prompt and adapt drawing of command
    // move to beginning of line
    let y = cur_line;
    mv(y, 0);
    // clear line
    clrtobot();
    // update prompt
    prompt.update();
    // print prompt
    printw(&prompt.left);
    // move to end of prompt
    mv(y, prompt.pos_left);
    // print command
    printw(&cmd.cmd);
    // move cursor to previous position
    cursor.set(y, prompt.pos_left + cmd.pos);
    mv(cursor.y, cursor.x);
}

pub fn read_line() -> Result<String, std::io::Error> {
    let mut cursor = Cursor::current_pos();
    let mut cmd = Command::new();
    let mut prompt = Prompt::new();

    printw("\n");
    cursor.down();
    let cur_line = cursor.y;
    print_all(cur_line, &mut prompt, &cmd, &mut cursor);

    loop {
        match getch() {
            KEY_ENTER | KEY_BREAK | KEY_EOL | 10 => break,
            KEY_BACKSPACE => {
                cmd.remove();
                cursor.left();
            }
            KEY_LEFT => {
                cmd.left();
                cursor.left();
            }
            KEY_RIGHT => {
                cmd.right();
                cursor.right();
            }
            c => {
                cmd.insert(c);
                cursor.right();
            }
        }
        print_all(cur_line, &mut prompt, &cmd, &mut cursor);
    }

    printw("\n");
    cursor.down();
    Ok(cmd.cmd)
}
