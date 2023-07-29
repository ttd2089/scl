mod commands;
mod cwrap;

use std::env::args_os;

fn main() {
    commands::scl::new().run(args_os());
}
