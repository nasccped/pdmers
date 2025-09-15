use clap::Parser;
mod cli;
use std::process;
mod runnable_items;
mod utils;

fn exit_with_code(code: i32) {
    process::exit(code);
}

fn main() {
    let app = cli::App::parse();
    let exit = app.run_pdmers();
    exit_with_code(exit.into());
}
