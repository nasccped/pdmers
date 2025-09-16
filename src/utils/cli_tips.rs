use super::print::Printer;
use colored::Colorize;

const POSSIBLE_VALUES_AND_DESC: [(&str, &str); 3] = [
    ("alpha", "order input files alphabetically"),
    ("datetime", "order inputs by most old to most recent"),
    (
        "def",
        "default ordering (as passed to `\x1b[32m--input\x1b[0m` flag)",
    ),
];

/// Tips for `--input` and `--output` flags usage.
pub fn merge_input_output() {
    Printer::echoln("Input should be at least 1 directory path or 2 pdf file paths.");
    Printer::echoln("Output should be at least 1 pdf file path.");
    Printer::echoln(format!(
        "Example: `{}`",
        "pdmers -i integrals.pdf derivatives.pdf -o math.pdf".green()
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

/// Tips for `--order-by` flag usage.
pub fn merge_order() {
    Printer::echoln(format!(
        "Possible values for `{}` flag are:",
        "--order-by".green()
    ));
    Printer::echoln(
        POSSIBLE_VALUES_AND_DESC
            .iter()
            .map(|(v, desc)| format!(" {} {} {desc}", v.cyan(), " ".repeat(8 - v.len())))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    Printer::blankln(1);
    Printer::echoln(format!(
        "{}, if value not specified, '{}' will be used (default",
        "Also".yellow(),
        "def".cyan()
    ));
    Printer::echoln(format!(
        "behavior for directory input is '{}').",
        "alpha".cyan()
    ));
}

/// Tips when no args provided.
pub fn help_tip() {
    Printer::echoln(format!(
        "Try using `{}` to get usage tips!",
        "pdmers --help".green()
    ));
}
