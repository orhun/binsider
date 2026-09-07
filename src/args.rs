use clap::Parser;
use ratatui::style::Color;
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

    /// Accent color of the application.
    #[arg(env, long, value_name = "COLOR")]
    pub accent_color: Option<Color>,

    /// Dynamic analysis options.
    #[cfg(feature = "dynamic-analysis")]
    #[command(flatten)]
    pub trace: TraceArgs,
}

/// Options for formatting dynamic analysis output.
#[cfg(feature = "dynamic-analysis")]
#[derive(Clone, Debug, Default, clap::Args)]
pub struct TraceArgs {
    /// Display system call numbers during dynamic analysis.
    #[arg(long)]
    pub syscall_number: bool,

    /// Print un-abbreviated strings during dynamic analysis.
    #[arg(long)]
    pub no_abbrev: bool,

    /// Maximum string argument size to print during dynamic analysis.
    #[arg(long, conflicts_with = "no_abbrev")]
    pub string_limit: Option<usize>,
}

#[cfg(feature = "dynamic-analysis")]
impl From<TraceArgs> for lurk_cli::args::Args {
    fn from(args: TraceArgs) -> Self {
        Self {
            syscall_number: args.syscall_number,
            no_abbrev: args.no_abbrev,
            string_limit: args.string_limit,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn test_args() {
        Args::command().debug_assert();
    }

    #[cfg(feature = "dynamic-analysis")]
    #[test]
    fn test_trace_options() {
        let args = Args::try_parse_from([
            "binsider",
            "--syscall-number",
            "--string-limit",
            "128",
            "-n",
            "8",
            "/bin/ls",
        ])
        .expect("trace options should parse");
        assert_eq!(args.min_strings_len, 8);
        assert_eq!(args.files, vec![PathBuf::from("/bin/ls")]);
        let trace: lurk_cli::args::Args = args.trace.into();
        assert!(trace.syscall_number);
        assert_eq!(trace.string_limit, Some(128));
        assert!(!trace.no_abbrev);

        let args =
            Args::try_parse_from(["binsider", "--no-abbrev"]).expect("trace options should parse");
        let trace: lurk_cli::args::Args = args.trace.into();
        assert!(trace.no_abbrev);
        assert_eq!(trace.string_limit, None);
    }

    #[cfg(feature = "dynamic-analysis")]
    #[test]
    fn test_trace_option_validation() {
        let error = Args::try_parse_from(["binsider", "--no-abbrev", "--string-limit", "128"])
            .expect_err("abbreviation options must conflict");
        assert_eq!(error.kind(), clap::error::ErrorKind::ArgumentConflict);
        assert!(Args::try_parse_from(["binsider", "--string-limit", "invalid"]).is_err());
        let trace: lurk_cli::args::Args = Args::try_parse_from(["binsider"])
            .expect("default options should parse")
            .trace
            .into();
        assert!(!trace.syscall_number);
        assert!(!trace.no_abbrev);
        assert_eq!(trace.string_limit, None);
    }
}
