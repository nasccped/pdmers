/// Common output that should be returned at the program's end. That's better than using some
/// [`Result`] variant since the result handling + final returning can be a little complex along
/// the development.
pub enum AppOutput {
    Ok,
    Err,
}

/* Since [`std::process::exit`] accepts only integers ([`i32`]), it's convenient to convert our
 * output type to an integer. */
impl From<AppOutput> for i32 {
    fn from(value: AppOutput) -> Self {
        match value {
            AppOutput::Ok => 0,
            AppOutput::Err => 1,
        }
    }
}
