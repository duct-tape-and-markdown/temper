//! Glob compilation cache — the one glob-matching surface the crate shares,
//! serving membership tests, discovery walks, coverage notes, and liveness checks.
//! Foundation vocabulary with no harness dependencies.

/// Compile `glob` into a `globset` matcher — the one glob-matching surface every
/// caller shares, in this module or across the crate (a kind's own
/// [`CustomKind::owns_source`] membership test, `import`'s per-segment discovery
/// walk, `coverage_note`'s `governs` leaf test, `graph`'s `paths-match` liveness
/// test). `literal_separator` is on: `*`/`?` stay within one `/`-separated segment,
/// `**` crosses segments (a leading `**/` matching zero or more, per `globset`'s
/// documented three-position grammar) — the one semantics every call site needs,
/// whether the candidate it tests is a bare filename (no `/` to cross) or a full
/// repo-relative path. `None` for a glob `globset` cannot compile (a malformed
/// character class); the caller decides what an uncompilable pattern means for its
/// own match (`graph`'s liveness test treats it as matching, never a false
/// negative on a pattern it failed to understand).
#[must_use]
pub(crate) fn compile_glob(glob: &str) -> Option<globset::GlobMatcher> {
    GLOB_CACHE.with(|cache| {
        if let Some(hit) = cache.borrow().get(glob) {
            return hit.clone();
        }
        // A globset build is Aho-Corasick/regex construction, not a byte compare: the
        // discovery walk tests one leaf glob against every candidate name at a level, so a
        // per-candidate rebuild of the same handful of loci globs is the whole-input cost
        // this memo hoists. Compilation is a pure function of the string, so a
        // process-lifetime cache keyed on it is always the same matcher a rebuild would
        // yield — the miss counter below advances once per distinct glob, the count a
        // hoist-by-count pin asserts against.
        GLOB_COMPILES.with(|c| c.set(c.get() + 1));
        let compiled = globset::GlobBuilder::new(glob)
            .literal_separator(true)
            .build()
            .ok()
            .map(|compiled| compiled.compile_matcher());
        cache
            .borrow_mut()
            .insert(glob.to_string(), compiled.clone());
        compiled
    })
}

thread_local! {
    /// Per-thread memo of compiled glob matchers, keyed by the glob string. A matcher is a
    /// pure function of its glob, so a cache entry never goes stale within a process; the
    /// walk's per-candidate leaf-glob tests hit it rather than rebuilding.
    static GLOB_CACHE: std::cell::RefCell<std::collections::HashMap<String, Option<globset::GlobMatcher>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
    /// Per-thread count of actual glob *builds* — cache misses, one per distinct glob a
    /// thread ever compiles. The discovery walk tests one leaf glob per candidate name, so
    /// without the memo this would scale with the consumer's file count; with it, the
    /// count-pin ([`glob_compile_count`]) reads it as the number of distinct loci globs, a
    /// small constant independent of tree size — decidable and machine-independent, never a
    /// wall-clock threshold.
    static GLOB_COMPILES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// This thread's cumulative count of distinct glob compilations (cache misses). Read
/// before and after a discovery pass and compare the delta to the loci globs the pass
/// ranges over, pinning that glob compilation is hoisted per distinct glob rather than
/// recomputed per candidate file.
#[must_use]
pub fn glob_compile_count() -> usize {
    GLOB_COMPILES.with(std::cell::Cell::get)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether `glob` matches `candidate` through the shared `crate::glob::compile_glob` surface —
    /// `None` (an uncompilable glob) reported as no match, the polarity every
    /// segment-level caller (`owns_source`, `import`, `coverage_note`) wants.
    fn matches(glob: &str, candidate: &str) -> bool {
        compile_glob(glob).is_some_and(|matcher| matcher.is_match(candidate))
    }

    #[test]
    fn compile_glob_matches_common_path_globs_within_and_across_segments() {
        // `**/` matches any number of leading segments including none, a flat `*`
        // stays within one segment — the semantics every caller through
        // `compile_glob` leans on, whether matching a bare filename or a full
        // repo-relative path.
        assert!(matches("**/*.rs", "foo.rs"));
        assert!(matches("**/*.rs", "src/a/foo.rs"));
        assert!(!matches("**/*.rs", "foo.md"));

        assert!(matches("src/**", "src/graph.rs"));
        assert!(matches("src/**", "src/a/b.rs"));
        assert!(!matches("src/**", "tests/graph.rs"));

        // A single `*` does not cross a `/`.
        assert!(matches("*.md", "README.md"));
        assert!(!matches("*.md", "docs/README.md"));

        // A `?` matches exactly one character, never a `/`.
        assert!(matches("SKILL.md", "SKILL.md"));
        assert!(matches("SKILL.m?", "SKILL.md"));
        assert!(!matches("SKILL.m?", "SKILL.mkd"));
        assert!(!matches("*/SKILL.md", "SKILL.md"));
        assert!(matches("[0-9][0-9]-*.md", "07-kinds.md"));
        assert!(!matches("[0-9][0-9]-*.md", "ab-kinds.md"));
    }

    #[test]
    fn compile_glob_is_none_for_an_uncompilable_pattern() {
        // An unterminated character class is a `globset` compile error — `None`,
        // never a panic.
        assert!(compile_glob("[abc").is_none());
    }
}
