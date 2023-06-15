use elf::to_str::*;
use elf::{endian::AnyEndian, file::FileHeader as ElfFileHeader};

/// An ELF file header entry.
#[derive(Debug)]
pub struct FileHeader {
    /// Name of the header.
    pub name: String,
    /// Value of the header.
    pub value: String,
}

impl FileHeader {
    /// Constructs a new instance.
    pub fn new<S: Into<String>>(name: S, value: S) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// ELF file header wrapper.
#[derive(Debug)]
pub struct FileHeaders {
    /// Inner type.
    inner: ElfFileHeader<AnyEndian>,
}

impl From<ElfFileHeader<AnyEndian>> for FileHeaders {
    fn from(inner: ElfFileHeader<AnyEndian>) -> Self {
        Self { inner }
    }
}

impl FileHeaders {
    /// Returns the headers.
    pub fn get(&self) -> Vec<FileHeader> {
        let mut headers = Vec::new();
        headers.push(FileHeader::new("Class", &format!("{:?}", self.inner.class)));
        headers.push(FileHeader::new(
            "Endianness",
            &format!("{:?}", self.inner.endianness),
        ));
        headers.push(FileHeader::new(
            "Version: {}",
            &match self.inner.version {
                1 => String::from("1 (current)"),
                v => v.to_string(),
            },
        ));
        headers.push(FileHeader::new(
            "OS/ABI",
            e_osabi_to_string(self.inner.osabi)
                .strip_prefix("ELFOSABI_")
                .unwrap_or("unknown"),
        ));
        headers.push(FileHeader::new(
            "ABI Version",
            &self.inner.abiversion.to_string(),
        ));
        headers.push(FileHeader::new(
            "Type",
            &match e_type_to_human_str(self.inner.e_type) {
                Some(s) => s.to_string(),
                None => format!("e_type({:#x})", self.inner.e_type),
            },
        ));
        headers.push(FileHeader::new(
            "Arch",
            &match e_machine_to_human_str(self.inner.e_machine) {
                Some(s) => s.to_string(),
                None => format!("e_machine({:#x})", self.inner.e_machine),
            },
        ));
        headers.push(FileHeader::new(
            "Entry point address",
            &format!("{:#x}", self.inner.e_entry),
        ));
        headers.push(FileHeader::new(
            "Start of program headers",
            &format!("{} (bytes into file)", self.inner.e_phoff),
        ));
        headers.push(FileHeader::new(
            "Start of section headers",
            &format!("{} (bytes into file)", self.inner.e_shoff),
        ));
        headers.push(FileHeader::new(
            "Flags",
            &format!("{:#x}", self.inner.e_flags),
        ));
        headers.push(FileHeader::new(
            "Size of this header",
            &format!("{} (bytes)", self.inner.e_ehsize),
        ));
        headers.push(FileHeader::new(
            "Size of program header",
            &format!("{} (bytes)", self.inner.e_phentsize),
        ));
        headers.push(FileHeader::new(
            "Number of program headers",
            &self.inner.e_phnum.to_string(),
        ));
        headers.push(FileHeader::new(
            "Size of section headers",
            &format!("{} (bytes)", self.inner.e_shentsize),
        ));
        headers.push(FileHeader::new(
            "Number of section headers",
            &self.inner.e_shnum.to_string(),
        ));
        headers.push(FileHeader::new(
            "Section headers string table section index",
            &self.inner.e_shstrndx.to_string(),
        ));
        headers
    }
}
