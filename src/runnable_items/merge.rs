//! # [`Merge`] Runnable
//!
//! This module provides basic data types to convert the [`crate::cli::subcommands::MergeArgs`]
//! into a runnable executor.
use colored::Colorize;

use crate::{
    cli::subcommands::MergeArgs,
    utils::{check::CheckableItem, print::PrintableItem},
};
use std::{ffi::OsStr, path::PathBuf};

/// Merge action executor. It stores data to be converted in input/output file paths and action
/// arguments.
#[derive(Debug, PartialEq)]
pub struct Merge {
    /// Path to all inputs to merge. Can be a single directory path or +2 pdf file paths.
    input: Vec<PathBuf>,
    /// Where to place the output merge.
    output: PathBuf,
    /// Override output file if it already exists.
    ovrrd: bool,
    /// Allow input repetition.
    repetition: bool,
    /// Catch files until the until the defined depth.
    depth: DepthVariant,
    /// Create parent dirs of the output if not exists.
    parent: bool,
    /// Merging order
    order: OrderMode,
}

impl TryFrom<MergeArgs> for Merge {
    type Error = MergeBuildError;
    fn try_from(value: MergeArgs) -> Result<Self, Self::Error> {
        let MergeArgs {
            input,
            output,
            override_output,
            allow_repetition,
            depth,
            parent,
            order_by,
        } = value;
        let input = if input.is_empty() {
            Err(MergeBuildError::InputIsEmpty)
        } else {
            Ok(input
                .into_iter()
                .map(|entry| PathBuf::from(entry.trim()))
                .collect())
        }?;
        let output = if let Some(o) = output {
            Ok(PathBuf::from(o.trim()))
        } else {
            Err(MergeBuildError::OutputIsEmpty)
        }?;
        let ovrrd = override_output;
        let repetition = allow_repetition;
        let depth = if let Some(d) = depth {
            let d = d.trim();
            if d == "*" {
                Ok(DepthVariant::Infinite)
            } else {
                match d.trim().parse::<usize>() {
                    Ok(0) => Err(MergeBuildError::UnparseableDepth(d.into())),
                    Ok(d) => Ok(DepthVariant::UntilLayer(d)),
                    _ => Err(MergeBuildError::UnparseableDepth(d.into())),
                }
            }
        } else {
            Ok(DepthVariant::NotSpecified)
        }?;
        let order = OrderMode::try_from(order_by)?;
        Ok(Merge {
            input,
            output,
            ovrrd,
            repetition,
            depth,
            parent,
            order,
        })
    }
}

impl CheckableItem for Merge {
    type CheckableOutput = Result<(), MergeCheckError>;
    fn check_item(&self) -> Self::CheckableOutput {
        {
            let mut inps = self.input.iter().take(2);
            match (inps.next(), inps.next()) {
                (Some(p), None) if p.is_file() => Err(MergeCheckError::InputIsSingleFile(p.into())),
                _ => Ok(()),
            }?;
            for p in &self.input {
                match (p, p.extension()) {
                    (path, _ext)
                        if path.iter().any(|subdir| {
                            subdir == OsStr::new(".") || subdir == OsStr::new("..")
                        }) =>
                    {
                        Err(MergeCheckError::InputIsDirectoryReference(
                            path.to_path_buf(),
                        ))
                    }
                    (path, ext) if ext.is_some_and(|e| e != OsStr::new("pdf")) => {
                        Err(MergeCheckError::InputIsNotPdfFile(path.to_path_buf()))
                    }
                    (_, _) => Ok(()),
                }?;
            }
            Ok(())
        }?;

        match &self.output {
            x if x.is_dir() => Err(MergeCheckError::OutputIsDirectory(x.into())),
            x if x.extension() != Some(OsStr::new("pdf")) => {
                Err(MergeCheckError::OutputIsNotPdfFile(x.into()))
            }
            x if x
                .iter()
                .any(|subdir| subdir == OsStr::new(".") || subdir == OsStr::new("..")) =>
            {
                Err(MergeCheckError::OutputIsDirectoryReference(x.into()))
            }
            _ => Ok(()),
        }?;

        if !self.repetition {
            let mut remains = self.input.clone();
            let mut pop = remains.pop();
            while let Some(p) = pop {
                if remains.contains(&p) {
                    return Err(MergeCheckError::InputRepetitionWithoutFlag(p));
                }
                pop = remains.pop();
            }
        }

        let mut buf = PathBuf::from(".");
        for dir in self.output.iter().take(self.output.iter().count() - 1) {
            buf.push(dir);
            if !buf.exists() && !self.parent {
                return Err(MergeCheckError::ParentOutputWithoutFlag(
                    self.output.clone(),
                ));
            }
        }

        match self.output.try_exists() {
            Ok(true) if !self.ovrrd => {
                Err(MergeCheckError::OutputAlreadyExists(self.output.clone()))
            }
            Err(_) => Err(MergeCheckError::CouldNotReadOrCheckFilePath(
                self.output.clone(),
            )),
            Ok(_) => Ok(()),
        }
    }
}

