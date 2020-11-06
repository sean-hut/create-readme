mod sections;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;

use crate::sections::{
    changelog::CHANGELOG,
    contributing::contributing,
    documentation::documentation,
    example_use::example_use,
    license::licence,
    overview::overview,
    top_heading::top_heading,
    versions::{development_version, stable_version},
};

const README: &str = "README.md";

const BLANK_LINE: &str = "\n\n";

pub fn create_readme() {
    let mut file = open_readme();

    append(&mut file, top_heading());
    append(&mut file, BLANK_LINE);

    append(&mut file, overview());
    append(&mut file, BLANK_LINE);

    append(&mut file, example_use());
    append(&mut file, BLANK_LINE);

    append(&mut file, licence());
    append(&mut file, BLANK_LINE);

    append(&mut file, documentation());
    append(&mut file, BLANK_LINE);

    append(&mut file, CHANGELOG);
    append(&mut file, BLANK_LINE);

    append(&mut file, development_version());
    append(&mut file, BLANK_LINE);

    append(&mut file, stable_version());
    append(&mut file, BLANK_LINE);

    append(&mut file, contributing());
}

fn open_readme() -> File {
    match OpenOptions::new().append(true).create(true).open(README) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening {}: {}", README, e);
            exit(1);
        }
    }
}

fn append(file: &mut File, text: &str) {
    // match writeln!(file, "{}", text) {
    match write!(file, "{}", text) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error writing to file");
            exit(1);
        }
    }
}
