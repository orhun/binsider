use crate::error::{Error, Result};
use goblin::elf::Elf;
use rust_strings::BytesConfig;

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

    /// Returns the sequences of printable characters.
    pub fn extract_strings(&self, min_length: usize) -> Result<Vec<(String, u64)>> {
        let config = BytesConfig::new(self.bytes.to_vec()).with_min_length(min_length);
        let strings =
            rust_strings::strings(&config).map_err(|e| Error::StringsError(e.to_string()))?;
        Ok(strings)
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
    fn test_init() -> Result<()> {
        assert!(Analyzer::new(get_test_bytes()?.as_slice()).is_ok());
        Ok(())
    }

    #[test]
    fn test_extract_strings() -> Result<()> {
        let test_bytes = get_test_bytes()?;
        let analyzer = Analyzer::new(test_bytes.as_slice())?;
        let strings = analyzer.extract_strings(4)?;
        assert!(strings.iter().map(|(s, _)| s).any(|v| v == ".debug_str"));
        Ok(())
    }
}
