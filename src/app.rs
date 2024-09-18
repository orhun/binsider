use crate::{
    elf::Elf,
    error::{Error, Result},
    file::FileInfo,
    tui::event::Event,
};
use elf::{endian::AnyEndian, ElfBytes};
use heh::app::Application as Heh;
use heh::decoder::Encoding;
use lddtree::DependencyAnalyzer;
use ratatui::text::Line;
use rust_strings::BytesConfig;
use std::{
    fmt::{self, Debug, Formatter},
    path::PathBuf,
    sync::mpsc,
    thread,
};

/// Tracer data.
#[derive(Debug, Default)]
pub struct TraceData {
    /// System calls.
    pub syscalls: Vec<u8>,
    /// Summary.
    pub summary: Vec<u8>,
}

/// Binary analyzer.
pub struct Analyzer<'a> {
    /// List of files that are being analyzed.
    pub files: Vec<PathBuf>,
    /// Current file information.
    pub file: FileInfo<'a>,
    /// Elf properties.
    pub elf: Elf,
    /// Strings.
    pub strings: Option<Vec<(u64, String)>>,
    /// Min length of the strings.
    pub strings_len: usize,
    /// Heh application.
    pub heh: Heh,
    /// Tracer data.
    pub tracer: TraceData,
    /// System calls.
    pub system_calls: Vec<Line<'a>>,
    /// Library dependencies.
    pub dependencies: Vec<(String, String)>,
}

impl Debug for Analyzer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Analyzer")
            .field("bytes", &self.file.bytes)
            .finish()
    }
}

impl<'a> Analyzer<'a> {
    /// Constructs a new instance.
    pub fn new(
        mut file_info: FileInfo<'a>,
        strings_len: usize,
        files: Vec<PathBuf>,
    ) -> Result<Self> {
        let elf_bytes = ElfBytes::<AnyEndian>::minimal_parse(file_info.bytes)?;
        let elf = Elf::try_from(elf_bytes)?;
        let heh = Heh::new(file_info.open_file()?, Encoding::Ascii, 0)
            .map_err(|e| Error::HexdumpError(e.to_string()))?;
        Ok(Self {
            dependencies: Self::extract_libs(&file_info)?,
            files,
            file: file_info,
            elf,
            strings: None,
            strings_len,
            heh,
            tracer: TraceData::default(),
            system_calls: Vec::new(),
        })
    }

    /// Extracts the library dependencies.
    pub fn extract_libs(file_info: &FileInfo<'a>) -> Result<Vec<(String, String)>> {
        let mut dependencies = DependencyAnalyzer::default()
            .analyze(file_info.path)?
            .libraries
            .clone()
            .into_iter()
            .map(|(name, lib)| {
                (
                    name.to_string(),
                    lib.realpath
                        .unwrap_or(lib.path)
                        .to_string_lossy()
                        .to_string(),
                )
            })
            .collect::<Vec<(String, String)>>();
        dependencies.sort_by(|a, b| {
            let lib_condition1 = a.0.starts_with("lib");
            let lib_condition2 = b.0.starts_with("lib");
            match (lib_condition1, lib_condition2) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.0.cmp(&b.0),
            }
        });
        Ok(dependencies)
    }

    /// Returns the sequences of printable characters.
    pub fn extract_strings(&mut self, event_sender: mpsc::Sender<Event>) {
        let config = BytesConfig::new(self.file.bytes.to_vec()).with_min_length(self.strings_len);
        thread::spawn(move || {
            event_sender
                .send(Event::FileStrings(
                    rust_strings::strings(&config).map_err(|e| Error::StringsError(e.to_string())),
                ))
                .expect("failed to send strings event");
        });
    }
}
