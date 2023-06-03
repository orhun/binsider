use binsider::args::Args;
use clap::Parser;
use std::process;

fn main() {
    let _args = Args::parse();
    match binsider::start_tui() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
