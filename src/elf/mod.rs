/// ELF header.
pub mod header;

use elf::{endian::AnyEndian, ElfBytes};
use header::{FileHeaders, ProgramHeaders};

/// ELF property for receiving information.
pub trait Property<'a> {
    /// Returns the items.
    fn items(&self) -> Vec<Vec<String>>;
}

/// ELF information.
#[derive(Debug)]
pub enum Info {
    /// File headers.
    FileHeaders,
    /// Program headers (segments).
    ProgramHeaders,
    /// Section headers.
    SectionHeaders,
    /// Symbols.
    Symbols,
    /// Dynamic symbols.
    DynamicSymbols,
    /// Dynamics.
    Dynamics,
    /// Relocations.
    Relocations,
    /// Notes.
    Notes,
}

impl Info {
    /// Returns the title.
    pub const fn title(&self) -> &str {
        match self {
            Info::FileHeaders => todo!(),
            Info::ProgramHeaders => "Program Headers / Segments",
            Info::SectionHeaders => todo!(),
            Info::Symbols => todo!(),
            Info::DynamicSymbols => todo!(),
            Info::Dynamics => todo!(),
            Info::Relocations => todo!(),
            Info::Notes => todo!(),
        }
    }

    /// Returns the headers.
    pub fn headers(&self) -> &[&str] {
        match self {
            Info::FileHeaders => todo!(),
            Info::ProgramHeaders => &[
                "p_type", "p_offset", "p_vaddr", "p_paddr", "p_filesz", "p_memsz", "p_align",
                "p_flags",
            ],
            Info::SectionHeaders => todo!(),
            Info::Symbols => todo!(),
            Info::DynamicSymbols => todo!(),
            Info::Dynamics => todo!(),
            Info::Relocations => todo!(),
            Info::Notes => todo!(),
        }
    }
}

/// Elf wrapper.
#[derive(Debug)]
pub struct Elf {
    /// File headers.
    pub file_headers: FileHeaders,
    /// Program headers.
    pub program_headers: ProgramHeaders,
}

impl<'a> From<ElfBytes<'a, AnyEndian>> for Elf {
    fn from(elf_bytes: ElfBytes<'a, AnyEndian>) -> Self {
        Self {
            file_headers: FileHeaders::from(elf_bytes.ehdr),
            program_headers: ProgramHeaders::from(match elf_bytes.segments() {
                Some(segments) => segments.iter().collect(),
                None => vec![],
            }),
        }
    }
}

impl Elf {
    /// Returns the information about the ELF file.
    pub fn info<'a>(&self, info: &Info) -> Box<dyn Property<'a>>
    where
        FileHeaders: Property<'a>,
        ProgramHeaders: Property<'a>,
    {
        match info {
            Info::FileHeaders => Box::new(self.file_headers),
            Info::ProgramHeaders => Box::new(self.program_headers.clone()),
            Info::SectionHeaders => todo!(),
            Info::Symbols => todo!(),
            Info::DynamicSymbols => todo!(),
            Info::Dynamics => todo!(),
            Info::Relocations => todo!(),
            Info::Notes => todo!(),
        }
    }
}
