use elf::{endian::AnyEndian, note::Note as ElfNote, ElfBytes, ParseError};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// Representation of an ELF note.
#[derive(Clone, Debug, Default)]
pub struct Note {
    /// Name of the note section.
    pub name: String,
    /// Header of the notes.
    pub header: Vec<String>,
    /// Contents of the note.
    pub text: Vec<String>,
}

/// ELF notes wrapper.
#[derive(Clone, Debug)]
pub struct Notes {
    /// Notes text.
    pub inner: Vec<Note>,
}

impl<'a> TryFrom<&'a ElfBytes<'a, AnyEndian>> for Notes {
    type Error = ParseError;
    fn try_from(elf: &'a ElfBytes<'a, AnyEndian>) -> Result<Self, Self::Error> {
        let section_headers = elf.section_headers_with_strtab()?;
        let (parsing_table, string_table) = (
            section_headers.0.ok_or_else(|| {
                ParseError::IOError(IoError::new(
                    IoErrorKind::Other,
                    "parsing table does not exist",
                ))
            })?,
            section_headers.1.ok_or_else(|| {
                ParseError::IOError(IoError::new(
                    IoErrorKind::Other,
                    "string table does not exist",
                ))
            })?,
        );
        let mut notes = Vec::new();
        parsing_table
            .iter()
            .filter(|v| v.sh_type == elf::abi::SHT_NOTE)
            .for_each(|section_header| {
                let name = string_table
                    .get(section_header.sh_name as usize)
                    .expect("section name should parse");
                let elf_notes = elf
                    .section_data_as_notes(&section_header)
                    .expect("Failed to read notes section");
                let mut note = Note {
                    name: format!("Displaying notes found in: {name}"),
                    ..Default::default()
                };
                for elf_note in elf_notes {
                    match elf_note {
                        ElfNote::GnuAbiTag(abi) => {
                            let os_str = elf::to_str::note_abi_tag_os_to_str(abi.os)
                                .map_or(format!("{}", abi.os), |val| val.to_string());
                            note.header
                                .extend(vec![String::from("OS"), String::from("ABI")]);
                            note.text.extend(vec![
                                os_str,
                                format!("{}.{}.{}", abi.major, abi.minor, abi.subminor),
                            ])
                        }
                        ElfNote::GnuBuildId(build_id) => {
                            note.header.extend(vec![String::from("Build ID")]);
                            note.text.extend(vec![build_id
                                .0
                                .iter()
                                .map(|v| format!("{v:02x}"))
                                .collect()]);
                        }
                        ElfNote::Unknown(any) => {
                            note.header.extend(vec![
                                String::from("Type"),
                                String::from("Name"),
                                String::from("Description"),
                            ]);
                            note.text.extend(vec![
                                any.n_type.to_string(),
                                any.name.to_string(),
                                format!("{:02X?}", any.desc),
                            ]);
                        }
                    }
                }
                notes.push(note);
            });
        Ok(Self { inner: notes })
    }
}
