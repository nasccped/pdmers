use crate::utils::print::PrintableItem;
use colored::Colorize;
use std::path::PathBuf;

/// Possible errors when trying to convert a [`MergeArgs`] into a [`Merge`] struct.
#[derive(Clone, Debug, PartialEq)]
pub enum MergeBuildError {
    /// When no input was passed.
    InputIsEmpty,
    /// When no output was passed.
    OutputIsEmpty,
    /// When the depth input is "unparseable".
    UnparseableDepth(String),
    /// When the order mode input is "unparseable".
    UnparseableOrderMode(String),
}

impl std::fmt::Display for MergeBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MergeBuildError::InputIsEmpty => "input path(s) wasn't provided".into(),
                MergeBuildError::OutputIsEmpty => "output path wasn't provided".into(),
                MergeBuildError::UnparseableDepth(d) =>
                    format!("couldn't parse the `depth` value (`{}`)", d.bright_cyan()),
                MergeBuildError::UnparseableOrderMode(o) => format!(
                    "couldn't parse the `order_by` value (`{}`)",
                    o.bright_cyan()
                ),
            }
        )
    }
}

impl PrintableItem for MergeBuildError {}

/// Common errors when checking the [`Merge`] before run.
#[derive(Clone, PartialEq, Debug)]
pub enum MergeCheckError {
    /// When a single pdf file is passed as input.
    InputIsSingleFile(PathBuf),
    /// When output isn't the expected value (pdf file path)
    OutputIsDirectory(PathBuf),
    /// When input contains a directory reference (not allowed).
    InputIsDirectoryReference(PathBuf),
    /// When output contains a directory reference (not allowed).
    OutputIsDirectoryReference(PathBuf),
    /// When a non `.pdf` file is passed as input.
    InputIsNotPdfFile(PathBuf),
    /// When a non `.pdf` file is passed as output.
    OutputIsNotPdfFile(PathBuf),
    /// When trying to repeat an input file path without `--allow-repetition` flag.
    InputRepetitionWithoutFlag(PathBuf),
    /// When passing an already existing output path without the `--override` flag.
    OutputAlreadyExists(PathBuf),
    /// When the file path couldn't be read (due to permissions or time-out reasons).
    CouldNotReadOrCheckFilePath(PathBuf),
    /// When output contains parent dir withou `--parent` flag.
    ParentOutputWithoutFlag(PathBuf),
}

impl std::fmt::Display for MergeCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InputIsSingleFile(p) => format!(
                    "single file input isn't allowed (`{}`)",
                    p.to_string_lossy().bright_cyan()
                ),
                Self::OutputIsDirectory(p) => format!(
                    "output as directory isn't allowed (`{}`)",
                    p.to_string_lossy().bright_cyan()
                ),
                Self::InputIsDirectoryReference(p) | Self::OutputIsDirectoryReference(p) =>
                    format!(
                        "directory reference isn't allowed (`{}`)",
                        p.to_string_lossy().bright_cyan()
                    ),
                Self::InputIsNotPdfFile(p) | Self::OutputIsNotPdfFile(p) => {
                    format!(
                        "non pdf file argument (`{}`)",
                        p.to_string_lossy().bright_cyan()
                    )
                }
                Self::InputRepetitionWithoutFlag(p) => {
                    format!(
                        "you passed the same input more than once (`{}`)",
                        p.to_string_lossy().bright_cyan()
                    )
                }
                Self::OutputAlreadyExists(p) => {
                    format!(
                        "output already exists (`{}`)",
                        p.to_string_lossy().bright_cyan()
                    )
                }
                Self::CouldNotReadOrCheckFilePath(p) => {
                    format!(
                        "couldn't read/check file path (`{}`)",
                        p.to_string_lossy().bright_cyan()
                    )
                }
                Self::ParentOutputWithoutFlag(p) => {
                    format!(
                        "output contains parent dir (`{}`)",
                        p.to_string_lossy().bright_green()
                    )
                }
            }
        )
    }
}

impl PrintableItem for MergeCheckError {}
