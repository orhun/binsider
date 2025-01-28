use crate::elf::Property;
use elf::{
    endian::AnyEndian,
    relocation::{Rel, Rela},
    ElfBytes, ParseError,
};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// ELF relocations wrapper.
#[derive(Clone, Debug, Default)]
pub struct Relocations {
    /// Relocations.
    rels: Vec<Rel>,
    /// Relocations with addend.
    relas: Vec<Rela>,
}

impl<'a> TryFrom<&'a ElfBytes<'a, AnyEndian>> for Relocations {
    type Error = ParseError;
    fn try_from(elf: &'a ElfBytes<'a, AnyEndian>) -> Result<Self, Self::Error> {
        let parsing_table = elf.section_headers().ok_or_else(|| {
            ParseError::IOError(IoError::new(
                IoErrorKind::Other,
                "parsing table does not exist",
            ))
        })?;
        Ok(Self {
            rels: parsing_table
                .iter()
                .filter(|shdr| shdr.sh_type == elf::abi::SHT_REL)
                .flat_map(|v| {
                    elf.section_data_as_rels(&v)
                        .expect("failed to read rels section")
                        .collect::<Vec<Rel>>()
                })
                .collect(),
            relas: parsing_table
                .iter()
                .filter(|shdr| shdr.sh_type == elf::abi::SHT_RELA)
                .flat_map(|v| {
                    elf.section_data_as_relas(&v)
                        .expect("failed to read relas section")
                        .collect::<Vec<Rela>>()
                })
                .collect(),
        })
    }
}

impl Property<'_> for Relocations {
    fn items(&self) -> Vec<Vec<String>> {
        let mut relocations = Vec::new();
        self.rels.iter().for_each(|v| {
            relocations.push(vec![
                format!("{:#X?}", v.r_type),
                format!("{:#X?}", v.r_sym),
                format!("{:#X?}", v.r_offset),
                String::from("-"),
            ]);
        });
        self.relas.iter().for_each(|v| {
            relocations.push(vec![
                format!("{:#X?}", v.r_type),
                format!("{:#X?}", v.r_sym),
                format!("{:#X?}", v.r_offset),
                format!("{:#X?}", v.r_addend),
            ]);
        });
        relocations
    }
}
