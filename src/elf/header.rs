use crate::elf::Property;
use bytesize::ByteSize;
use elf::parse::ParsingTable;
use elf::section::SectionHeader;
use elf::segment::ProgramHeader;
use elf::string_table::StringTable;
use elf::{endian::AnyEndian, file::FileHeader as ElfFileHeader};
use elf::{to_str::*, ParseError};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// ELF file header wrapper.
#[derive(Clone, Copy, Debug)]
pub struct FileHeaders {
    /// Inner type.
    inner: ElfFileHeader<AnyEndian>,
}

impl From<ElfFileHeader<AnyEndian>> for FileHeaders {
    fn from(inner: ElfFileHeader<AnyEndian>) -> Self {
        Self { inner }
    }
}

impl Property<'_> for FileHeaders {
    fn items(&self) -> Vec<Vec<String>> {
        let mut headers = Vec::new();
        headers.push(vec![
            String::from("Class"),
            format!("{:?}", self.inner.class),
        ]);
        headers.push(vec![
            String::from("Endianness"),
            format!("{:?}", self.inner.endianness),
        ]);
        headers.push(vec![
            String::from("Version"),
            match self.inner.version {
                1 => String::from("1 (current)"),
                v => v.to_string(),
            },
        ]);
        headers.push(vec![
            String::from("OS/ABI"),
            e_osabi_to_string(self.inner.osabi)
                .trim_start_matches("ELFOSABI_")
                .to_string(),
        ]);
        headers.push(vec![
            String::from("ABI Version"),
            self.inner.abiversion.to_string(),
        ]);
        headers.push(vec![
            String::from("Type"),
            match e_type_to_human_str(self.inner.e_type) {
                Some(s) => s.to_string(),
                None => format!("e_type({:#x})", self.inner.e_type),
            },
        ]);
        headers.push(vec![
            String::from("Arch"),
            match e_machine_to_human_str(self.inner.e_machine) {
                Some(s) => s.to_string(),
                None => format!("e_machine({:#x})", self.inner.e_machine),
            },
        ]);
        headers.push(vec![
            String::from("Entry point address"),
            format!("{:#x}", self.inner.e_entry),
        ]);
        headers.push(vec![
            String::from("Start of program headers"),
            format!("{} (bytes into file)", self.inner.e_phoff),
        ]);
        headers.push(vec![
            String::from("Start of section headers"),
            format!("{} (bytes into file)", self.inner.e_shoff),
        ]);
        headers.push(vec![
            String::from("Flags"),
            format!("{:#x}", self.inner.e_flags),
        ]);
        headers.push(vec![
            String::from("Size of this header"),
            format!("{} (bytes)", self.inner.e_ehsize),
        ]);
        headers.push(vec![
            String::from("Size of program header"),
            format!("{} (bytes)", self.inner.e_phentsize),
        ]);
        headers.push(vec![
            String::from("Number of program headers"),
            self.inner.e_phnum.to_string(),
        ]);
        headers.push(vec![
            String::from("Size of section headers"),
            format!("{} (bytes)", self.inner.e_shentsize),
        ]);
        headers.push(vec![
            String::from("Number of section headers"),
            self.inner.e_shnum.to_string(),
        ]);
        headers.push(vec![
            String::from("Section headers string table section index"),
            self.inner.e_shstrndx.to_string(),
        ]);
        headers
    }
}

/// ELF program header wrapper.
#[derive(Clone, Debug)]
pub struct ProgramHeaders {
    /// Inner type.
    inner: Vec<ProgramHeader>,
    /// Human readable format
    human_readable: bool,
}

impl ProgramHeaders {
    /// Toggles the value for human readable format.
    pub fn toggle_readability(&mut self) {
        self.human_readable = !self.human_readable;
    }
}

impl From<Vec<ProgramHeader>> for ProgramHeaders {
    fn from(inner: Vec<ProgramHeader>) -> Self {
        Self {
            inner,
            human_readable: true,
        }
    }
}

impl Property<'_> for ProgramHeaders {
    fn items(&self) -> Vec<Vec<String>> {
        self.inner
            .iter()
            .map(|item| {
                vec![
                    elf::to_str::p_type_to_string(item.p_type)
                        .trim_start_matches("PT_")
                        .to_string(),
                    format!("{:#x}", item.p_offset),
                    format!("{:#x}", item.p_vaddr),
                    format!("{:#x}", item.p_paddr),
                    if self.human_readable {
                        format!("{}", ByteSize(item.p_filesz))
                    } else {
                        format!("{:#x}", item.p_filesz)
                    },
                    if self.human_readable {
                        format!("{}", ByteSize(item.p_memsz))
                    } else {
                        format!("{:#x}", item.p_memsz)
                    },
                    elf::to_str::p_flags_to_string(item.p_flags),
                    format!("{:#x}", item.p_align),
                ]
            })
            .collect()
    }
}

/// ELF file section header wrapper.
#[derive(Clone, Debug, Default)]
pub struct SectionHeaders {
    /// Inner type.
    inner: Vec<SectionHeader>,
    /// Section names.
    names: Vec<String>,
    /// Human readable format
    human_readable: bool,
}

impl SectionHeaders {
    /// Toggles the value for human readable format.
    pub fn toggle_readability(&mut self) {
        self.human_readable = !self.human_readable;
    }
}

impl<'a>
    TryFrom<(
        Option<ParsingTable<'a, AnyEndian, SectionHeader>>,
        Option<StringTable<'a>>,
    )> for SectionHeaders
{
    type Error = ParseError;
    fn try_from(
        value: (
            Option<ParsingTable<'a, AnyEndian, SectionHeader>>,
            Option<StringTable<'a>>,
        ),
    ) -> Result<Self, Self::Error> {
        let (parsing_table, string_table) = (
            value.0.ok_or_else(|| {
                ParseError::IOError(IoError::new(
                    IoErrorKind::Other,
                    "parsing table does not exist",
                ))
            })?,
            value.1.ok_or_else(|| {
                ParseError::IOError(IoError::new(
                    IoErrorKind::Other,
                    "string table does not exist",
                ))
            })?,
        );
        Ok(Self {
            inner: parsing_table.iter().collect(),
            names: parsing_table
                .iter()
                .map(|v| {
                    string_table
                        .get(v.sh_name as usize)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|_| String::from("unknown"))
                })
                .collect(),
            human_readable: true,
        })
    }
}

impl Property<'_> for SectionHeaders {
    fn items(&self) -> Vec<Vec<String>> {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, header)| {
                vec![
                    self.names[i].to_string(),
                    elf::to_str::sh_type_to_string(header.sh_type)
                        .trim_start_matches("SHT_")
                        .to_string(),
                    format!("{:#x}", header.sh_addr),
                    format!("{:#x}", header.sh_offset),
                    if self.human_readable {
                        format!("{}", ByteSize(header.sh_size))
                    } else {
                        format!("{:#x}", header.sh_size)
                    },
                    header.sh_entsize.to_string(),
                    format!("{:#x}", header.sh_flags),
                    header.sh_link.to_string(),
                    header.sh_info.to_string(),
                    header.sh_addralign.to_string(),
                ]
            })
            .collect()
    }
}
