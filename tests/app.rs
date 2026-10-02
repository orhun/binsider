use binsider::{
    app::Analyzer,
    error::Result,
    file::FileInfo,
    prelude::{Event, State},
};
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
            None,
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
            None,
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

#[test]
fn test_general_tab_search() -> Result<()> {
    let test_bytes = get_test_bytes()?;
    let test_path = get_test_path();
    let analyzer = Analyzer::new(
        FileInfo::new(
            test_path.to_str().expect("failed to get test path"),
            None,
            test_bytes.as_slice(),
        )?,
        4,
        vec![],
    )?;
    let mut state = State::new(analyzer, None)?;
    // Use a fixed set of dependencies so the test does not depend on the
    // linker resolving the real ones at runtime.
    state.analyzer.dependencies = vec![
        ("libc.so.6".to_string(), "/lib/libc.so.6".to_string()),
        (
            "libgcc_s.so.1".to_string(),
            "/lib/libgcc_s.so.1".to_string(),
        ),
        (
            "libssl.so.3".to_string(),
            "/usr/lib/libssl.so.3".to_string(),
        ),
    ];

    // Empty query keeps every dependency.
    state.input = "".into();
    state.handle_tab()?;
    assert_eq!(state.list.items.len(), 3);

    // Match against the library name.
    state.input = "ssl".into();
    state.handle_tab()?;
    assert_eq!(state.list.items.len(), 1);
    assert_eq!(state.list.items[0][0], "libssl.so.3");

    // Match against the resolved path, case-insensitively.
    state.input = "USR".into();
    state.handle_tab()?;
    assert_eq!(state.list.items.len(), 1);
    assert_eq!(state.list.items[0][0], "libssl.so.3");

    // A query that matches nothing clears the list.
    state.input = "nope".into();
    state.handle_tab()?;
    assert!(state.list.items.is_empty());

    Ok(())
}
