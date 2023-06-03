use crate::error::Result;
use elf::{endian::AnyEndian, ElfBytes};
use std::{fs, path::Path};

/// Binary analyzer.
pub struct Analyzer {
    elf_bytes: ElfBytes<'static, AnyEndian>,
}

impl Analyzer {
    /// Constructs a new instance.
    pub fn new(path: &Path) -> Result<Self> {
        let file_data = fs::read(path)?;
        let data = Box::leak(file_data.into_boxed_slice());
        let elf_bytes = ElfBytes::<AnyEndian>::minimal_parse(data)?;
        Ok(Self { elf_bytes })
    }
}
