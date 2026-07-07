//! The embedded built-in lock (`specs/architecture/50-distribution.md`, "The stranger
//! gate"; "Decision: the built-in lock is derived from the SDK module, never
//! transcribed"). `src/builtin_lock.toml` is the real `[declaration.*]` family
//! `drift::emit` writes for a memberless `Payload` compiled from a memberless
//! harness over `@dtmd/temper/claude-code`'s built-in kinds and floors — embedded
//! as data (`include_str!`) and parsed once here into the [`Declarations`] IR the
//! gate already reads off a committed lock ([`crate::drift::read_declarations`]).
//!
//! [`crate::builtin`] projects each built-in kind's floor `Contract` straight off
//! [`declarations`]'s clause rows; [`crate::builtin_kind`] still carries its own
//! hand-written kind facts.

use std::path::Path;
use std::sync::LazyLock;

use crate::drift::{Declarations, parse_declarations};

/// The embedded built-in lock's raw TOML bytes — generated data (see the file's own
/// header for how to regenerate it), never hand-edited.
const BUILTIN_LOCK_TOML: &str = include_str!("builtin_lock.toml");

/// Parsed once, on first use — the embedded bytes never change at runtime, so
/// re-parsing per call would only repeat identical work.
static BUILTIN_DECLARATIONS: LazyLock<Declarations> = LazyLock::new(|| {
    parse_declarations(Path::new("src/builtin_lock.toml"), BUILTIN_LOCK_TOML).expect(
        "src/builtin_lock.toml is compiled-in data produced by this crate's own emit — \
         a parse failure here is a build-time bug, never a runtime condition",
    )
});

/// The built-in lock, parsed into the `Declarations` IR — the default program's
/// declaration source (`specs/architecture/50-distribution.md`, "The stranger gate").
#[must_use]
pub fn declarations() -> &'static Declarations {
    &BUILTIN_DECLARATIONS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_lock_parses_into_kind_facts_and_floor_clauses_only() {
        let declarations = declarations();

        // Kind facts: the built-in kinds the memberless emit's `expect` bindings
        // named (`skill`, `rule`, `memory` — no `provider` column, since the SDK
        // module exports none today).
        let mut names: Vec<&str> = declarations
            .kinds
            .iter()
            .map(|row| row.name.as_str())
            .collect();
        names.sort_unstable();
        assert_eq!(names, vec!["memory", "rule", "skill"]);
        assert!(declarations.kinds.iter().all(|row| row.provider.is_none()));

        // Floor clauses: every row names one of the built-in kinds, and carries a
        // declared severity — no requirements, no satisfies, no provenance or
        // emit-fingerprint rows (nothing was emitted; there are no members).
        assert!(!declarations.clauses.is_empty());
        for clause in &declarations.clauses {
            assert!(matches!(
                clause.kind.as_deref(),
                Some("skill" | "rule" | "memory")
            ));
            assert!(matches!(clause.severity.as_str(), "required" | "advisory"));
        }
        assert!(declarations.requirements.is_empty());
        assert!(declarations.satisfies.is_empty());
    }

    #[test]
    fn re_parsing_the_embedded_bytes_is_deterministic() {
        // The embed is static data; parsing it twice must agree byte-for-byte with
        // itself (`Declarations` derives `PartialEq`).
        let parsed = parse_declarations(Path::new("src/builtin_lock.toml"), BUILTIN_LOCK_TOML)
            .expect("the embedded lock parses");
        assert_eq!(&parsed, declarations());
    }
}
