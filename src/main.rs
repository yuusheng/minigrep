mod error;
mod minigrep;

use std::process;

use clap::Parser;
use minigrep::{Args, MiniGrep};

fn main() {
    let args = Args::parse();

    if let Err(error) = MiniGrep::run(args) {
        eprintln!("Command excuting error: {}", error);
        process::exit(0);
    }
}
