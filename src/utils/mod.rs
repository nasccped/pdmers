//! Utilities for the `pdmers` program.

mod checkable;
mod cli_tips;
mod printable;
mod runnable_item;

/// # Printing utilities
///
/// Provides a set of printing features, such as:
/// - a common struct that holds the printing responsibility ([`Printer`])
/// - a common trait that allows complex items (`struct`, `enum`, `...`) to be printed
///   ([PrintableItem])
/// - a file enum to redirect printing output ([`StdWhere`])
/// - a set of tags for pretty fast printing ([`PrintableTag`])
pub mod print {
    pub use super::printable::{PrintableItem, PrintableTag, Printer};
}

/// # Running utilities
///
/// Provides a common trait to turn a type _"runnable"_ ([`RunnableItem`]).
pub mod run {
    pub use super::runnable_item::RunnableItem;
}

/// # Check utilities
///
/// Provides the [`CheckableItem`] trait.
pub mod check {
    pub use super::checkable::CheckableItem;
}

/// # Tips utilities
///
/// Common tips to be print at the terminal.
pub mod tips {
    pub use super::cli_tips::*;
}
