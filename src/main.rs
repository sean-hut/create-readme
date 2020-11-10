use clap::{App, Arg};

use side_effects::create_readme;

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
            Arg::with_name("overwrite")
                .long("overwrite")
                .short("O")
                .multiple(false)
                .display_order(1)
                .help("Overwrite README.md"),
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
        .arg(
            Arg::with_name("license-exclude")
                .long("exclude-license")
                .short("L")
                .multiple(false)
                .display_order(3)
                .help("Exclude license section"),
        )
        .arg(
            Arg::with_name("documentation-exclude")
                .long("exclude-documentation")
                .short("d")
                .multiple(false)
                .display_order(4)
                .help("Exclude documentation section"),
        )
        .arg(
            Arg::with_name("changelog-exclude")
                .long("exclude-changelog")
                .short("c")
                .multiple(false)
                .display_order(5)
                .help("Exclude changelog section"),
        )
        .arg(
            Arg::with_name("development-version-exclude")
                .long("exclude-development-version")
                .short("D")
                .multiple(false)
                .display_order(6)
                .help("Exclude development version section"),
        )
        .arg(
            Arg::with_name("stable-version-exclude")
                .long("exclude-stable-version")
                .short("s")
                .multiple(false)
                .display_order(7)
                .help("Exclude stable version section"),
        )
        .arg(
            Arg::with_name("contributing-exclude")
                .long("exclude-contributing")
                .short("C")
                .multiple(false)
                .display_order(8)
                .help("Exclude contributing section"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .multiple(false)
                .display_order(9)
                .help("Verbose output"),
        )
        .get_matches();

    create_readme(arguments);
}
