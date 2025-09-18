//! # [`Merge`] Runnable
//!
//! This module provides basic data types to convert the [`crate::cli::subcommands::MergeArgs`]
//! into a runnable executor.
mod depth;
mod errors;
mod order_mode;

#[cfg(test)]
mod tests;

use crate::{cli::subcommands::MergeArgs, utils::check::CheckableItem};
use depth::Depth;
pub use errors::*;
use order_mode::OrderMode;
use std::ffi::OsStr;
use std::path::PathBuf;

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
    depth: Depth,
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
        let input = match input {
            x if x.is_empty() => Err(MergeBuildError::InputIsEmpty),
            x => Ok(x.into_iter().map(PathBuf::from).collect()),
        }?;
        let output = match output {
            Some(o) => Ok(PathBuf::from(o)),
            None => Err(MergeBuildError::OutputIsEmpty),
        }?;
        let ovrrd = override_output;
        let repetition = allow_repetition;
        let depth = depth.map_or(Ok(Depth::default()), |d| {
            Depth::try_from(d.trim().to_string())
        })?;
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
        // check input paths
        let mut inp_count = 0;
        let mut contains_dir = false;
        for path in &self.input {
            if path
                .iter()
                .any(|sd| sd == OsStr::new(".") || sd == OsStr::new(".."))
            {
                return Err(MergeCheckError::InputIsDirectoryReference(path.into()));
            }
            if path.is_file() && path.extension().is_some_and(|ext| ext != "pdf") {
                return Err(MergeCheckError::InputIsNotPdfFile(path.into()));
            }
            contains_dir |= path.is_dir();
            inp_count += 1;
        }
        if inp_count == 1 && !contains_dir {
            return Err(MergeCheckError::InputIsSingleFile((&self.input[0]).into()));
        }
        // check output paths
        let out = self.output.clone();
        if out.is_dir() {
            return Err(MergeCheckError::OutputIsDirectory(out));
        } else if out.extension().is_some_and(|ext| ext != "pdf") {
            return Err(MergeCheckError::OutputIsNotPdfFile(out));
        } else if out
            .iter()
            .any(|sd| sd == OsStr::new(".") || sd == OsStr::new(".."))
        {
            return Err(MergeCheckError::OutputIsDirectoryReference(out));
        }
        // check repetitions
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
        // check parent flag
        let mut buf = PathBuf::new();
        for dir in self.output.iter().take(self.output.iter().count() - 1) {
            buf.push(dir);
            if !buf.exists() && !self.parent {
                return Err(MergeCheckError::ParentOutputWithoutFlag(out));
            }
        }
        // finally: return dir not exists or if override is allowed
        self.output
            .try_exists()
            .map_err(|_| MergeCheckError::CouldNotReadOrCheckFilePath(out.clone()))
            .and_then(|res| {
                if !res || self.ovrrd {
                    Ok(())
                } else {
                    Err(MergeCheckError::OutputAlreadyExists(out))
                }
            })
    }
}
