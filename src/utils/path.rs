//! # Path utils
use std::path::{Path, PathBuf};

/// Normalize a path (turns `./some.file` into `some.file`).
pub fn normalize_path_buf(source: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    source.components().for_each(|comp| match comp {
        std::path::Component::CurDir => {}
        other => normalized.push(other),
    });
    normalized
}
