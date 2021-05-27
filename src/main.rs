use parser::i3Config;
use shellexpand::{env_with_context_no_errors, tilde};
use std::{
    borrow::Cow::Borrowed,
    env::var,
    path::{Path, PathBuf},
};

mod parser;

fn context(s: &str) -> Option<String> {
    match var(s) {
        Ok(val) => Some(val),
        Err(_err) => Some("INVALID_VAR".to_string()),
    }
}

fn find_config_file() -> PathBuf {
    let possible_paths = [
        tilde("~/.config/i3/config"),
        env_with_context_no_errors("$XDG_CONFIG_HOME/i3/config", context),
        tilde("~/.i3/config"),
        Borrowed("/etc/xdg/i3/config"),
        env_with_context_no_errors("$XDG_CONFIG_DIRS/i3/config", context),
        Borrowed("/etc/i3/config"),
    ];

    let mut valid_path: String = String::new();

    for p in possible_paths.iter() {
        if Path::new(&p.to_string()).exists() {
            valid_path = p.to_string();
            break;
        }
    }

    if valid_path.is_empty() {
        panic!("Failed to find an i3 config file!");
    }

    PathBuf::from(valid_path)
}

fn main() {
    let config_file = find_config_file();
    let mut config = i3Config::from_path(config_file.as_path());
    println!("{}", config.dmenu_output().unwrap());
}
