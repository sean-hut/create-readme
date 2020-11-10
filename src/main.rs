use clap::{App, Arg};

use side_effects::create_readme;

fn main() {
    create_readme();
    let arguments = App::new("create-readme")
        .version("0.1.0")
        .author("Sean Hutchings <seanhut@yandex.com>")
        .about("Create a readme markdown file")
        .arg(
            Arg::with_name("top-heading")
                .long("top-heading")
                .short("t")
                .value_name("TEXT")
                .required(true)
                .takes_value(true)
                .multiple(false)
                .display_order(1)
                .help("Top heading text that follows \"# \""),
        )
        .arg(
            Arg::with_name("license")
                .long("license")
                .short("l")
                .value_name("LICENSE")
                .possible_value("0bsd")
                .possible_value("mit0")
                .possible_value("unlicense")
                .possible_value("cc0")
                .required(true)
                .takes_value(true)
                .multiple(false)
                .display_order(2)
                .help("Select a LICENSE"),
        )
        .arg(
            Arg::with_name("overview-exclude")
                .long("exclude-overview")
                .short("o")
                .multiple(false)
                .display_order(1)
                .help("Exclude overview section"),
        )
        .arg(
            Arg::with_name("example-use-exclude")
                .long("exclude-example-use")
                .short("e")
                .multiple(false)
                .display_order(2)
                .help("Exclude example use section"),
        )
}
