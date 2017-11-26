#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
// #![warn(missing_docs)]
use std::io::Write;

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
    // print!("{} | {} > ", username, cwd.display());
    print!(
        "{} | {} > ",
        username,
        cwd.into_iter()
            .map(|a| a.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(" > ")
    );
}

fn read_line() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    let _ = std::io::stdout().flush();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(String::from(buffer.trim()))
}

fn list_env() {
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}

fn push_history(line: &String, history: &mut Vec<String>) {
    history.push(line.clone());
}

fn print_history(history: &Vec<String>) {
    for line in history.iter() {
        println!("{}", line);
    }
}

fn abort_mission() {
    // TODO: Write History
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
    println!("Hello, world!");
    // load history from file!
    // put current date as first
    let mut history: Vec<String> = vec![String::from("BLA")];
    loop {
        // borrow checker... really ugly. needs to be cleaned up!
        let home_dir = user_home_dir().into_os_string().into_string().unwrap();
        prompt();

        let cmd = read_line().unwrap();

        push_history(&cmd, &mut history);

        // All of this is just for testing right now.
        let cmd_split: Vec<&str> = cmd.split(' ').collect();
        match cmd_split[0] {
            // match cmd.as_ref() {
            "exit" => break,
            "cwd" => println!("{}", cwd().display()),
            "listenv" => list_env(),
            "printhist" => print_history(&history),
            "cd" => {
                let new_dir;
                if cmd_split.len() > 1 {
                    new_dir = cmd_split[1];
                } else {
                    // `cd` without path should move to home directory.
                    new_dir = &home_dir;
                }
                if let Err(e) = chdir(new_dir) {
                    println!("{}", e);
                };
            }
            _ => println!("back: {}", cmd),
        };
    }
    abort_mission();
}
