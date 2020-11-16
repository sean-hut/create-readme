use std::process::exit;

use clap::ArgMatches;

use crate::side_effects::append::{append, open};

use crate::licenses::{bsd0::BSD0, cc0::CC0, mit0::MIT0, unlicense::UNLICENSE};

pub fn licence_section(arguments: &ArgMatches) {
    let verbose: bool = arguments.is_present("verbose");

    let mut file = open();

    if arguments.is_present("exclude-license") {
        if verbose {
            println!("[Info] License section excluded")
        }
    } else {
        append(&mut file, LICENCE_PREAMBLE);
        append(&mut file, license_content(&arguments));

        if verbose {
            println!("[Info] Licence section appended")
        }
    }
}

const LICENCE_PREAMBLE: &str = "## License\n\
                                \n\
                                The license file is `LICENSE`.\n\
                                \n";

fn license_content(arguments: &ArgMatches) -> &'static str {
    match arguments.value_of("license") {
        Some(heading) => match heading {
            "0bsd" => BSD0,
            "mit0" => MIT0,
            "unlicense" => UNLICENSE,
            "cc0" => CC0,
            _ => {
                eprintln!("[Error] Invalid license");
                exit(1);
            }
        },
        None => {
            eprintln!("[Error] Must provide LICENSE to --license option");
            exit(1);
        }
    }
}
