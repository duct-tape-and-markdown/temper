//! Shared scaffolding for in-src `#[cfg(test)] mod tests` blocks.

use std::path::PathBuf;

/// A fresh, empty temp directory, uniquely named via the sanctioned `tempfile`
/// crate rather than a hand-rolled counter+pid scheme.
pub(crate) fn tmpdir(label: &str) -> PathBuf {
    tempfile::Builder::new()
        .prefix(label)
        .tempdir()
        .expect("failed to create temp dir")
        .keep()
}
