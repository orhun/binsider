use crate::elf::Property;
use elf::segment::ProgramHeader as ElfProgramHeader;
use elf::to_str::*;
use elf::{endian::AnyEndian, file::FileHeader as ElfFileHeader};

/// ELF file file header wrapper.
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

impl<'a> Property<'a> for FileHeaders {
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
                .strip_prefix("ELFOSABI_")
                .unwrap_or("unknown")
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

/// ELF file program header wrapper.
#[derive(Clone, Debug)]
pub struct ProgramHeaders {
    /// Inner type.
    inner: Vec<ElfProgramHeader>,
}

impl From<Vec<ElfProgramHeader>> for ProgramHeaders {
    fn from(inner: Vec<ElfProgramHeader>) -> Self {
        Self { inner }
    }
}

impl<'a> Property<'a> for ProgramHeaders {
    fn headers() -> Option<&'a [&'a str]> {
        Some(&[
            "p_type", "p_offset", "p_vaddr", "p_paddr", "p_filesz", "p_memsz", "p_align", "p_flags",
        ])
    }
    fn items(&self) -> Vec<Vec<String>> {
        self.inner
            .iter()
            .map(|item| {
                vec![
                    elf::to_str::p_type_to_string(item.p_type),
                    format!("{:#x}", item.p_offset),
                    format!("{:#x}", item.p_vaddr),
                    format!("{:#x}", item.p_paddr),
                    format!("{:#x}", item.p_filesz),
                    format!("{:#x}", item.p_memsz),
                    item.p_align.to_string(),
                    elf::to_str::p_flags_to_string(item.p_flags),
                ]
            })
            .collect()
    }
}
