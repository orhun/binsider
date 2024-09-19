/// ELF dynamic section.
pub mod dynamic;
/// ELF header.
pub mod header;
/// ELF notes.
pub mod notes;
/// ELF relocations.
pub mod relocations;
/// ELF symbols.
pub mod symbols;

use dynamic::Dynamic;
use elf::{endian::AnyEndian, ElfBytes, ParseError};
use header::{FileHeaders, ProgramHeaders, SectionHeaders};
use notes::Notes;
use relocations::Relocations;
use symbols::{DynamicSymbols, Symbols};

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
            Info::Symbols => "Symbols",
            Info::DynamicSymbols => "Dynamic Symbols",
            Info::Dynamics => "Dynamic",
            Info::Relocations => "Relocations",
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
            Info::Symbols => &["Name", "Type", "Value", "Siz", "Bind", "Vis", "Ndx"],
            Info::DynamicSymbols => &["Name", "Reqs", "Type", "Value", "Siz", "Bind", "Vis", "Ndx"],
            Info::Dynamics => &["Tag", "Value"],
            Info::Relocations => &["Type", "Symbol", "Offset", "Addend"],
            Info::Notes => todo!(),
        }
    }
}

/// ELF wrapper.
#[derive(Debug)]
pub struct Elf {
    /// File headers.
    pub file_headers: FileHeaders,
    /// Program headers.
    pub program_headers: ProgramHeaders,
    /// Section headers.
    pub section_headers: SectionHeaders,
    /// Symbols.
    pub symbols: Symbols,
    /// Dynamic symbols.
    pub dynamic_symbols: DynamicSymbols,
    /// Dynamic.
    pub dynamic: Dynamic,
    /// Relocations.
    pub relocations: Relocations,
    /// Notes.
    pub notes: Notes,
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
            section_headers: SectionHeaders::try_from(elf_bytes.section_headers_with_strtab()?)
                .unwrap_or_default(),
            symbols: Symbols::try_from(elf_bytes.symbol_table()?).unwrap_or_default(),
            dynamic_symbols: DynamicSymbols::try_from((
                elf_bytes.dynamic_symbol_table()?,
                elf_bytes.symbol_version_table()?,
            ))
            .unwrap_or_default(),
            dynamic: Dynamic::try_from(elf_bytes.dynamic()?).unwrap_or_default(),
            relocations: Relocations::try_from(&elf_bytes).unwrap_or_default(),
            notes: Notes::try_from(&elf_bytes).unwrap_or_default(),
        })
    }
}

impl Elf {
    /// Returns the information about the ELF file.
    pub fn info<'a>(&self, info: &Info) -> Box<dyn Property<'a>> {
        match info {
            Info::FileHeaders => Box::new(self.file_headers),
            Info::ProgramHeaders => Box::new(self.program_headers.clone()),
            Info::SectionHeaders => Box::new(self.section_headers.clone()),
            Info::Symbols => Box::new(self.symbols.clone()),
            Info::DynamicSymbols => Box::new(self.dynamic_symbols.clone()),
            Info::Dynamics => Box::new(self.dynamic.clone()),
            Info::Relocations => Box::new(self.relocations.clone()),
            Info::Notes => todo!(),
        }
    }
}
