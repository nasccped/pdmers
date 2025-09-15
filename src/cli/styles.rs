use clap::builder::{Styles, styling};

/// The program's default style.
pub const APP_STYLE: Styles = Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Yellow.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());
