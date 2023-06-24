/// ELF header.
pub mod header;

use elf::{endian::AnyEndian, ElfBytes, ParseError};
use header::{FileHeaders, ProgramHeaders, SectionHeaders};

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
    pub fn title(&self) -> &str {
        match self {
            Info::FileHeaders => todo!(),
            Info::ProgramHeaders => "Program Headers / Segments",
            Info::SectionHeaders => "Section Headers",
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
                "Type", "Offset", "VirtAddr", "PhysAddr", "FileSiz", "MemSiz", "Flags", "Align",
            ],
            Info::SectionHeaders => &[
                "Name", "Type", "Addr", "Offset", "Size", "EntSiz", "Flags", "Link", "Info",
                "Align",
            ],
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
    /// Section headers.
    pub section_headers: SectionHeaders,
}

impl<'a> TryFrom<ElfBytes<'a, AnyEndian>> for Elf {
    type Error = ParseError;
    fn try_from(elf_bytes: ElfBytes<'a, AnyEndian>) -> Result<Self, Self::Error> {
        Ok(Self {
            file_headers: FileHeaders::from(elf_bytes.ehdr),
            program_headers: ProgramHeaders::from(match elf_bytes.segments() {
                Some(segments) => segments.iter().collect(),
                None => vec![],
            }),
            section_headers: SectionHeaders::try_from(elf_bytes.section_headers_with_strtab()?)?,
        })
    }
}

impl Elf {
    /// Returns the information about the ELF file.
    pub fn info<'a>(&self, info: &Info) -> Box<dyn Property<'a>>
    where
        FileHeaders: Property<'a>,
        ProgramHeaders: Property<'a>,
        SectionHeaders: Property<'a>,
    {
        match info {
            Info::FileHeaders => Box::new(self.file_headers),
            Info::ProgramHeaders => Box::new(self.program_headers.clone()),
            Info::SectionHeaders => Box::new(self.section_headers.clone()),
            Info::Symbols => todo!(),
            Info::DynamicSymbols => todo!(),
            Info::Dynamics => todo!(),
            Info::Relocations => todo!(),
            Info::Notes => todo!(),
        }
    }
}
