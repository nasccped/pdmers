use super::{app_output::AppOutput, subcommands::MergeArgs};
use crate::{
    runnable_items::merge::{Merge, MergeBuildError},
    utils::{
        print::{PrintableTag, Printer},
        tips,
    },
};
use clap::Parser;

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
        Printer::title(PrintableTag::Warning, Some("this is a beta testing..."));
        let cmd = self.0;
        if cmd.is_empty_call() {
            Printer::set_err(true);
            Printer::title(PrintableTag::Error, Some("no arguments provided"));
            Printer::blankln(1);
            tips::help_tip();
            return AppOutput::Err;
        }

        let _action = match Merge::try_from(cmd) {
            Ok(v) => v,
            Err(e) => {
                Self::handle_merge_try_from_error(e);
                return AppOutput::Err;
            }
        };

        AppOutput::Ok
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
            MergeBuildError::UnparseableOrderMode(_) => tips::merge_order(),
            // allow this for future implementations
            #[allow(unreachable_patterns)]
            _ => todo!("Code must be implemented..."),
        }
    }
}
