use std::process;

fn main() {
    match binsider::start_tui() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
