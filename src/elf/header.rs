use elf::to_str::*;
use elf::{endian::AnyEndian, file::FileHeader};
use std::fmt::Display;

/// ELF header wrapper.
#[derive(Debug)]
pub struct Header {
    /// Inner type.
    inner: FileHeader<AnyEndian>,
}

impl From<FileHeader<AnyEndian>> for Header {
    fn from(inner: FileHeader<AnyEndian>) -> Self {
        Self { inner }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Class: {:?}", self.inner.class)?;
        writeln!(f, "Endianness: {:?}", self.inner.endianness)?;
        writeln!(
            f,
            "Object Type: {}",
            match e_type_to_human_str(self.inner.e_type) {
                Some(s) => s.to_string(),
                None => format!("e_type({:#x})", self.inner.e_type),
            }
        )?;
        writeln!(
            f,
            "Arch: {}",
            match e_machine_to_human_str(self.inner.e_machine) {
                Some(s) => s.to_string(),
                None => format!("e_machine({:#x})", self.inner.e_machine),
            }
        )?;
        writeln!(f, "OS/ABI: {}", e_osabi_to_string(self.inner.osabi))?;
        writeln!(f, "Entry point address: {:#x}", self.inner.e_entry)?;
        writeln!(
            f,
            "Start of program headers: {:#x} (bytes into file)",
            self.inner.e_phoff
        )?;
        writeln!(
            f,
            "Start of section headers: {:#x} (bytes into file)",
            self.inner.e_shoff
        )?;
        writeln!(f, "Flags: {:#x}", self.inner.e_flags)?;
        writeln!(f, "Size of this header: {:#x}", self.inner.e_ehsize)?;
        writeln!(f, "Size of program header: {:#x}", self.inner.e_phentsize)?;
        writeln!(f, "Number of program headers: {:#x}", self.inner.e_phnum)?;
        writeln!(f, "Size of section header: {:#x}", self.inner.e_shentsize)?;
        writeln!(f, "Number of section headers: {:#x}", self.inner.e_shnum)?;
        writeln!(
            f,
            "Section headers string table section index: {}",
            self.inner.e_shstrndx
        )?;
        Ok(())
    }
}
