use ncurses::*;

pub fn push_history(line: &str, history: &mut Vec<String>) {
    history.push(line.to_owned());
}

pub fn print_history(history: &[String]) {
    for line in history.iter() {
        // TODO: Last one doesn't require \n
        printw(format!("{}\n", line).as_ref());
    }
}
