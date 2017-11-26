#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
// #![warn(missing_docs)]
use std::io::Write;

fn cwd() -> std::path::PathBuf {
    std::env::current_dir().unwrap()
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

fn abort_mission() {
    std::process::exit(0);
}

fn chdir(dir: &str) -> Result<(), &'static str> {
    let path = std::path::Path::new(dir);

    if !path.exists() {
        return Err("Directory doesn't exists.");
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
    loop {
        prompt();

        let cmd = read_line().unwrap();

        // All of this is just for testing right now.
        match cmd.as_ref() {
            "exit" => break,
            "cwd" => println!("{}", cwd().display()),
            "listenv" => list_env(),
            _ => {}
        };

        let cmd_split: Vec<&str> = cmd.split(' ').collect();

        match cmd_split[0] {
            "cd" => {
                if let Err(e) = chdir(cmd_split[1]) {
                    println!("{}", e);
                };
            }
            _ => println!("back: {}", cmd),
        }
    }
    abort_mission();
}
