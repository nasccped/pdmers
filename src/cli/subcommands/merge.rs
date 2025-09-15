use clap::Args;

#[derive(Args)]
pub struct MergeArgs {
    /// Inputs to be passed to our program. They're get as a [`Vec<String>`], then the
    /// `--input file1 file2 file3` will works as expected (better than passing
    /// `"file1, file2, file3"`). They're not required when the subcommand is called, but required
    /// by the `Merge` runnable item (read [`crate::runnable_items`]), otherwise it'll fail at
    /// [`crate::utils::check::CheckableItem`] trait's function.
    #[arg(
        long,
        short,
        num_args = 1..,
        value_delimiter = ' ',
        value_name = "FILES|DIRS",
        help = "PDF files to be merged"
    )]
    pub input: Vec<String>,

    /// Place merged inputs at.
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

    /// Order files by alphanumeric, datetime or default (input order)
    #[arg(long, value_name = "ALPHA|DATETIME|DEF")]
    pub order_by: Option<String>,
}
