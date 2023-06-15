/// ELF header.
pub mod header;

use elf::{endian::AnyEndian, segment::ProgramHeader, ElfBytes};
use header::{FileHeader, FileHeaders};

/// Elf wrapper.
pub struct Elf {
    /// File headers.
    pub file_headers: Vec<FileHeader>,
    /// Program headers.
    pub program_headers: Vec<ProgramHeader>,
}

impl<'a> From<ElfBytes<'a, AnyEndian>> for Elf {
    fn from(elf_bytes: ElfBytes<'a, AnyEndian>) -> Self {
        Self {
            file_headers: FileHeaders::from(elf_bytes.ehdr).get(),
            program_headers: match elf_bytes.segments() {
                Some(segments) => segments.iter().collect(),
                None => vec![],
            },
        }
    }
}
