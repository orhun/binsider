use crate::elf::Property;
use elf::{dynamic::Dyn, endian::AnyEndian, parse::ParsingTable, ParseError};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// ELF dynamic section wrapper.
#[derive(Clone, Debug)]
pub struct Dynamic {
    /// Dynamics.
    dynamics: Vec<Dyn>,
}

impl<'a> TryFrom<Option<ParsingTable<'a, AnyEndian, Dyn>>> for Dynamic {
    type Error = ParseError;
    fn try_from(value: Option<ParsingTable<'a, AnyEndian, Dyn>>) -> Result<Self, Self::Error> {
        let parsing_table = value.ok_or_else(|| {
            ParseError::IOError(IoError::new(
                IoErrorKind::Other,
                "parsing table does not exist",
            ))
        })?;
        Ok(Self {
            dynamics: parsing_table.iter().collect(),
        })
    }
}

impl<'a> Property<'a> for Dynamic {
    fn items(&self) -> Vec<Vec<String>> {
        self.dynamics
            .iter()
            .map(|dynamic| {
                let d_tag_str = elf::to_str::d_tag_to_str(dynamic.d_tag)
                    .map_or(format!("{:#X?}", dynamic.d_tag), |val| val.to_string());
                vec![
                    format!("{d_tag_str}").trim_start_matches("DT_").to_string(),
                    format!("{:#X?}", dynamic.clone().d_val()),
                ]
            })
            .collect()
    }
}
