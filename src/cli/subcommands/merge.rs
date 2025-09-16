use super::super::styles::APP_STYLE;
use clap::Parser;

/// Arguments for pdf merging
#[derive(Parser, Default, Debug, PartialEq)]
#[command(
    name = "pdmers",
    author,
    version = env!("CARGO_PKG_VERSION"),
    about,
    styles = APP_STYLE
)]
pub struct MergeArgs {
    /// PDF files to be merged.
    #[arg(
        long,
        short,
        num_args = 1..,
        value_delimiter = ' ',
        value_name = "FILES|DIRS",
    )]
    pub input: Vec<String>,

    /// Where to place the output file.
    #[arg(long, short, required = false)]
    pub output: Option<String>,

    /// Override output file if it already exists.
    #[arg(long = "override")]
    pub override_output: bool,

    /// Pass same input twice.
    #[arg(long)]
    pub allow_repetition: bool,

    /// Merge PDFs until reaches the `N` directory layer (use `*` to infinity).
    #[arg(long, short, value_name = "N", required = false)]
    pub depth: Option<String>,

    /// Creates parent directories of the output file (if they don't exists).
    #[arg(long, short)]
    pub parent: bool,

    /// Order files alphabetically, datetime or default (input order)
    #[arg(long, value_name = "ALPHA|DATETIME|DEF")]
    pub order_by: Option<String>,
}

impl MergeArgs {
    /// Test if no args was passed.
    pub fn is_empty_call(&self) -> bool {
        &MergeArgs::default() == self
    }
}
