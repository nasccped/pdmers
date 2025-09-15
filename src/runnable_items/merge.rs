//! # [`Merge`] Runnable
//!
//! This module provides basic data types to convert the [`crate::cli::subcommands::MergeArgs`]
//! into a runnable executor.
use colored::Colorize;

use crate::{cli::subcommands::MergeArgs, utils::print::PrintableItem};
use std::path::PathBuf;

/// Merge action executor. It stores data to be converted in input/output file paths and action
/// arguments.
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

/// How deep catch files (only works for directory path inputs).
enum DepthVariant {
    /// Depth was not specified (no problem if all inputs are only files).
    NotSpecified,
    /// Catch directory PDF files until reach the [`DepthVariant::UntilLayer::0`] layer.
    UntilLayer(usize),
    /// Go all input directory layers ahead.
    Infinite,
}

/// Decide how to order the input merging.
#[derive(Default)]
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
#[derive(Clone, Debug)]
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
                    format!("couldn't parse the `depth` value (`{}`)", d.bright_red()),
                MergeBuildError::UnparseableOrderMode(o) =>
                    format!("couldn't parse the `order_by` value (`{}`)", o.bright_red()),
            }
        )
    }
}

impl PrintableItem for MergeBuildError {}
