mod sections;
mod side_effects;

use clap::ArgMatches;

use crate::sections::{
    changelog::CHANGELOG,
    contributing::CONTRIBUTING,
    documentation::DOCUMENTATION,
    example_use::EXAMPLE_USE,
    overview::OVERVIEW,
    section_structs::Section,
    versions::{DEVELOPMENT_VERSION, STABLE_VERSION},
};
use crate::side_effects::{
    append::{append, open},
    checks::overwrite_checks,
    output::succes_message,
};

pub fn create_readme(arguments: ArgMatches) {
    overwrite_checks(&arguments);


    section(&arguments, OVERVIEW);

    section(&arguments, EXAMPLE_USE);


    section(&arguments, DOCUMENTATION);

    section(&arguments, CHANGELOG);

    section(&arguments, DEVELOPMENT_VERSION);

    section(&arguments, STABLE_VERSION);

    section(&arguments, CONTRIBUTING);

    succes_message(&arguments);
}

fn section(arguments: &ArgMatches, section: Section) {
    let verbose: bool = arguments.is_present("verbose");

    let mut file = open();

    if arguments.is_present(section.flag) {
        if verbose {
            println!("{}", section.exclude_message)
        }
    } else {
        append(&mut file, section.content);

        if verbose {
            println!("{}", section.append_message)
        }
    }
}

