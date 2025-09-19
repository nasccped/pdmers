use super::{app_output::AppOutput, subcommands::MergeArgs};
use crate::{
    runnable_items::merge::{Merge, MergeBuildError, MergeCheckError, MergeRunError},
    utils::{
        check::CheckableItem,
        print::{PrintableTag, Printer},
        run::RunnableItem,
        tips,
    },
};
use clap::Parser;
use colored::Colorize;

pub struct App(pub MergeArgs);

impl App {
    /// Since the [`App`] is just a wrapper, we'll need a new parse for this object, calling the
    /// [`MergeArgs`] parsing.
    pub fn parse() -> Self {
        Self(MergeArgs::parse())
    }

    /// Function that actually runs this project (better than using `run` or `run_app` since they
    /// already exists).
    pub fn run_pdmers(self) -> AppOutput {
        let cmd = self.0;
        if cmd.is_empty_call() {
            Printer::set_err(true);
            Printer::title(PrintableTag::Error, Some("no arguments provided"));
            Printer::blankln(1);
            tips::help_tip();
            return AppOutput::Err;
        }

        let action = match Merge::try_from(cmd) {
            Ok(v) => v,
            Err(e) => {
                Self::handle_merge_try_from_error(e);
                return AppOutput::Err;
            }
        };

        if let Err(e) = action.check_item() {
            Self::handle_merge_check_error(e);
            return AppOutput::Err;
        }

        match action.run_item() {
            Ok(success) => {
                Printer::title(PrintableTag::Done, Some(success.clone()));
                Printer::echoln("The files:");
                Printer::blankln(1);
                for f in success.files {
                    Printer::echoln(format!(" - {}", f.to_string_lossy().cyan()));
                }
                Printer::blankln(1);
                Printer::echoln(format!(
                    "was {} merged into `{}`. (took {} second(s))",
                    "successfully".green(),
                    success.output.to_string_lossy().cyan(),
                    format!("{:.3}", success.seconds).cyan()
                ));
                AppOutput::Ok
            }
            Err(e) => {
                Self::handle_merge_run_error(e);
                AppOutput::Err
            }
        }
    }

    /// Print the suitable tip by a given [`MergeBuildError`] variant.
    fn handle_merge_try_from_error(value: MergeBuildError) {
        Printer::set_err(true);
        Printer::title(PrintableTag::Error, Some(value.clone()));
        Printer::blankln(1);
        match value {
            MergeBuildError::InputIsEmpty | MergeBuildError::OutputIsEmpty => {
                tips::merge_input_output()
            }
            MergeBuildError::UnparseableDepth(_) => tips::merge_depth(),
            // allow this for future implementations
            #[allow(unreachable_patterns)]
            _ => todo!("Code must be implemented..."),
        }
    }

    /// Print the suitable tip by a given [`MergeCheckError`] variant.
    fn handle_merge_check_error(value: MergeCheckError) {
        Printer::set_err(true);
        Printer::title(PrintableTag::Error, Some(value.clone()));
        Printer::blankln(1);
        match value {
            MergeCheckError::InputIsSingleFile(_)
            | MergeCheckError::OutputIsDirectory(_)
            | MergeCheckError::InputIsNotPdfFile(_)
            | MergeCheckError::OutputIsNotPdfFile(_) => tips::merge_input_output(),
            MergeCheckError::InputIsDirectoryReference(_)
            | MergeCheckError::OutputIsDirectoryReference(_) => tips::directory_references(),
            MergeCheckError::InputRepetitionWithoutFlag(_) => tips::repetition_flag(),
            MergeCheckError::OutputAlreadyExists(_) => tips::override_flag(),
            MergeCheckError::CouldNotReadOrCheckFilePath(_) => tips::non_readable_file_path(),
            MergeCheckError::ParentOutputWithoutFlag(_) => tips::parent_flag_usage(),
            MergeCheckError::DepthNotSpecified => tips::depth_flag_usage(),
        }
    }

    /// Print the suitable tip by a given [`MergeRunError`] variant.
    fn handle_merge_run_error(value: MergeRunError) {
        Printer::set_err(true);
        Printer::title(PrintableTag::Error, Some(value.clone()));
        if let MergeRunError::EntryDoesNotExists(_path) = value {
            return;
        }
        Printer::blankln(1);
        match value {
            MergeRunError::CouldNotReadEntry(_) => {
                tips::non_readable_file_path();
            }
            MergeRunError::PathRepetitionWithoutFlag(_) => {
                tips::repetition_flag();
            }
            MergeRunError::CouldNotLoadInput(_)
            | MergeRunError::RootPageNotFound
            | MergeRunError::CatalogIsNone => {
                tips::could_not_handle_pdf();
            }
            MergeRunError::CouldNotSaveTheOutput(_) => {
                tips::could_not_save_pdf();
            }
            // This isn't necessarry since the function already stoped at this variant
            MergeRunError::EntryDoesNotExists(_) => {}
        }
    }
}
