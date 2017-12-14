use ncurses::*;
use std;
use utils;
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

    pub fn set(&mut self, cmd: String) -> &mut Self {
        self.cmd = cmd;
        let cmd_len = self.cmd.chars().count() as i32;
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

pub fn read_line(history: &mut History) -> Result<String> {
    let mut cursor = Cursor::current_pos();
    let mut cmd = Command::new();
    let mut prompt = Prompt::new();

    let mut state = State::INSERT;

    printw("\n");
    cursor.down();
    let cur_line = cursor.y;
    print_all(cur_line, &mut prompt, &cmd, &mut cursor);

    loop {
        let ch = getch();
        match (state.clone(), ch, std::char::from_u32(ch as u32).unwrap()) {
            (State::INSERT, 27, _) => {
                state = State::NORMAL;
                ()
            }
            (_, KEY_ENTER, _) |
            (_, KEY_BREAK, _) |
            (_, KEY_EOL, _) |
            (_, _, '\n') => {
                // in case the cursor is not at the end of the line when pressing return, the
                // cursor has to be moved to the end of the command and the command needs to be
                // printed again. otherwise, everything after the cursor will vanish.
                cmd.to_end();
                break;
            }
            (_, KEY_BACKSPACE, _) => {
                cmd.remove();
                cursor.left();
            }
            (_, KEY_LEFT, _) |
            (State::NORMAL, _, 'h') => {
                cmd.left();
                cursor.left();
            }
            (_, KEY_RIGHT, _) |
            (State::NORMAL, _, 'l') => {
                cmd.right();
                cursor.right();
            }
            (_, KEY_UP, _) |
            (State::NORMAL, _, 'k') => {
                // TODO: Stash the previous command!
                if let Some(s) = history.backwards() {
                    cmd.set(s);
                };
            }
            (_, KEY_DOWN, _) |
            (State::NORMAL, _, 'j') => {
                match history.forward() {
                    Some(s) => cmd.set(s),
                    None => cmd.set("".to_owned()),
                };
            }
            (State::NORMAL, _, 'i') => {
                state = State::INSERT;
            }
            (State::NORMAL, _, 'a') => {
                state = State::INSERT;
                cursor.right();
                cmd.right();
            }
            (State::NORMAL, _, _) => {}
            (State::INSERT, c, _) => {
                cmd.insert(c);
                cursor.right();
            }
        }
        print_all(cur_line, &mut prompt, &cmd, &mut cursor);
    }

    // print again to avoid printing \n in the middle of a command
    print_all(cur_line, &mut prompt, &cmd, &mut cursor);

    printw("\n");
    cursor.down();
    history.push(&cmd.cmd);
    Ok(cmd.cmd)
}
