# Use > instead of a tab as recipe prefixes.
.RECIPEPREFIX = >

####################
# Internal variables
####################

main = src/main.rs
side-effects = src/side_effects.rs
sections = src/sections/
checks = $(sections)changelog.rs $(sections)contributing.rs \
$(sections)documentation.rs $(sections)example_use.rs \
$(sections)license.rs $(sections)overview.rs \
$(sections)versions.rs
rust-files = $(main) $(side-effects) $(checks)

#######
# rules
#######

all: rust-format rust-clippy-lints build test git-diff-check

rust-all: rust-format rust-clippy-lints build test

rust-format: $(rust-files)
> cargo fmt -- --check --files-with-diff

rust-clippy-lints: $(rust-files)
> cargo clippy -- --deny clippy::all

build: $(rust-files)
> cargo build

test: $(rust-files)
> cargo test

.PHONY: git-diff-check
git-diff-check:
> git diff --check

.PHONY: clean
clean:
> cargo clean
