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

/// The lexical path from `root` to `target` — the spelling a lock row carries when a
/// source dependency sits outside the harness tree. Both sides are normalized
/// ([`normalize_path`]), their shared leading segments dropped, and every root segment
/// left over becomes a `..`, so a target above the root reads `../../src/x.txt` rather
/// than one machine's absolute path. Joining that row back onto whatever root a reader
/// holds reaches the same place, which is what keeps a committed lock reproducible
/// across checkouts.
///
/// Disk is never touched: the caller resolves symlinks before it asks (a lexical
/// derivation cannot), and a path that does not exist relativizes exactly as one that
/// does.
///
/// `None` when no relative path exists — the two sides sit under roots no run of `..`
/// climbs between (on Windows, different drive prefixes), or the root keeps a leading
/// `..` [`normalize_path`] had nothing to pop, naming a parent this side cannot spell.
#[must_use]
pub fn relative_from_root(root: &Path, target: &Path) -> Option<PathBuf> {
    let root = normalize_path(root);
    let target = normalize_path(target);
    let mut root_rest = root.components().peekable();
    let mut target_rest = target.components().peekable();
    while let (Some(from), Some(to)) = (root_rest.peek(), target_rest.peek()) {
        if from != to {
            break;
        }
        root_rest.next();
        target_rest.next();
    }

    let mut relative = PathBuf::new();
    for component in root_rest {
        match component {
            Component::Normal(_) => relative.push(".."),
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) | Component::ParentDir => return None,
        }
    }
    for component in target_rest {
        match component {
            Component::Normal(segment) => relative.push(segment),
            Component::ParentDir => relative.push(".."),
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    if relative.as_os_str().is_empty() {
        relative.push(".");
    }
    Some(relative)
}

/// Relativize a file path against a harness root. If the path is absolute, strip the root
/// prefix and return the relative path. If the path is already relative, return it as-is.
/// All backslashes are normalized to forward slashes.
/// Returns `None` if an absolute path is not under the root, or if a relative root cannot
/// be resolved against the working directory.
#[must_use]
pub fn relativize_against_root(file_path: &str, root: &Path) -> Option<String> {
    let file_path_normalized = file_path.replace('\\', "/");
    let file_path_buf = PathBuf::from(&file_path_normalized);

    // If the path is not absolute, it's already relative — return as-is.
    if !file_path_buf.is_absolute() {
        return Some(file_path_normalized);
    }

    let root_absolute = absolute_root(root)?;

    file_path_buf
        .strip_prefix(&root_absolute)
        .ok()
        .map(|rel_path| rel_path.to_string_lossy().replace('\\', "/").to_string())
}

/// The harness root as an absolute, lexically normalized path. A relative root — `.`, the
/// form the installed guard hook command carries — is joined onto the working directory
/// first: [`normalize_path`] erases a lone `.` entirely, and stripping that empty prefix
/// succeeds against *every* absolute path, leaving it unrelativized. Disk is never touched,
/// so a root that does not exist resolves the same way any other does.
fn absolute_root(root: &Path) -> Option<PathBuf> {
    let absolute = if root.is_absolute() {
        root.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(root)
    };
    Some(normalize_path(&absolute))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_target_above_the_root_is_spelled_with_parent_segments() {
        let relative = relative_from_root(
            Path::new("/checkout/packages/agent"),
            Path::new("/checkout/src/x.txt"),
        );
        assert_eq!(relative, Some(PathBuf::from("../../src/x.txt")));
    }

    #[test]
    fn a_target_under_the_root_keeps_its_plain_relative_spelling() {
        let relative = relative_from_root(
            Path::new("/checkout/agent"),
            Path::new("/checkout/agent/docs/included.md"),
        );
        assert_eq!(relative, Some(PathBuf::from("docs/included.md")));
    }

    #[test]
    fn a_root_naming_a_parent_it_cannot_spell_has_no_relative_path() {
        // The reachable stand-in for the case the refusal exists for: a root segment no
        // run of `..` climbs from. A Windows drive prefix is the real shape, and `Path`
        // parses no prefix off a Unix host, so the un-poppable leading `..` stands in.
        assert_eq!(
            relative_from_root(Path::new("../outside"), Path::new("/checkout/x.txt")),
            None
        );
    }
}
