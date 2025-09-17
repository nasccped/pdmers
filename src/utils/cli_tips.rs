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
    Printer::echoln("Output must be a single pdf file path.");
    Printer::blankln(1);
    Printer::echoln(format!(
        "{}: `{}`",
        "ie".green(),
        "pdmers -i integrals.pdf derivatives.pdf -o math.pdf".cyan()
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

/// Tips when passing directory references.
pub fn directory_references() {
    Printer::echoln(format!(
        "By default, directory references {} (this avoid path",
        "aren't allowed".red()
    ));
    Printer::echoln("exploit and infinity directory traversal).");
    Printer::blankln(1);
    Printer::echoln(format!(
        "You should avoid '{}' for current path. Just use the dir/file",
        ".".cyan()
    ));
    Printer::echoln("name instead.");
    Printer::blankln(1);
    Printer::echoln(format!(
        "{}: if you want to merge all PDFs of curdir, move they",
        "ie".green()
    ));
    Printer::echoln(format!(
        "    to a new directory and then use this dir as {}.",
        "input".cyan()
    ))
}

/// Tips when `--allow-repetition` flag is required.
pub fn repetition_flag() {
    Printer::echoln("This prevents duplicates content within the output");
    Printer::echoln(format!(
        "file (works both for {} and {} paths).",
        "input".cyan(),
        "file".cyan()
    ));
    Printer::blankln(1);
    Printer::echoln("If you're sure about what you're doing, use the");
    Printer::echoln(format!("`{}` flag.", "--allow-repetition".green()));
}

/// Tips when `--override` flag is required.
pub fn override_flag() {
    Printer::echoln("This prevents from accidentaly override a pdf file");
    Printer::blankln(1);
    Printer::echoln("If you're sure about what you're doing, use the");
    Printer::echoln(format!("`{}` flag.", "--override".green()));
}
