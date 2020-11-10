use clap::{App, Arg};

use side_effects::create_readme;

fn main() {
    create_readme();
    let arguments = App::new("create-readme")
        .version("0.1.0")
        .author("Sean Hutchings <seanhut@yandex.com>")
        .about("Create a readme markdown file")
}
