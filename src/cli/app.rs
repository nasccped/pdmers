use super::{app_output::AppOutput, styles::APP_STYLE, subcommands::AppSubcommand};
use crate::{
    runnable_items::merge::{Merge, MergeBuildError},
    utils::{
        print::{PrintableTag, Printer},
        tips,
    },
};
use clap::Parser;

/// App's entrypoint.
#[derive(Parser)]
#[command(
    name = "pdmers",
    author,
    version = env!("CARGO_PKG_VERSION"),
    about,
    styles = APP_STYLE
)]
pub struct App {
    /// Subcommands that the app can receives.
    #[command(subcommand)]
    subcommand: Option<AppSubcommand>,
}

impl App {
    /// Function that actually runs this project (better than using `run` or `run_app` since they
    /// already exists).
    pub fn run_pdmers(self) -> AppOutput {
        Printer::title(PrintableTag::Warning, Some("this is a beta testing..."));
        let subcommand = if let Some(s) = self.subcommand {
            s
        } else {
            Printer::set_err(true);
            Printer::title(PrintableTag::Error, Some("no subcommand/flag provided"));
            Printer::blankln(1);
            tips::no_subcommand();
            return AppOutput::Err;
        };
        match subcommand {
            AppSubcommand::MergeSubcommand(args) => {
                let _merge = match Merge::try_from(args) {
                    Ok(m) => m,
                    Err(e) => {
                        Self::handle_merge_try_from_error(e);
                        return AppOutput::Err;
                    }
                };
            }
        }
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
