use std;
use ncurses::*;

pub fn cwd() -> std::path::PathBuf {
    std::env::current_dir().unwrap()
}

pub fn user_home_dir() -> std::path::PathBuf {
    match std::env::home_dir() {
        Some(path) => path,
        None => cwd(),
    }
}

pub fn list_env() {
    for (key, value) in std::env::vars() {
        printw(format!("{}: {}\n", key, value).as_ref());
    }
}

pub fn chdir(dir: &str) -> Result<(), &'static str> {
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
