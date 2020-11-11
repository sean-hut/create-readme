use std::process::exit;

use clap::ArgMatches;

use crate::sections::licenses::{BSD0, CC0, MIT0, UNLICENSE};

const LICENCE_PREAMBLE: &str = "## License

The license file is `LICENSE`.

";

pub fn license(arguments: &ArgMatches) -> String {
    [LICENCE_PREAMBLE, license_content(&arguments)].concat()
}

fn license_content(arguments: &ArgMatches) -> &'static str {
    match arguments.value_of("license") {
        Some(heading) => match heading {
            "0bsd" => BSD0,
            "mit0" => MIT0,
            "unlicense" => UNLICENSE,
            "cc0" => CC0,
            _ => {
                eprintln!("Invalid license");
                exit(1);
            }
        },
        None => {
            eprintln!("Must provide license");
            exit(1);
        }
    }
}
