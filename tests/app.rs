use binsider::{app::Analyzer, error::Result, file::FileInfo, prelude::Event};
use std::{fs, path::PathBuf, sync::mpsc};

fn get_test_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_binsider"))
}

fn get_test_bytes() -> Result<Vec<u8>> {
    let debug_binary = get_test_path();
    Ok(fs::read(debug_binary)?)
}

#[test]
fn test_init() -> Result<()> {
    Analyzer::new(
        FileInfo::new(
            get_test_path().to_str().expect("failed to get test path"),
            get_test_bytes()?.as_slice(),
        )?,
        4,
        vec![],
    )
    .map(|_| ())
}

#[test]
fn test_extract_strings() -> Result<()> {
    let test_bytes = get_test_bytes()?;
    let test_path = get_test_path();
    let mut analyzer = Analyzer::new(
        FileInfo::new(
            test_path.to_str().expect("failed to get test path"),
            test_bytes.as_slice(),
        )?,
        4,
        vec![],
    )?;
    let (tx, rx) = mpsc::channel();
    analyzer.extract_strings(tx);
    if let Event::FileStrings(strings) = rx.recv()? {
        assert!(strings?.iter().map(|(s, _)| s).any(|v| v == ".debug_str"));
    } else {
        panic!("strings did not succeed");
    }
    Ok(())
}
