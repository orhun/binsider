use clap::Parser;
use std::path::PathBuf;

use crate::tui::ui::Tab;

/// Argument parser powered by [`clap`].
#[derive(Clone, Debug, Default, Parser)]
#[clap(
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
)]
pub struct Args {
    /// Binary / ELF object file.
    #[arg(env, name = "FILE")]
    pub files: Vec<PathBuf>,

    /// Minimum length of strings.
    #[arg(env, short = 'n', long = "min-len", default_value = "15")]
    pub min_strings_len: usize,

    /// The initial application tab to open.
    #[arg(env, short = 't', long = "tab", default_value = "general")]
    pub tab: Tab,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn test_args() {
        Args::command().debug_assert();
    }
}
