use super::MergeBuildError;

/// Decide how to order the input merging.
#[derive(Default, Debug, PartialEq)]
pub enum OrderMode {
    /// Alphabetically
    Alpha,
    /// From most old to most recent.
    DateTime,
    /// Not specified (use input list order).
    #[default]
    Default,
}

impl TryFrom<Option<String>> for OrderMode {
    type Error = MergeBuildError;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        let val = if let Some(v) = value {
            v.trim().to_lowercase()
        } else {
            return Ok(Self::default());
        };
        match val.as_str() {
            "alpha" => Ok(Self::Alpha),
            "datetime" => Ok(Self::DateTime),
            "def" => Ok(Self::default()),
            _ => Err(MergeBuildError::UnparseableOrderMode(val)),
        }
    }
}
