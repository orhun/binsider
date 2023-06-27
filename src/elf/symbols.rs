use crate::elf::Property;
use elf::{
    endian::AnyEndian, gnu_symver::SymbolVersionTable, parse::ParsingTable,
    string_table::StringTable, symbol::Symbol, ParseError,
};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

/// ELF symbols wrapper.
#[derive(Clone, Debug)]
pub struct Symbols {
    /// Symbols.
    symbols: Vec<Symbol>,
    /// Symbol names.
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
            symbols: parsing_table.iter().collect(),
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
        self.symbols
            .iter()
            .enumerate()
            .map(|(i, symbol)| {
                let name = self.names[i].to_string();
                vec![
                    format!("{:#x}", symbol.st_value),
                    symbol.st_size.to_string(),
                    elf::to_str::st_symtype_to_string(symbol.st_symtype())
                        .trim_start_matches("STT_")
                        .to_string(),
                    elf::to_str::st_bind_to_string(symbol.st_bind())
                        .trim_start_matches("STB_")
                        .to_string(),
                    elf::to_str::st_vis_to_string(symbol.st_vis())
                        .trim_start_matches("STV_")
                        .to_string(),
                    symbol.st_shndx.to_string(),
                    format!(
                        "{}{}",
                        name.chars().take(15).collect::<String>(),
                        if name.is_empty() { "" } else { "[...]" }
                    ),
                ]
            })
            .collect()
    }
}

/// ELF dynamic symbols wrapper.
#[derive(Clone, Debug)]
pub struct DynamicSymbols {
    /// Symbols.
    symbols: Vec<Symbol>,
    /// Names.
    names: Vec<String>,
    /// Requirements.
    requirements: Vec<String>,
}

impl<'a>
    TryFrom<(
        Option<(ParsingTable<'a, AnyEndian, Symbol>, StringTable<'a>)>,
        Option<SymbolVersionTable<'a, AnyEndian>>,
    )> for DynamicSymbols
{
    type Error = ParseError;
    fn try_from(
        value: (
            Option<(ParsingTable<'a, AnyEndian, Symbol>, StringTable<'a>)>,
            Option<SymbolVersionTable<'a, AnyEndian>>,
        ),
    ) -> Result<Self, Self::Error> {
        let (parsing_table, string_table) = value.0.ok_or_else(|| {
            ParseError::IOError(IoError::new(
                IoErrorKind::Other,
                "symbol table does not exist",
            ))
        })?;
        let version_table = value.1.ok_or_else(|| {
            ParseError::IOError(IoError::new(
                IoErrorKind::Other,
                "symbol version table does not exist",
            ))
        })?;
        Ok(Self {
            symbols: parsing_table.iter().collect(),
            names: parsing_table
                .iter()
                .map(|v| {
                    string_table
                        .get(v.st_name as usize)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|_| String::from("unknown"))
                })
                .collect(),
            requirements: parsing_table
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if v.is_undefined() {
                        version_table
                            .get_requirement(i)
                            .ok()
                            .flatten()
                            .map(|v| v.name)
                            .unwrap_or_else(|| "unknown")
                    } else {
                        "-"
                    }
                    .to_string()
                })
                .collect(),
        })
    }
}

impl<'a> Property<'a> for DynamicSymbols {
    fn items(&self) -> Vec<Vec<String>> {
        self.symbols
            .iter()
            .enumerate()
            .map(|(i, symbol)| {
                vec![
                    format!("{:#x}", symbol.st_value),
                    symbol.st_size.to_string(),
                    elf::to_str::st_symtype_to_string(symbol.st_symtype())
                        .trim_start_matches("STT_")
                        .to_string(),
                    elf::to_str::st_bind_to_string(symbol.st_bind())
                        .trim_start_matches("STB_")
                        .to_string(),
                    elf::to_str::st_vis_to_string(symbol.st_vis())
                        .trim_start_matches("STV_")
                        .to_string(),
                    symbol.st_shndx.to_string(),
                    self.requirements[i].to_string(),
                    self.names[i].to_string(),
                ]
            })
            .collect()
    }
}
