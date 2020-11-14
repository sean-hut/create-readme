use std::fs::remove_file;
use std::path::Path;
use std::process::exit;

use clap::ArgMatches;

use crate::side_effects::contstants::README;

pub fn overwrite_checks(arguments: &ArgMatches) {
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
