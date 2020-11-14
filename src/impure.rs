mod sections;
mod side_effects;

use std::fs::remove_file;
use std::path::Path;
use std::process::exit;

use clap::ArgMatches;

use crate::sections::{
    changelog::changelog,
    contributing::contributing,
    documentation::documentation,
    example_use::example_use,
    license::license,
    overview::overview,
    section_structs::{
        Section, CHANGELOG, CONTRIBUTING, DEVELOPMENT_VERSION, DOCUMENTATION, EXAMPLE_USE, LICENSE,
        OVERVIEW, STABLE_VERSION,
    },
    versions::{development_version, stable_version},
};
use crate::side_effects::append::{append, open};

const README: &str = "README.md";

const BLANK_LINE: &str = "\n\n";

pub fn create_readme(arguments: ArgMatches) {
    overwrite_checks(&arguments);

    top_heading(&arguments);

    section(&arguments, OVERVIEW, &overview);

    section(&arguments, EXAMPLE_USE, &example_use);

    licence_section(&arguments, LICENSE);

    section(&arguments, DOCUMENTATION, &documentation);

    section(&arguments, CHANGELOG, &changelog);

    section(&arguments, DEVELOPMENT_VERSION, &development_version);

    section(&arguments, STABLE_VERSION, &stable_version);

    section(&arguments, CONTRIBUTING, &contributing);

    succes_message(&arguments);
}

fn overwrite_checks(arguments: &ArgMatches) {
    let overwrite: bool = arguments.is_present("overwrite");

    if overwrite {
        match remove_file(README) {
            Ok(_) => {
                if arguments.is_present("verbose") {
                    println!("[Info] Overwriting README.md")
                }
            }
            Err(e) => {
                eprintln!("[Error] Could not overwrite README.md: {}", e);
                exit(1);
            }
        }
    }

    if !overwrite && Path::new(README).exists() {
        eprintln!(
            "[Error] README.md already exists. \
             If you want to overwrite README.md use the --overwrite flag."
        );
        exit(1);
    }
}

fn section(arguments: &ArgMatches, section: Section, function: &dyn Fn() -> &'static str) {
    let verbose: bool = arguments.is_present("verbose");

    let mut file = open();

    if arguments.is_present(section.flag) {
        if verbose {
            println!("{}", section.exclude_message)
        }
    } else {
        append(&mut file, function());
        append(&mut file, BLANK_LINE);

        if verbose {
            println!("{}", section.append_message)
        }
    }
}

fn top_heading(arguments: &ArgMatches) {
    let verbose: bool = arguments.is_present("verbose");

    let mut file = open();

    match arguments.value_of("top-heading") {
        Some(heading) => {
            append(&mut file, "# ");
            append(&mut file, heading);
            append(&mut file, BLANK_LINE);

            if verbose {
                println!("[Info] Top heading appended")
            }
        }
        None => {
            eprintln!(
                "[Error] Text for the top heading must be provided. \
                 Use the option --top-heading"
            );
            exit(1);
        }
    }
}

fn licence_section(arguments: &ArgMatches, section: Section) {
    let verbose: bool = arguments.is_present("verbose");

    let mut file = open();

    if arguments.is_present(section.flag) {
        if verbose {
            println!("{}", section.exclude_message)
        }
    } else {
        append(&mut file, &license(&arguments)[..]);
        append(&mut file, BLANK_LINE);

        if verbose {
            println!("{}", section.append_message)
        }
    }
}

fn succes_message(arguments: &ArgMatches) {
    if arguments.is_present("verbose") {
        println!("[Success] README.md created")
    }
}
