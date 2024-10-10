use binsider::args::Args;
use binsider::error::Result;
use clap::Parser;
use ratatui::style::Color;
use std::process;
use std::time::Duration;
use termbg::Theme;

fn main() -> Result<()> {
    let mut args = Args::parse();
    if args.accent_color.is_none() {
        args.accent_color = termbg::theme(Duration::from_millis(10))
            .map(|theme| {
                if theme == Theme::Dark {
                    Color::White
                } else {
                    Color::Black
                }
            })
            .ok();
    }
    match binsider::run(args) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
