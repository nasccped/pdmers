mod merge;

use clap::Subcommand;
pub use merge::MergeArgs;

#[derive(Subcommand)]
pub enum AppSubcommand {
    #[command(
        name = "merge",
        about = "Merge two or more PDFs into a single one",
        visible_alias = "m"
    )]
    MergeSubcommand(MergeArgs),
}
