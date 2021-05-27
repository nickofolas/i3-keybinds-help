use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

lazy_static! {
    static ref GET_KEYBINDS: Regex = Regex::new(r"# ## .* ##\nbindsym .+").unwrap();
    static ref GET_COMMENT: Regex = Regex::new(r"# ## (.*) ##").unwrap();
}

// Because I3Config just doesn't look right
#[allow(non_camel_case_types)]
pub struct i3Config {
    reader: BufReader<File>,
}

impl i3Config {
    pub fn from_path(path: &Path) -> i3Config {
        let file: File = File::open(path).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        i3Config { reader }
    }

    fn extract_hotkey(line: &str) -> String {
        let hotkey: &str = line.split(' ').collect::<Vec<&str>>()[1];
        hotkey.replace("+", " + ").replace("$mod", "")
    }

    fn extract_comment(line: &str) -> String {
        let caps = GET_COMMENT.captures(line).unwrap();
        caps[1].to_string()
    }

    fn parse_file(&mut self) -> Result<Vec<String>, regex::Error> {
        let mut keybinds: Vec<String> = Vec::new();
        let mut file_contents: String = String::new();
        self.reader
            .read_to_string(&mut file_contents)
            .expect("Failed to read the config file into a string!");

        for mat in GET_KEYBINDS.find_iter(&file_contents) {
            let mat = mat.as_str().lines().collect::<Vec<&str>>();

            let comment = i3Config::extract_comment(&mat[0]);
            let keybind = i3Config::extract_hotkey(&mat[1]);

            let formatted = format!("<b>{}:</b> {}", keybind, comment);
            keybinds.push(formatted);
        }

        Ok(keybinds)
    }

    pub fn dmenu_output(&mut self) -> Result<String, regex::Error> {
        let keybinds = self.parse_file()?;
        Ok(keybinds.join("\n"))
    }
}
