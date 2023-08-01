mod commands;

use std::env::args_os;

fn main() {
    commands::scl::new()
        .run(args_os())
        .unwrap_or_else(|e| eprintln!("{}", e));
}
