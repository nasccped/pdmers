use super::print::Printer;
use colored::Colorize;

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
    Printer::echoln("This prevents from accidentaly override a pdf file.");
    Printer::blankln(1);
    Printer::echoln("If you're sure about what you're doing, use the");
    Printer::echoln(format!("`{}` flag.", "--override".green()));
}

/// Warn when path reading fails
pub fn non_readable_file_path() {
    Printer::echoln("This occurs when the program fails to read a file/dir");
    Printer::echoln(format!(
        "path due to {} or {} reasons.",
        "timeout".red(),
        "privileges".red()
    ));
}

/// Tips for `--parent` flag usage
pub fn parent_flag_usage() {
    Printer::echoln("If the directory path to the output file doesn't exists,");
    Printer::echoln("this fail will occur.");
    Printer::blankln(1);
    Printer::echoln("If you're sure about what you're doing, use");
    Printer::echoln(format!("the `{}` flag.", "--parent".green()));
}

/// When PDF file handling (load + save) fails.
pub fn could_not_handle_pdf() {
    Printer::echoln(format!(
        "This occurs when PDF file handling fails (within `{}`",
        "lopdf".cyan()
    ));
    Printer::echoln("library).");
    Printer::blankln(1);
    Printer::echoln(format!("The reason can be {},", "bad formatting".red()));
    Printer::echoln(format!(
        "{} privileges, {} file, etc.",
        "not enough".red(),
        "empty".red()
    ));
    Printer::blankln(1);
    Printer::echoln("If you think this is a bug, consider");
    Printer::echoln(format!(
        "opening a {} at {}",
        "pull-request".cyan(),
        "https://github.com/nasccped/pdmers".cyan()
    ));
}

/// Tips for `--depth` usage.
pub fn depth_flag_usage() {
    Printer::echoln("This occurs when trying to access a directory");
    Printer::echoln(format!(
        "without specifying the `{}` flag.",
        "--depth".green()
    ));
    Printer::blankln(1);
    Printer::echoln(format!(
        "The `{}` must always be greater than {}.",
        "depth".green(),
        "0".cyan()
    ));
    Printer::echoln(format!(
        "The app will catch PDF files until the {}th layer.",
        "N".cyan()
    ));
}

/// When PDF saving fails.
pub fn could_not_save_pdf() {
    Printer::echoln(format!(
        "This usually happens at '{}' privileges",
        "not enough".red()
    ));
    Printer::echoln("environments.");
    Printer::blankln(1);
    Printer::echoln(format!("Can also be mem. issue {}", "._.".cyan()));
}
