mod sections;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;

use clap::ArgMatches;

use crate::sections::{
    changelog::CHANGELOG,
    contributing::contributing,
    documentation::documentation,
    example_use::example_use,
    license::{BSD0, CC0, LICENCE, MIT0, UNLICENSE},
    overview::overview,
    versions::{development_version, stable_version},
};

const README: &str = "README.md";

const BLANK_LINE: &str = "\n\n";

pub fn create_readme(arguments: ArgMatches) {
    let mut file = open_readme();

    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    match arguments.value_of("top-heading") {
        Some(heading) => {
            append(&mut file, "# ");
            append(&mut file, heading);
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Top heading appended")
            }
        }
        None => {
            eprintln!("Text for the top heading must be provided.");
            exit(1);
        }
    }

    match arguments.occurrences_of("overview-exclude") {
        0 => {
            append(&mut file, overview());
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Overview section appended")
            }
        }
        1 => {
            if verbose {
                println!("Overview section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-overview allowed.");
            exit(1);
        }
    }

    match arguments.occurrences_of("example-use-exclude") {
        0 => {
            append(&mut file, example_use());
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Example use section appended")
            }
        }
        1 => {
            if verbose {
                println!("Example use section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-example-use allowed.");
            exit(1);
        }
    }

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
    match write!(file, "{}", text) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error writing to file");
            exit(1);
        }
    }
}
