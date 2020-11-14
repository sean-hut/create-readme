use std::process::exit;

use clap::ArgMatches;

use crate::side_effects::append::{append, open};

pub fn top_heading(arguments: &ArgMatches) {
    let mut file = open();

    match arguments.value_of("top-heading") {
        Some(heading) => {
            append(&mut file, &format!("# {}\n\n", heading)[..]);

            if arguments.is_present("verbose") {
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
