use goblin::elf::Header as ElfHeader;

/// ELF header wrapper.
#[derive(Debug)]
pub struct Header {
    /// Inner type.
    inner: ElfHeader,
}

impl From<ElfHeader> for Header {
    fn from(inner: ElfHeader) -> Self {
        Self { inner }
    }
}
