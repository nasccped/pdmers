use colored::Colorize;

/// If the content should be printed to `stderr`.
static mut PRINT_TO_STDERR: bool = false;

/// Common object (`struct`) for printing at terminal (helps to avoid [`println`] calls
/// indiscriminately).
///
/// ## Panics
///
/// All function within this struct will only fail if the panics occurs at the `std(out|err)` access
/// or when trying to do `unsafe` access to local `PRINT_TO_STDERR` variable.
///
/// ## Usage tips
///
/// To use any of it's functions that requires an "item" argument, it's item must implement the
/// [`PrintableItem`] trait (which is just a [`std::fmt::Display`] trait alias). If the item type
/// is local to the current module, you will implement the trait locally:
/// ```no_run
/// # type SomeType = i32;
/// impl PrintableItem for SomeType {}
/// ```
/// But make sure that the `SomeType` already implements [`std::fmt::Display`] trait, otherwise the
/// `impl` will fail!
///
/// # Also
///
/// If the item variable is an instance of a foreign type (ie, from a lib), you must implement this
/// trait within the [`Printer`]'s [`self`] module or solve the trait system constraint with an
/// intermediate trait/type.
///
/// The [`Printer`] will print to `stdout` by default. If redirects to `stderr` is needed, use the
/// [`Printer::set_err`] function.
pub struct Printer;

impl Printer {
    /// Prints the item to `std(out|err)` WITH a new line.
    pub fn echoln<T: PrintableItem>(item: T) {
        unsafe {
            if PRINT_TO_STDERR {
                eprintln!("{}", item);
            } else {
                println!("{}", item);
            }
        }
    }
    /// Prints the item to `std(out|err)` WITHOUT a new line.
    pub fn echo<T: PrintableItem>(item: T) {
        unsafe {
            if PRINT_TO_STDERR {
                eprint!("{}", item);
            } else {
                print!("{}", item);
            }
        }
    }
    /// Private function to handle [`PrintableTag`] printing (since this enum doesn't implement
    /// [`PrintableItem`]).
    fn print_tag(tag: PrintableTag) {
        unsafe {
            if PRINT_TO_STDERR {
                eprint!("{tag} ");
            } else {
                print!("{tag} ");
            }
        }
    }
    /// Prints a tag title (similar to cargo's behavior) with an optional message.
    pub fn title<T: PrintableItem>(tag: PrintableTag, message: Option<T>) {
        Self::print_tag(tag);
        match message {
            Some(m) => Self::echoln(m),
            _ => Self::blankln(1),
        }
    }
    /// Prints a blank line `n` times. If `n` is 0, prints nothing.
    pub fn blankln(n: usize) {
        let content = "\n".repeat(n);
        unsafe {
            if PRINT_TO_STDERR {
                eprint!("{content}");
            } else {
                print!("{content}");
            }
        }
    }
    /// Change the local `PRINT_TO_STDERR` variable (set printing to `std(out|err)`).
    pub fn set_err(val: bool) {
        unsafe {
            PRINT_TO_STDERR = val;
        }
    }
}

/// Trait alias for _"inheritance-like"_ behavior.
///
/// Only types that implements this traits can be used within [`Printer`] functions.
///
/// A bunch of types can implement [`std::fmt::Display`] trait, but just specific and necessary
/// ones can implement this trait.
pub trait PrintableItem: std::fmt::Display {}

// Already implementing for common primitive types
impl PrintableItem for &str {}
impl PrintableItem for String {}
impl PrintableItem for i32 {}
impl PrintableItem for f64 {}
impl PrintableItem for usize {}

/// Stylized tags to be printed (by the [`Printer::title`] function). Printing
/// implementation is at [`std::fmt::Display`] block.
pub enum PrintableTag {
    /// When the task was done successfully.
    Done,
    /// When something went wrong but the task was still done.
    Warning,
    /// When something went wrong and the task was aborted.
    Error,
}

impl std::fmt::Display for PrintableTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:",
            match self {
                Self::Done => "done".bright_green(),
                Self::Warning => "warning".bright_yellow(),
                Self::Error => "error".bright_red(),
            }
        )
    }
}
