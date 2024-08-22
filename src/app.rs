use crate::{
    elf::Elf,
    error::{Error, Result},
    file::FileInfo,
    tracer::TraceData,
    tui::event::Event,
};
use elf::{endian::AnyEndian, ElfBytes};
use heh::app::Application as Heh;
use heh::decoder::Encoding;
use lddtree::{DependencyAnalyzer, DependencyTree};
use ratatui::text::Line;
use rust_strings::BytesConfig;
use std::{
    fmt::{self, Debug, Formatter},
    path::PathBuf,
    sync::mpsc,
    thread,
};

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
    pub dependencies: DependencyTree,
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
        let dependencies = DependencyAnalyzer::default().analyze(file_info.path)?;
        Ok(Self {
            files,
            file: file_info,
            elf,
            strings: None,
            strings_len,
            heh,
            tracer: TraceData::default(),
            system_calls: Vec::new(),
            dependencies,
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn get_test_bytes() -> Result<Vec<u8>> {
        let debug_binary = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(env!("CARGO_PKG_NAME"));
        Ok(fs::read(debug_binary)?)
    }

    #[test]
    fn test_init() -> Result<()> {
        assert!(Analyzer::new(
            FileInfo::new("Cargo.toml", get_test_bytes()?.as_slice())?,
            4,
            vec![]
        )
        .is_ok());
        Ok(())
    }

    #[test]
    fn test_extract_strings() -> Result<()> {
        let test_bytes = get_test_bytes()?;
        let mut analyzer = Analyzer::new(
            FileInfo::new("Cargo.toml", test_bytes.as_slice())?,
            4,
            vec![],
        )?;
        let (tx, rx) = mpsc::channel();
        analyzer.extract_strings(tx);
        if let Event::FileStrings(strings) = rx.recv()? {
            assert!(strings?.iter().map(|(s, _)| s).any(|v| v == ".debug_str"));
        } else {
            panic!("strings did not succeed");
        }
        Ok(())
    }
}
