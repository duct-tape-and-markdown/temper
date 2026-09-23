//! The embedded built-in lock. `src/builtin_lock.toml` is the real `[declaration.*]` family
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
/// declaration source.
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
        // named — the one enumeration `builtin_kind::definitions()` already spells,
        // asserted here rather than restated, so a kind added to the Rust population
        // without a lock regeneration fails this test instead of shipping green.
        // (`tests/builtin_lock_frozen.rs` pins the other leg, SDK↔embedded lock.)
        let mut names: Vec<&str> = declarations
            .kinds
            .iter()
            .map(|row| row.name.as_str())
            .collect();
        names.sort_unstable();
        let population = crate::builtin_kind::definitions();
        // Both sides are derived now, so the comparison needs its own vacuity pin:
        // two empty sets would agree and prove nothing.
        assert!(!names.is_empty());
        // `definitions()` is a `BTreeMap` keyed by bare name — already in the order
        // `names` was just sorted into.
        assert_eq!(
            names,
            population.keys().map(String::as_str).collect::<Vec<_>>()
        );
        // No `provider` column, since the SDK module exports none today.
        assert!(declarations.kinds.iter().all(|row| row.provider.is_none()));

        // `supporting-doc` alone rides the nested-file locus: its row carries no governs
        // pair, and `skill`'s `templates` column is where its path fact actually lives.
        let doc = declarations
            .kinds
            .iter()
            .find(|row| row.name == "supporting-doc")
            .expect("supporting-doc ships in the embedded lock");
        assert_eq!(doc.governs_root, None);
        assert_eq!(doc.governs_glob, None);
        let skill = declarations
            .kinds
            .iter()
            .find(|row| row.name == "skill")
            .expect("skill ships in the embedded lock");
        assert_eq!(
            skill.templates,
            vec![crate::drift::TemplateRow {
                kind: "supporting-doc".to_string(),
                path: Some("*.md".to_string()),
            }]
        );

        // Floor clauses: every row names one of the built-in kinds, and carries a
        // declared severity — no requirements, no satisfies, no provenance or
        // emit-fingerprint rows (nothing was emitted; there are no members).
        assert!(!declarations.clauses.is_empty());
        for clause in &declarations.clauses {
            assert!(matches!(
                clause.kind.as_deref(),
                Some(
                    "agent"
                        | "command"
                        | "dial"
                        | "hook"
                        | "marketplace"
                        | "mcp-server"
                        | "plugin-manifest"
                        | "settings-local"
                        | "skill"
                        | "supporting-doc"
                        | "rule"
                        | "memory"
                )
            ));
            assert!(matches!(clause.severity.as_str(), "required" | "advisory"));
        }
        assert!(declarations.requirements.is_empty());
        assert!(declarations.satisfies.is_empty());
    }

    #[test]
    fn every_lock_kind_row_lifts_to_the_facts_the_rust_population_spells() {
        let declarations = declarations();
        let population = crate::builtin_kind::definitions();

        // The sibling of the names assertion above, and it carries that assertion's
        // vacuity pin for the same reason: an empty row set would agree with anything.
        assert!(!declarations.kinds.is_empty());

        for row in &declarations.kinds {
            let lifted = crate::kind::CustomKind::from_kind_fact_row(row).unwrap_or_else(|err| {
                panic!("the embedded lock's `{}` row lifts: {err}", row.name)
            });
            let spelled = population
                .get(&row.name)
                .unwrap_or_else(|| panic!("`{}` ships in the Rust population", row.name));

            // Group by group, never a whole-struct compare: three fact groups the Rust
            // population carries have no `KindFactRow` column to ride, so the lift fills
            // them with its own fixed answer — `extraction` with the generic
            // markdown-structure primitive set, `relationships` and `bare_root_file`
            // empty. Comparing those would assert the lift's defaults, not agreement.
            let name = &row.name;
            assert_eq!(lifted.governs, spelled.governs, "{name}: governs");
            assert_eq!(lifted.commitment, spelled.commitment, "{name}: commitment");
            assert_eq!(lifted.format, spelled.format, "{name}: format");
            assert_eq!(lifted.unit_shape, spelled.unit_shape, "{name}: unit_shape");
            assert_eq!(
                lifted.registration, spelled.registration,
                "{name}: registration"
            );
            assert_eq!(lifted.templates, spelled.templates, "{name}: templates");
            assert_eq!(lifted.content, spelled.content, "{name}: content");
            assert_eq!(
                lifted.collection_address, spelled.collection_address,
                "{name}: collection_address"
            );
        }
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
