use binsider::app::Analyzer;
use binsider::args::Args;
use binsider::error::Result;
use clap::Parser;
use std::{env, fs, process};

fn main() -> Result<()> {
    let args = Args::parse();
    let file = args.file.unwrap_or(env::current_exe()?);
    let file_data = fs::read(&file)?;
    let bytes = file_data.as_slice();
    let analyzer = Analyzer::new(file.to_str().unwrap_or_default(), bytes)?;
    match binsider::start_tui(analyzer) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
