use binsider::app::Analyzer;
use binsider::args::Args;
use clap::Parser;
use std::process;

fn main() {
    let args = Args::parse();
    let _analyzer = Analyzer::new(&args.file);
    match binsider::start_tui() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
