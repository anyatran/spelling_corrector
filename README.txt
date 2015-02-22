# Homework 4: A Spelling Corrector

## Files and Folders

* `src/'
  - `main.rs': contains the Rust code for the spelling corrector program
* `Cargo.toml' - Cargo project description
* `Cargo.lock' - Necessary as we use the `regex' module and for some reason,
Cargo was not respecting the versions listed in `Cargo.toml', requiring us to
edit the `Cargo.lock' directly to have the compatible module versions included.
* `.gitignore' - Prevents target folder, training text, and testing text from
being included in the git repo.
* `train.txt' - Text used as the training corpus for the spelling corrector
* `test.txt' - Text used to test the output of our spelling corrector
