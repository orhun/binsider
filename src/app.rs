use crate::error::Result;
use goblin::elf::Elf;

/// Binary analyzer.
pub struct Analyzer<'a> {
    bytes: &'a [u8],
    elf: Elf<'a>,
}

impl<'a> Analyzer<'a> {
    /// Constructs a new instance.
    pub fn new(bytes: &'a [u8]) -> Result<Self> {
        let elf = Elf::parse(bytes)?;
        Ok(Self { bytes, elf })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn get_test_bytes() -> Result<Vec<u8>> {
        let debug_binary = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(env!("CARGO_PKG_NAME"));
        Ok(fs::read(debug_binary)?)
    }

    #[test]
    fn test_analyzer_init() -> Result<()> {
        assert!(Analyzer::new(get_test_bytes()?.as_slice()).is_ok());
        Ok(())
    }
}
