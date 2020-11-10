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
}
