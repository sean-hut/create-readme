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

    match arguments.occurrences_of("license-exclude") {
        0 => {
            append(&mut file, LICENCE);

            match arguments.value_of("license") {
                Some(heading) => match heading {
                    "0bsd" => append(&mut file, BSD0),
                    "mit0" => append(&mut file, MIT0),
                    "unlicense" => append(&mut file, UNLICENSE),
                    "cc0" => append(&mut file, CC0),
                    _ => {
                        eprintln!("Invalid license");
                        exit(1);
                    }
                },
                None => {
                    eprintln!("Must provide licence");
                    exit(1);
                }
            }

            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Licence section appended")
            }
        }
        1 => {
            if verbose {
                println!("License section excluded")
            }
        }

        _ => {
            eprintln!("Only one --disable-licence allowed.");
            exit(1);
        }
    }

    match arguments.occurrences_of("documentation-exclude") {
        0 => {
            append(&mut file, documentation());
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Documentation section appended")
            }
        }
        1 => {
            if verbose {
                println!("Documentation section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-documentation allowed.");
            exit(1);
        }
    }

    match arguments.occurrences_of("changelog-exclude") {
        0 => {
            append(&mut file, CHANGELOG);
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Changelog section appended")
            }
        }
        1 => {
            if verbose {
                println!("Changelog section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-changelog allowed.");
            exit(1);
        }
    }

    match arguments.occurrences_of("development-version-exclude") {
        0 => {
            append(&mut file, development_version());
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Development version section appended")
            }
        }
        1 => {
            if verbose {
                println!("Development version section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-development-version allowed.");
            exit(1);
        }
    }

    match arguments.occurrences_of("stable-version-exclude") {
        0 => {
            append(&mut file, stable_version());
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Stable version section appended")
            }
        }
        1 => {
            if verbose {
                println!("Stable version section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-stable-version allowed.");
            exit(1);
        }
    }


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
