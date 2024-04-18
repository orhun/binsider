use binsider::args::Args;
use binsider::error::Result;
use clap::Parser;
use std::process;

fn main() -> Result<()> {
    let args = Args::parse();
    match binsider::run(args) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
