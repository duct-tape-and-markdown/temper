use std::fs;
use std::path::Path;

/// Create all parent directories of `path` and write `bytes` to it.
pub(crate) fn write_creating_parents(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)
}
