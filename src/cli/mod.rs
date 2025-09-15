//! # The program's CLI pieces.
//!
//! A lot of code is private, but the actual program ([`App`]) is public and can be used.
mod app;
mod app_output;
mod styles;
pub mod subcommands;

pub use app::App;
