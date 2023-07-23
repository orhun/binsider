use elf::{endian::AnyEndian, note::Note, ElfBytes, ParseError};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// ELF symbols wrapper.
#[derive(Clone, Debug)]
pub struct Notes {
    /// Notes text.
    pub text: Vec<String>,
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
        let mut note_text = Vec::new();
        parsing_table
            .iter()
            .filter(|v| v.sh_type == elf::abi::SHT_NOTE)
            .for_each(|section_header| {
                let name = string_table
                    .get(section_header.sh_name as usize)
                    .expect("section name should parse");
                let notes = elf
                    .section_data_as_notes(&section_header)
                    .expect("Failed to read notes section");
                note_text.push(format!("Displaying notes found in: {name}"));
                for note in notes {
                    match note {
                        Note::GnuAbiTag(abi) => {
                            let os_str = elf::to_str::note_abi_tag_os_to_str(abi.os)
                                .map_or(format!("{}", abi.os), |val| val.to_string());
                            note_text.push(format!(
                                "    OS: {os_str}, ABI: {}.{}.{}",
                                abi.major, abi.minor, abi.subminor
                            ));
                        }
                        Note::GnuBuildId(build_id) => {
                            note_text.push(String::from("    Build ID: "));
                            for byte in build_id.0 {
                                note_text.push(format!("{byte:02x}"));
                            }
                        }
                        Note::Unknown(any) => {
                            note_text.extend(vec![
                                String::from("type"),
                                String::from("name"),
                                String::from("desc"),
                            ]);
                            note_text.extend(vec![
                                any.n_type.to_string(),
                                any.name.to_string(),
                                format!("{:02X?}", any.desc),
                            ]);
                        }
                    }
                }
            });
        Ok(Self { text: note_text })
    }
}
