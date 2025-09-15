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

/// Tips for `--order-by` flag usage.
pub fn merge_order() {
    Printer::echoln(format!(
        "Possible values for `{}` flag are:",
        "--order-by".green()
    ));
    let possible_values = [
        ("alpha", "order input files alphabetically"),
        ("datetime", "order inputs by most old to most recent"),
        (
            "def",
            &format!(
                "default ordering (as passed to `{}` flag)",
                "--input".green()
            ),
        ),
    ];
    let as_string: String = possible_values
        .iter()
        .map(|(v, desc)| format!(" {} {} {desc}", v.cyan(), " ".repeat(8 - v.len())))
        .collect::<Vec<_>>()
        .join("\n");
    Printer::echoln(as_string);
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

/// Tips when no subcommand is provided.
pub fn no_subcommand() {
    Printer::echoln(format!(
        "consider using `{}` to get usage tips!",
        "pdmers --help".green()
    ));
}
