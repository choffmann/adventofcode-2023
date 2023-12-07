use std::{env, fs};
use std::fmt::Display;
use crate::days::Day;

pub enum Folder {
    Examples, Input
}

impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Folder::Examples => "examples".to_string(),
            Folder::Input => "input".to_string()
        };
        write!(f, "{}", str)
    }
}

#[must_use]
pub fn read_file(folder: Folder, day: Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data")
        .join(folder.to_string())
        .join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

#[must_use]
pub fn read_file_part(folder: Folder, day: Day, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder.to_string())
        .join(format!("{day}-{part}.txt"));
    println!("{}", filepath.display());
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
