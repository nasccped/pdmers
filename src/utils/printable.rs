use colored::Colorize;

/// Local bool to decide where to print [`Printer::blankln`].
///
/// This variable only changes within this module and it's not visible outside here.
static mut ERR_PRINT: bool = false;

/// Function used to change the [`ERR_PRINT`] value.
fn set_err_print(value: bool) {
    unsafe {
        ERR_PRINT = value;
    }
}

/// Common object (`struct`) for printing at terminal (helps to avoid [`println`] calls
/// indiscriminately).
///
/// ## Panics
///
/// All function within this struct will only fail if the panics occurs at the `stdout` access (if
/// the print macros fails, the program will crash).
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
pub struct Printer;

impl Printer {
    /// Prints the item to `stdout` with a new line.
    pub fn outln<T: PrintableItem>(item: T) {
        println!("{}", item);
    }
    /// Prints the item to `stdout` WITHOUT new line.
    pub fn out<T: PrintableItem>(item: T) {
        print!("{}", item);
    }
    /// Prints the item to `stderr` with a new line.
    pub fn errln<T: PrintableItem>(item: T) {
        eprintln!("{}", item);
    }
    /// Prints the item to `stderr` WITHOUT new line.
    pub fn err<T: PrintableItem>(item: T) {
        eprint!("{}", item);
    }
    /// Private function to handle [`PrintableTag`] printing (since this enum doesn't implement
    /// [`PrintableItem`]).
    fn print_tag(tag: &PrintableTag) {
        match tag {
            PrintableTag::Error => eprint!("{tag} "),
            _ => print!("{tag} "),
        }
    }
    /// Prints a tag title (similar to cargo's behavior) with an optional message.
    pub fn title<T: PrintableItem>(tag: PrintableTag, message: Option<T>) {
        Self::print_tag(&tag);
        let func = match tag {
            PrintableTag::Error => {
                set_err_print(true);
                Self::err
            }
            _ => Self::out,
        };
        if let Some(m) = message {
            func(m);
        }
        Self::blankln(1);
        set_err_print(false);
    }
    /// Prints a blank line (to `stdout` or `stderr`, depending on `is_err` arg) `n` times. If `n`
    /// is 0, prints nothing.
    pub fn blankln(n: usize) {
        let content = "\n".repeat(n);
        unsafe {
            if ERR_PRINT {
                eprint!("{content}");
            } else {
                print!("{content}");
            }
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
