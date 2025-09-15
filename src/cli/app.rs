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
                let merge = Merge::try_from(args);
                if let Err(e) = merge {
                    Printer::set_err(true);
                    Printer::title(PrintableTag::Error, Some(e.clone()));
                    Printer::blankln(1);
                    match e {
                        MergeBuildError::InputIsEmpty | MergeBuildError::OutputIsEmpty => {
                            tips::merge_input_output();
                        }
                        MergeBuildError::UnparseableDepth(_) => {
                            tips::merge_depth();
                        }
                        MergeBuildError::UnparseableOrderMode(_) => {
                            tips::merge_order();
                        }
                        // allow this for future implementations
                        #[allow(unreachable_patterns)]
                        _ => todo!("Code must be implemented..."),
                    }
                    return AppOutput::Err;
                }
            }
        }
        AppOutput::Ok
    }
}
