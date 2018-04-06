// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Todo

use std;
use errors::*;

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
        print!("{}: {}\n", key, value);
    }
}

pub fn chdir(dir: &str) -> Result<()> {
    let path = std::path::Path::new(dir);

    if !path.exists() {
        return Err("Directory doesn't exist.".into());
    }

    if !path.is_dir() {
        return Err("Directory is not a directory. wut?".into());
    }
    if path.is_file() {
        return Err("Not a directory.".into());
    }

    std::env::set_current_dir(&path).is_ok();
    Ok(())
}
