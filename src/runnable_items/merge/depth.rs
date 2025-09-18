use super::MergeBuildError;

/// How deep catch files (only works for directory path inputs).
#[derive(Debug, PartialEq, Default)]
pub enum Depth {
    /// Depth was not specified (no problem if all inputs are only files).
    #[default]
    NotSpecified,
    /// Catch directory PDF files until reach the [`Depth::Max::0`] layer.
    Max(usize),
    /// Go all input directory layers ahead.
    Infinite,
}

impl TryFrom<String> for Depth {
    type Error = MergeBuildError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.as_str() == "*" {
            Ok(Self::Infinite)
        } else {
            match value.parse() {
                Err(_) => Err(MergeBuildError::UnparseableDepth(value)),
                // 0 as depth is alson invalid
                Ok(0) => Err(MergeBuildError::UnparseableDepth(value)),
                Ok(d) => Ok(Self::Max(d)),
            }
        }
    }
}
