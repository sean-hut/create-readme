mod licenses;
mod sections;
mod side_effects;

use clap::ArgMatches;

use crate::sections::{
    changelog::CHANGELOG,
    contributing::CONTRIBUTING,
    documentation::DOCUMENTATION,
    example_use::EXAMPLE_USE,
    overview::OVERVIEW,
    versions::{DEVELOPMENT_VERSION, STABLE_VERSION},
};

use crate::side_effects::{
    checks::{check_license, overwrite_checks},
    license::licence_section,
    output::succes_message,
    section::section,
    top_heading::top_heading,
};

pub fn create_readme(arguments: ArgMatches) {
    overwrite_checks(&arguments);

    check_license(&arguments);

    top_heading(&arguments);

    section(&arguments, OVERVIEW);

    section(&arguments, EXAMPLE_USE);

    licence_section(&arguments);

    section(&arguments, DOCUMENTATION);

    section(&arguments, CHANGELOG);

    section(&arguments, DEVELOPMENT_VERSION);

    section(&arguments, STABLE_VERSION);

    section(&arguments, CONTRIBUTING);

    succes_message(&arguments);
}
