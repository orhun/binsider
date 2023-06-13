use binsider::app::Analyzer;
use binsider::args::Args;
use binsider::error::Result;
use clap::Parser;
use std::{fs, process};

fn main() -> Result<()> {
    let args = Args::parse();
    let file_data = fs::read(args.file)?;
    let bytes = file_data.as_slice();
    let analyzer = Analyzer::new(bytes)?;
    match binsider::start_tui(analyzer) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
