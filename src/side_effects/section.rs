use clap::ArgMatches;

use crate::sections::section_structs::Section;
use crate::side_effects::append::{append, open};

pub fn section(arguments: &ArgMatches, section: Section) {
    let verbose: bool = arguments.is_present("verbose");
    let flag: bool = arguments.is_present(section.flag);

    let mut file = open();

    if !flag && !verbose {
        append(&mut file, section.content);
    } else if !flag && verbose {
        append(&mut file, section.content);
        println!("{}", section.append_message);
    } else if flag && verbose {
        println!("{}", section.exclude_message);
    }
}
