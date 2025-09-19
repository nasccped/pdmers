/// Turn any specific type into a _"runnable"_ item.
///
/// With this trait, you can implement a subcommand functionality right bellow it's definition:
/// ```no_run
/// enum SomeItem {
///     IsOk,
///     IsErr,
/// }
/// impl RunnableItem for SomeItem {
///     type ArgType = ();
///     type Output = i32;
///     // Converts this type into exit code (int)
///     fn run_item(self) -> i32 {
///         match self {
///             SomeItem::IsOk => 0,
///             SomeItem::_ => 1,
///         }
///     }
///     fn run_with_arg(self, arg: ()) -> i32 {
///         panic!("This function shouldn't be called!");
///     }
/// }
/// ```
/// This trait provides two different functions:
/// - [`RunnableItem::run_item`] to just run the item
/// - [`RunnableItem::run_with_arg`] to run when an argument is necessary
///
/// # Tip
///
/// When you need just one of the functions mentioned above, consider panicking the other one. The
/// [`RunnableItem::ArgType`] and [`RunnableItem::Output`] must always be defined. If they're not
/// required, just use unit type (`()`).
pub trait RunnableItem {
    /// Type of the argument used within the [`RunnableItem::run_with_arg`] function.
    type ArgType;
    /// Type of the value returned at the `run`'s functions end.
    type Output;
    /// Run the item and return an output.
    fn run_item(self) -> Self::Output;
    /// Run the item (with an argument) and return an output.
    #[allow(dead_code)]
    fn run_with_arg(self, arg: Self::ArgType) -> Self::Output;
}
