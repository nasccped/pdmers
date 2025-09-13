/// Common trait to add check feature to specific items at runtime.
///
/// This trait prevents actions to be ran if a given item isn't _"safe enough"_.
/// ```no_run
/// # static value: bool = true;
/// # struct SomeType {
/// #     value: bool
/// # }
/// enum Output {
///     AllFunny,
///     HorribleError
/// }
/// impl CheckableItem for SomeType {
///     type CheckableOutput = Output;
///     fn check_item(&self) -> Output {
///         if self.value {
///             Output::AllFunny
///         } else {
///             Output::HorribleError
///         }
///     }
/// }
/// ```
/// Strongly recommend [`CheckableItem::CheckableOutput`] to be an `enum` or [`Result`] type.
pub trait CheckableItem {
    /// Type to be returned after item check.
    type CheckableOutput;
    /// Checks if the item is safe.
    fn check_item(&self) -> Self::CheckableOutput;
}
