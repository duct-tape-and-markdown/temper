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

/// The canonical spelling of a path — `/`-separated, with no leading `./` — the one
/// home for the form [`HarnessRelativePath`] declares. Every producer of that form
/// spells through here: a committed `lock.toml` row, a guard diagnostic's path, a
/// governed locus' pattern, the repo file set.
///
/// `Path::join` inserts the host separator at each join boundary (backslash on
/// Windows) — left alone, that forks a byte-committed path by host.
#[must_use]
pub fn to_canonical_spelling(path: &Path) -> String {
    canonical_spelling(&path.to_string_lossy())
}

/// [`to_canonical_spelling`] over a path already spelled as text — a lock row's
/// `source_path` as read, which arrives as a string and is compared as one.
///
/// The leading `./` is the engine's own fork rather than the host's: an engine before
/// the workspace path was lexically normalized spelled a root-`.` member `./CLAUDE.md`
/// where the current pass keys the bare `CLAUDE.md`. Both sides must normalize at read
/// time (decision 0024) or an older lock's every live projection reads ownerless —
/// mass-reaped by the reap sweep, and bound by no guard.
#[must_use]
fn canonical_spelling(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches("./").to_string()
}

/// Relativize a file path against a harness root. If the path is absolute, strip the root
/// prefix and return the relative path. If the path is already relative, return it as-is.
/// Either way the result carries the canonical spelling ([`to_canonical_spelling`]).
/// Returns `None` if an absolute path is not under the root, or if a relative root cannot
/// be resolved against the working directory.
#[must_use]
pub fn relativize_against_root(file_path: &str, root: &Path) -> Option<String> {
    let file_path_normalized = canonical_spelling(file_path);
    let file_path_buf = PathBuf::from(&file_path_normalized);

    // If the path is not absolute, it's already relative — return as-is.
    if !file_path_buf.is_absolute() {
        return Some(file_path_normalized);
    }

    let root_absolute = absolute_root(root)?;

    file_path_buf
        .strip_prefix(&root_absolute)
        .ok()
        .map(to_canonical_spelling)
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
    /// Create a new harness-relative path, canonicalizing what it is handed
    /// ([`canonical_spelling`]). The form the type declares is enforced at this one
    /// door, so a raw `lock.toml` row — an older engine's `./CLAUDE.md`, a
    /// host-separated `.claude\rules\rust.md` — arrives already in the vocabulary
    /// every reader of the column compares on, with no reader re-spelling it.
    #[must_use]
    pub fn new(s: String) -> Self {
        Self(canonical_spelling(&s))
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

impl From<HarnessRelativePath> for String {
    fn from(p: HarnessRelativePath) -> Self {
        p.0
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
    fn the_canonical_spelling_slash_separates_and_erases_a_leading_dot_segment() {
        // A Windows `Path::join` inserts `\` at the join boundary; simulate that shape
        // directly (Unix `Path` never inserts `\`, so a real join can't reproduce it
        // here) and assert the spelling still comes out `/`-separated.
        assert_eq!(
            to_canonical_spelling(&PathBuf::from("harness\\dir\\file.md")),
            "harness/dir/file.md"
        );
        // And the `./` an older engine's lock rows carry is erased, so a row spelled
        // `./CLAUDE.md` meets a reader keying the bare name (decision 0024).
        assert_eq!(canonical_spelling("./CLAUDE.md"), "CLAUDE.md");
        assert_eq!(
            to_canonical_spelling(&PathBuf::from(".\\.claude\\skills\\x\\SKILL.md")),
            ".claude/skills/x/SKILL.md"
        );
    }

    #[test]
    fn the_harness_relative_door_canonicalizes_what_it_is_handed() {
        // The type's declared form is its constructor's job, not each reader's: a raw
        // lock row arrives through this door, so the `./` an engine before decision 0024
        // wrote and the host separator a Windows `Path::join` inserts are both gone
        // before any judge compares the column.
        assert_eq!(
            &*HarnessRelativePath::new("./CLAUDE.md".to_string()),
            "CLAUDE.md"
        );
        assert_eq!(
            &*HarnessRelativePath::new(".\\.claude\\rules\\rust.md".to_string()),
            ".claude/rules/rust.md"
        );
        // And a spelling already canonical passes through untouched.
        assert_eq!(
            &*HarnessRelativePath::new(".claude/skills/x/SKILL.md".to_string()),
            ".claude/skills/x/SKILL.md"
        );
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
