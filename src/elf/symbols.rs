use crate::elf::Property;
use elf::{
    endian::AnyEndian, parse::ParsingTable, string_table::StringTable, symbol::Symbol, ParseError,
};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// ELF file file header wrapper.
#[derive(Clone, Debug)]
pub struct Symbols {
    /// Inner type.
    inner: Vec<Symbol>,
    /// Section names.
    names: Vec<String>,
}

impl<'a> TryFrom<Option<(ParsingTable<'a, AnyEndian, Symbol>, StringTable<'a>)>> for Symbols {
    type Error = ParseError;
    fn try_from(
        value: Option<(ParsingTable<'a, AnyEndian, Symbol>, StringTable<'a>)>,
    ) -> Result<Self, Self::Error> {
        let (parsing_table, string_table) = value.ok_or_else(|| {
            ParseError::IOError(IoError::new(
                IoErrorKind::Other,
                "symbol table does not exist",
            ))
        })?;
        Ok(Self {
            inner: parsing_table.iter().collect(),
            names: parsing_table
                .iter()
                .map(|v| {
                    string_table
                        .get(v.st_name as usize)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|_| String::from("unknown"))
                })
                .collect(),
        })
    }
}

impl<'a> Property<'a> for Symbols {
    fn items(&self) -> Vec<Vec<String>> {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, symbol)| {
                vec![
                    format!("{:#x}", symbol.st_value),
                    symbol.st_size.to_string(),
                    elf::to_str::st_symtype_to_string(symbol.st_symtype()),
                    elf::to_str::st_bind_to_string(symbol.st_bind()),
                    elf::to_str::st_vis_to_string(symbol.st_vis()),
                    symbol.st_shndx.to_string(),
                    self.names[i].to_string(),
                ]
            })
            .collect()
    }
}
