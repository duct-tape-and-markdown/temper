use std::fmt;
use std::ops::Deref;
use std::path::{Component, Path, PathBuf};

/// Lexically normalize a path — drop `.` and resolve `..` against a preceding normal
/// segment — **without touching disk**: a provenance path need not exist under the
/// check CWD, and both the index keys and a resolved target must normalize the identical
/// way to join. A leading `..` with nothing to pop is kept, so an out-of-tree target
/// stays distinct rather than silently rooting.
#[must_use]
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut out: Vec<Component> = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir if matches!(out.last(), Some(Component::Normal(_))) => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out.into_iter().collect()
}

/// A harness-relative path: the lock's own vocabulary for source paths. Always
/// `/`-separated, no leading `./`, normalized. This is the canonical form that's
/// committed to lock.toml and shared across Projection, RollupEntry, RawLockRow,
/// ProvenanceRow, and EmitOwnedEntry — so a path fact's root is its type, not its
/// doc comment.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HarnessRelativePath(String);

impl HarnessRelativePath {
    /// Create a new harness-relative path from a string. The string should already
    /// be in canonical form (harness-relative, `/`-separated, no leading `./`).
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl Deref for HarnessRelativePath {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for HarnessRelativePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<Path> for HarnessRelativePath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for HarnessRelativePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for HarnessRelativePath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<HarnessRelativePath> for String {
    fn from(p: HarnessRelativePath) -> Self {
        p.0
    }
}

impl From<&str> for HarnessRelativePath {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
