use clap::ArgMatches;

pub fn succes_message(arguments: &ArgMatches) {
    if arguments.is_present("verbose") {
        println!("[Success] README.md created")
    }
}
