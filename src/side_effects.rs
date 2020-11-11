mod sections;

use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;

use clap::ArgMatches;

use crate::sections::{
    changelog::CHANGELOG,
    contributing::contributing,
    documentation::documentation,
    example_use::example_use,
    license::license,
    overview::overview,
    versions::{development_version, stable_version},
};

const README: &str = "README.md";

const BLANK_LINE: &str = "\n\n";

pub fn create_readme(arguments: ArgMatches) {
    let overwrite: bool = arguments.occurrences_of("overwrite") > 0;

    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    if overwrite {
        match remove_file(README) {
            Ok(_) => {
                if verbose {
                    println!("Overwriting README.md")
                }
            }
            Err(e) => {
                eprintln!("Could not overwrite README.md: {}", e);
                exit(1);
            }
        }
    }

    if !overwrite && Path::new(README).exists() {
        eprintln!("README.md already exists.  If you want to overwrite README.md use the --overwrite flag.");
        exit(1);
    }

    let mut file = open_readme();

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
fn top_heading(arguments: &ArgMatches) {
    let verbose: bool = arguments.occurrences_of("verbose") > 0;

    let mut file = open_readme();

    match arguments.occurrences_of("documentation-exclude") {
        0 => {
            append(&mut file, documentation());
    match arguments.value_of("top-heading") {
        Some(heading) => {
            append(&mut file, "# ");
            append(&mut file, heading);
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Documentation section appended")
                println!("Top heading appended")
            }
        }
        1 => {
            if verbose {
                println!("Documentation section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-documentation allowed.");
        None => {
            eprintln!("Text for the top heading must be provided.  Use the option --top-heading");
            exit(1);
        }
    }
}

    match arguments.occurrences_of("changelog-exclude") {
        0 => {
            append(&mut file, CHANGELOG);
            append(&mut file, BLANK_LINE);
fn licence_section(arguments: &ArgMatches, section: Section) {
    let verbose: bool = arguments.occurrences_of("verbose") > 0;

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
    let mut file = open_readme();

    match arguments.occurrences_of("development-version-exclude") {
    match arguments.occurrences_of(section.flag) {
        0 => {
            append(&mut file, development_version());
            append(&mut file, &license(&arguments)[..]);
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("Development version section appended")
                println!("{}", section.append_message)
            }
        }
        1 => {
            if verbose {
                println!("Development version section excluded")
                println!("{}", section.exclude_message)
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
            eprintln!("{}", section.error_message);
            exit(1);
        }
    }
}

    match arguments.occurrences_of("contributing-exclude") {
        0 => {
            append(&mut file, contributing());

            if verbose {
                println!("Contributing section appended")
            }
        }
        1 => {
            if verbose {
                println!("Contributing section excluded")
            }
        }
        _ => {
            eprintln!("Only one --disable-contributing allowed.");
            exit(1);
        }
    }

    if verbose {
        println!("README.md created")
    }
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
