use crate::utils::print::PrintableItem;
use colored::Colorize;
use std::path::PathBuf;

/// Struct to report merging success.
#[derive(Clone)]
pub struct RunSuccess {
    pub files: Vec<PathBuf>,
    pub seconds: f64,
    pub output: PathBuf,
}

impl RunSuccess {
    /// Generate a new [`RunSuccess`].
    pub fn new(files: Vec<PathBuf>, seconds: f64, output: PathBuf) -> Self {
        Self {
            files,
            seconds,
            output,
        }
    }
}

impl std::fmt::Display for RunSuccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "merged successfully ({} files)",
            self.files.len().to_string().cyan()
        )
    }
}

impl PrintableItem for RunSuccess {}
