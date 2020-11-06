# Documentation

## Install

1. Install `rustup` with one of these options:

    - with your operating system's package manager
    - [instructions to install rust and cargo][rustup-install]

[rustup-install]: <https://doc.rust-lang.org/cargo/getting-started/installation.html>

2. Run `rustup update`.

3. Add `~/.cargo/bin` to your `PATH` environment variable.

4. Clone this git repository with:
   `git clone https://github.com/sean-hut/create-readme`

5. In the cloned git repository run:

    `cargo build --release`.

6. Copy the binary into the git hook directory of the repository.

    Copy
    `create-readme/target/release/create-readme`
    into the `~/.cargo/bin/`.

7. Make `~/.cargo/bin/create-readme` executable with this command:

    `chmod u=rwx ~/.cargo/bin/create-readme`

## Runtime Dependencies

This project has no runtime dependencies.

## Use

Create a readme for your project.

    1. Navigate to the root directory of your project.
	2. Run `create-readme`.

Customize the readme.

    1. Change `# TODO Top Heading`
	2. Change `TODO Overview section.`

## README.md content

This project's readme was created with `create-readme`.  Then the top
heading and overview section were customized.
