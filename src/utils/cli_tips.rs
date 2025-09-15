use super::print::Printer;
use colored::Colorize;

/// Tips for `--input` and `--output` flags usage.
pub fn merge_input_output() {
    Printer::echoln("Input should be at least 1 directory path or 2 pdf file paths.");
    Printer::echoln(format!(
        "Example: `{}`",
        "pdmers merge -i integrals.pdf derivatives.pdf -o math.pdf".green()
    ))
}

/// Tips for `--depth` flag usage.
pub fn merge_depth() {
    Printer::echoln(format!(
        "The `{}` flag must always be preceded with a positive number",
        "--depth".green()
    ));
    Printer::echoln(format!("or the infinity repr (`{}`).", "*".green()));
}

/// Tips when no subcommand is provided.
pub fn no_subcommand() {
    Printer::echoln(format!(
        "consider using `{}` to get usage tips!",
        "pdmers --help".green()
    ));
}
