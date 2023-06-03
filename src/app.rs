use crate::error::Result;
use goblin::elf::Elf;
use std::{fs, path::Path};

/// Binary analyzer.
pub struct Analyzer {
    elf: Elf<'static>,
}

impl Analyzer {
    /// Constructs a new instance.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file_data = fs::read(path)?;
        let data = Box::leak(file_data.into_boxed_slice());
        let elf = Elf::parse(data)?;
        Ok(Self { elf })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn get_debug_binary() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(env!("CARGO_PKG_NAME"))
    }

    #[test]
    fn test_analyzer_init() {
        assert!(Analyzer::new(get_debug_binary()).is_ok());
    }
}
