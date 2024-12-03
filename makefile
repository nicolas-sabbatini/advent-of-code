## Create the lib
rustc --crate-name input_loader --edition=2021 input_loader/src/lib.rs --crate-type lib
## Build the lib
rustc --crate-name 2024_02b --edition=2021 2024/src/day-02-b.rs --extern input_loader=libinput_loader.rlib
