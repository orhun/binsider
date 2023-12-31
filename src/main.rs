use binsider::app::Analyzer;
use binsider::args::Args;
use binsider::error::Result;
use clap::Parser;
use std::{env, fs, process};

fn main() -> Result<()> {
    let args = Args::parse();
    let file = args.file.clone().unwrap_or(env::current_exe()?);
    let file_data = fs::read(&file)?;
    let bytes = file_data.as_slice();
    let analyzer =
        Analyzer::new(bytes, args.min_strings_len)?.with_path(file.to_str().unwrap_or_default());
    match binsider::start_tui(analyzer) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
