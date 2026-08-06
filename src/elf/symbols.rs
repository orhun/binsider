use crate::elf::Property;
use elf::{
    endian::AnyEndian, gnu_symver::SymbolVersionTable, parse::ParsingTable,
    string_table::StringTable, symbol::Symbol, ParseError,
};
use std::io::Error as IoError;

// Sorting criteria.
#[derive(Clone, Debug, Default)]
enum SortBy {
    /// Elf encounter order.
    #[default]
    None = 0,
    /// Name.
    Name = 1,
    /// Value.
    Value = 2,
}

/// ELF symbols wrapper.
#[derive(Clone, Debug, Default)]
pub struct Symbols {
    /// Symbols.
    symbols: Vec<Symbol>,
    /// Symbol names.
    names: Vec<String>,
    /// Sort by.
    sort: SortBy,
}

impl<'a> TryFrom<Option<(ParsingTable<'a, AnyEndian, Symbol>, StringTable<'a>)>> for Symbols {
    type Error = ParseError;
    fn try_from(
        value: Option<(ParsingTable<'a, AnyEndian, Symbol>, StringTable<'a>)>,
    ) -> Result<Self, Self::Error> {
        let (parsing_table, string_table) = value
            .ok_or_else(|| ParseError::IOError(IoError::other("symbol table does not exist")))?;
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
            sort: SortBy::default(),
        })
    }
}

impl Property<'_> for Symbols {
    fn items(&self) -> Vec<Vec<String>> {
        let mut indices: Vec<usize> = (0..self.symbols.len()).collect();
        match self.sort {
            SortBy::Name => {
                indices.sort_by(|&a, &b| self.names[a].cmp(&self.names[b]));
            }
            SortBy::Value => {
                indices.sort_by(|&a, &b| self.symbols[a].st_value.cmp(&self.symbols[b].st_value));
            }
            SortBy::None => {}
        }

        indices
            .iter()
            .map(|&i| {
                vec![
                    self.names[i].to_string(),
                    elf::to_str::st_symtype_to_string(self.symbols[i].st_symtype())
                        .trim_start_matches("STT_")
                        .to_string(),
                    format!("{:#x}", self.symbols[i].st_value),
                    self.symbols[i].st_size.to_string(),
                    elf::to_str::st_bind_to_string(self.symbols[i].st_bind())
                        .trim_start_matches("STB_")
                        .to_string(),
                    elf::to_str::st_vis_to_string(self.symbols[i].st_vis())
                        .trim_start_matches("STV_")
                        .to_string(),
                    self.symbols[i].st_shndx.to_string(),
                ]
            })
            .collect()
    }
}

impl Symbols {
    /// Cycle sorting criteria.
    pub fn cycle_sort(&mut self) {
        match self.sort {
            SortBy::None => self.sort = SortBy::Name,
            SortBy::Name => self.sort = SortBy::Value,
            SortBy::Value => self.sort = SortBy::None,
        }
    }
}

/// ELF dynamic symbols wrapper.
#[derive(Clone, Debug, Default)]
pub struct DynamicSymbols {
    /// Symbols.
    symbols: Vec<Symbol>,
    /// Names.
    names: Vec<String>,
    /// Requirements.
    requirements: Vec<String>,
    /// Sort by.
    sort: SortBy,
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
        let (parsing_table, string_table) = value
            .0
            .ok_or_else(|| ParseError::IOError(IoError::other("symbol table does not exist")))?;
        let version_table = value.1.ok_or_else(|| {
            ParseError::IOError(IoError::other("symbol version table does not exist"))
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
            sort: SortBy::default(),
        })
    }
}

impl Property<'_> for DynamicSymbols {
    fn items(&self) -> Vec<Vec<String>> {
        let mut indices: Vec<usize> = (0..self.symbols.len()).collect();
        match self.sort {
            SortBy::Name => {
                indices.sort_by(|&a, &b| self.names[a].cmp(&self.names[b]));
            }
            SortBy::Value => {
                indices.sort_by(|&a, &b| self.symbols[a].st_value.cmp(&self.symbols[b].st_value));
            }
            SortBy::None => {}
        }

        indices
            .iter()
            .map(|&i| {
                vec![
                    self.names[i].to_string(),
                    self.requirements[i].to_string(),
                    elf::to_str::st_symtype_to_string(self.symbols[i].st_symtype())
                        .trim_start_matches("STT_")
                        .to_string(),
                    format!("{:#x}", self.symbols[i].st_value),
                    self.symbols[i].st_size.to_string(),
                    elf::to_str::st_bind_to_string(self.symbols[i].st_bind())
                        .trim_start_matches("STB_")
                        .to_string(),
                    elf::to_str::st_vis_to_string(self.symbols[i].st_vis())
                        .trim_start_matches("STV_")
                        .to_string(),
                    self.symbols[i].st_shndx.to_string(),
                ]
            })
            .collect()
    }
}

impl DynamicSymbols {
    /// Cycle sorting criteria.
    pub fn cycle_sort(&mut self) {
        match self.sort {
            SortBy::None => self.sort = SortBy::Name,
            SortBy::Name => self.sort = SortBy::Value,
            SortBy::Value => self.sort = SortBy::None,
        }
    }
}