/// How deep catch files (only works for directory path inputs).
#[derive(Debug, PartialEq)]
enum DepthVariant {
    /// Depth was not specified (no problem if all inputs are only files).
    NotSpecified,
    /// Catch directory PDF files until reach the [`DepthVariant::UntilLayer::0`] layer.
    UntilLayer(usize),
    /// Go all input directory layers ahead.
    Infinite,
}

/// Decide how to order the input merging.
#[derive(Default, Debug, PartialEq)]
enum OrderMode {
    /// Alphabetically
    Alpha,
    /// From most old to most recent.
    DateTime,
    /// Not specified (use input list order).
    #[default]
    Default,
}

impl TryFrom<Option<String>> for OrderMode {
    type Error = MergeBuildError;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        let val = if let Some(v) = value {
            v.trim().to_lowercase()
        } else {
            return Ok(Self::default());
        };
        match val.as_str() {
            "alpha" => Ok(Self::Alpha),
            "datetime" => Ok(Self::DateTime),
            "def" => Ok(Self::default()),
            _ => Err(MergeBuildError::UnparseableOrderMode(val)),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const NO_INPUT: [&str; 3] = ["merge", "-o", "output.pdf"];
    const NO_OUTPUT: [&str; 3] = ["merge", "-i", "input.pdf"];
    const UNPARSEABLE_DEPTH: [&str; 7] = ["merge", "-i", "inputs", "-o", "some.pdf", "-d", "0"];
    const UNPARSEABLE_ORDER_MODE: [&str; 7] = [
        "merge",
        "-i",
        "dir",
        "-o",
        "some.pdf",
        "--order-by",
        "invalid",
    ];
    const SINGLE_FILE_INPUT: [&str; 5] = ["merge", "-i", "f.pdf", "-o", "out.pdf"];
    const OUTPUT_IS_DIR: [&str; 5] = ["merge", "-i", "directory", "-o", "src"];
    const INPUT_DIRECTORY_REFERENCE: [&str; 6] =
        ["merge", "-i", "somedir/../this.pdf", "dir", "-o", "out.pdf"];
    const TXT_INPUT: [&str; 6] = ["merge", "-i", "pdf.pdf", "text.txt", "-o", "out.pdf"];
    const INPUT_REPETITION: [&str; 6] = ["merge", "-i", "src", "src", "-o", "out.pdf"];
    const ALREADY_EXISTING_OUTPUT: [&str; 5] = ["merge", "-i", "src", "-o", "f.pdf"];
    const PARENT_OUTPUT: [&str; 5] = ["merge", "-i", "src", "-o", "some/f.pdf"];

    #[test]
    fn merge_try_from() {
        [
            (
                MergeArgs::from_iter(NO_INPUT),
                MergeBuildError::InputIsEmpty,
            ),
            (
                MergeArgs::from_iter(NO_OUTPUT),
                MergeBuildError::OutputIsEmpty,
            ),
            (
                MergeArgs::from_iter(UNPARSEABLE_DEPTH),
                MergeBuildError::UnparseableDepth("0".into()),
            ),
            (
                MergeArgs::from_iter(UNPARSEABLE_ORDER_MODE),
                MergeBuildError::UnparseableOrderMode("invalid".into()),
            ),
        ]
        .into_iter()
        .for_each(|(m, err_variant)| assert!(Merge::try_from(m).is_err_and(|e| e == err_variant)));
    }

    #[test]
    fn merge_check() {
        match Path::new("f.pdf").try_exists() {
            Ok(false) => {
                panic!(
                    "this test requires a `f.pdf` at crate root. create it and try again...\n(can be empty)"
                )
            }
            Err(_) => panic!("failed to check the `f.pdf` file. This is an unexpected behavior"),
            _ => {}
        }
        [
            (
                MergeArgs::from_iter(SINGLE_FILE_INPUT),
                MergeCheckError::InputIsSingleFile("f.pdf".into()),
            ),
            (
                MergeArgs::from_iter(OUTPUT_IS_DIR),
                MergeCheckError::OutputIsDirectory("src".into()),
            ),
            (
                MergeArgs::from_iter(INPUT_DIRECTORY_REFERENCE),
                MergeCheckError::InputIsDirectoryReference("somedir/../this.pdf".into()),
            ),
            (
                MergeArgs::from_iter(TXT_INPUT),
                MergeCheckError::InputIsNotPdfFile("text.txt".into()),
            ),
            (
                MergeArgs::from_iter(INPUT_REPETITION),
                MergeCheckError::InputRepetitionWithoutFlag("src".into()),
            ),
            (
                MergeArgs::from_iter(ALREADY_EXISTING_OUTPUT),
                MergeCheckError::OutputAlreadyExists("f.pdf".into()),
            ),
            (
                MergeArgs::from_iter(PARENT_OUTPUT),
                MergeCheckError::ParentOutputWithoutFlag("some/f.pdf".into()),
            ),
        ]
        .into_iter()
        .for_each(|(m, err)| {
            let m = Merge::try_from(m.clone())
                .unwrap_or_else(|_| panic!("This was expected to be OK {:?}", m));
            assert_eq!(m.check_item(), Err(err))
        });
    }
}
