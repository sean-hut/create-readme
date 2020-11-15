use clap::{App, Arg};

use impure::create_readme;

fn main() {
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
                .help("Top heading TEXT that follows \"# \""),
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
                .required(false)
                .takes_value(true)
                .multiple(false)
                .display_order(2)
                .help("Select a LICENSE\n"),
        )
        .arg(
            Arg::with_name("overwrite")
                .long("overwrite")
                .short("O")
                .multiple(false)
                .display_order(1)
                .help("Overwrite README.md"),
        )
        .arg(
            Arg::with_name("exclude-overview")
                .long("exclude-overview")
                .short("o")
                .multiple(false)
                .display_order(2)
                .help("Exclude overview section"),
        )
        .arg(
            Arg::with_name("exclude-example-use")
                .long("exclude-example-use")
                .short("e")
                .multiple(false)
                .display_order(3)
                .help("Exclude example use section"),
        )
        .arg(
            Arg::with_name("exclude-license")
                .long("exclude-license")
                .short("L")
                .multiple(false)
                .display_order(4)
                .help("Exclude license section"),
        )
        .arg(
            Arg::with_name("exclude-documentation")
                .long("exclude-documentation")
                .short("d")
                .multiple(false)
                .display_order(5)
                .help("Exclude documentation section"),
        )
        .arg(
            Arg::with_name("exclude-changelog")
                .long("exclude-changelog")
                .short("c")
                .multiple(false)
                .display_order(6)
                .help("Exclude changelog section"),
        )
        .arg(
            Arg::with_name("exclude-development-version")
                .long("exclude-development-version")
                .short("D")
                .multiple(false)
                .display_order(7)
                .help("Exclude development version section"),
        )
        .arg(
            Arg::with_name("exclude-stable-version")
                .long("exclude-stable-version")
                .short("s")
                .multiple(false)
                .display_order(8)
                .help("Exclude stable version section"),
        )
        .arg(
            Arg::with_name("exclude-contributing")
                .long("exclude-contributing")
                .short("C")
                .multiple(false)
                .display_order(9)
                .help("Exclude contributing section"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .multiple(false)
                .display_order(10)
                .help("Verbose output"),
        )
        .get_matches();

    create_readme(arguments);
}
