use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;

const README: &str = "README.md";

pub fn open() -> File {
    match OpenOptions::new().append(true).create(true).open(README) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("[Error] Could not open {}: {}", README, e);
            exit(1);
        }
    }
}

pub fn append(file: &mut File, text: &str) {
    match write!(file, "{}", text) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[Error] Could not write to file. {}", e);
            exit(1);
        }
    }
}
