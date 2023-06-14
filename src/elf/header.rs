use elf::to_str::*;
use elf::{endian::AnyEndian, file::FileHeader};

/// An ELF header entry.
#[derive(Debug)]
pub struct Header {
    /// Name of the header.
    pub name: String,
    /// Value of the header.
    pub value: String,
}

impl Header {
    /// Constructs a new instance.
    pub fn new<S: Into<String>>(name: S, value: S) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// ELF header wrapper.
#[derive(Debug)]
pub struct Headers {
    /// Inner type.
    inner: FileHeader<AnyEndian>,
}

impl From<FileHeader<AnyEndian>> for Headers {
    fn from(inner: FileHeader<AnyEndian>) -> Self {
        Self { inner }
    }
}

impl Headers {
    /// Returns the headers.
    pub fn get(&self) -> Vec<Header> {
        let mut headers = Vec::new();
        headers.push(Header::new("Class", &format!("{:?}", self.inner.class)));
        headers.push(Header::new(
            "Endianness",
            &format!("{:?}", self.inner.endianness),
        ));
        headers.push(Header::new(
            "Version: {}",
            &match self.inner.version {
                1 => String::from("1 (current)"),
                v => v.to_string(),
            },
        ));
        headers.push(Header::new(
            "OS/ABI",
            e_osabi_to_string(self.inner.osabi)
                .strip_prefix("ELFOSABI_")
                .unwrap_or("unknown"),
        ));
        headers.push(Header::new(
            "ABI Version",
            &self.inner.abiversion.to_string(),
        ));
        headers.push(Header::new(
            "Type",
            &match e_type_to_human_str(self.inner.e_type) {
                Some(s) => s.to_string(),
                None => format!("e_type({:#x})", self.inner.e_type),
            },
        ));
        headers.push(Header::new(
            "Arch",
            &match e_machine_to_human_str(self.inner.e_machine) {
                Some(s) => s.to_string(),
                None => format!("e_machine({:#x})", self.inner.e_machine),
            },
        ));
        headers.push(Header::new(
            "Entry point address",
            &format!("{:#x}", self.inner.e_entry),
        ));
        headers.push(Header::new(
            "Start of program headers",
            &format!("{} (bytes into file)", self.inner.e_phoff),
        ));
        headers.push(Header::new(
            "Start of section headers",
            &format!("{} (bytes into file)", self.inner.e_shoff),
        ));
        headers.push(Header::new("Flags", &format!("{:#x}", self.inner.e_flags)));
        headers.push(Header::new(
            "Size of this header",
            &format!("{} (bytes)", self.inner.e_ehsize),
        ));
        headers.push(Header::new(
            "Size of program header",
            &format!("{} (bytes)", self.inner.e_phentsize),
        ));
        headers.push(Header::new(
            "Number of program headers",
            &self.inner.e_phnum.to_string(),
        ));
        headers.push(Header::new(
            "Size of section headers",
            &format!("{} (bytes)", self.inner.e_shentsize),
        ));
        headers.push(Header::new(
            "Number of section headers",
            &self.inner.e_shnum.to_string(),
        ));
        headers.push(Header::new(
            "Section headers string table section index",
            &self.inner.e_shstrndx.to_string(),
        ));
        headers
    }
}
