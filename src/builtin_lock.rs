//! The embedded built-in lock. `src/builtin_lock.toml` is the real `[declaration.*]` family
//! `drift::emit` writes for a memberless `Payload` compiled from a memberless
//! harness over `@dtmd/temper/claude-code`'s built-in kinds and floors — embedded
//! as data (`include_str!`) and parsed once here into the [`Declarations`] IR
//! [`crate::gate`] lifts off a committed lock with `drift::declarations_from_doc` — the
//! same IR by the same lift, off embedded bytes instead of a disk read.
//!
//! [`crate::builtin`] projects each built-in kind's floor `Contract` straight off
//! [`declarations`]'s clause rows; [`crate::builtin_kind`] lifts each built-in kind off its
//! kind rows.

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

        // Floor clauses: every row names one of the built-in kinds *whose default contract
        // carries clauses* — or names no kind at all, which is the **root member's** own row
        // and nothing else. The assertion's job is to catch a row nobody declared, so the
        // kind-less case is pinned as the root's (exactly the four the shipped default
        // binds, `root.reachable`, `root.fresh`, `root.locus-declared` and
        // `root.engine-matches`) rather than widened to "any `None`": a stray unowned row
        // would then slip through the very check that exists to catch it. Every row carries
        // a declared severity, and there are no requirements, satisfies, provenance or
        // emit-fingerprint rows (nothing was emitted; there are no members).
        //
        // The kinds shipping an empty default contract — `installed-plugin`,
        // `known-marketplace`, `mcp`, `output-style` — are deliberately *absent* from the list below, so
        // their emptiness is checked here rather than assumed: a clause appearing under one
        // of those names panics as unowned, which is the signal that the contract grew and
        // the lock's own header arithmetic needs re-reading.
        assert!(!declarations.clauses.is_empty());
        let mut root_labels: Vec<&str> = Vec::new();
        for clause in &declarations.clauses {
            match clause.kind.as_deref() {
                Some(
                    "agent" | "command" | "dial" | "handler" | "hook" | "marketplace"
                    | "mcp-server" | "plugin-manifest" | "settings" | "settings-local" | "skill"
                    | "supporting-doc" | "rule" | "memory" | "tool",
                ) => {}
                None => root_labels.push(
                    clause
                        .label
                        .as_deref()
                        .expect("emit stamps every lock clause row with its label"),
                ),
                other => panic!("the embedded lock carries a clause row for `{other:?}`"),
            }
            assert!(matches!(clause.severity.as_str(), "required" | "advisory"));
        }
        assert_eq!(
            root_labels,
            vec![
                "root.reachable",
                "root.fresh",
                "root.locus-declared",
                "root.engine-matches"
            ],
            "the shipped root default is the \
             `reachable`/`fresh`/`locus-declared`/`engine-matches` family, in declaration \
             order; a fifth kind-less row is a row nobody declared"
        );
        assert!(declarations.requirements.is_empty());
        assert!(declarations.satisfies.is_empty());
        assert!(declarations.mentions.is_empty());
    }

    #[test]
    fn every_built_in_kind_is_its_lock_rows_lift_plus_only_the_row_less_facts() {
        let declarations = declarations();
        let population = crate::builtin_kind::definitions();
        let edges = crate::drift::edges_from_declarations(declarations).unwrap();

        // Vacuity pin: an empty row set would agree with anything.
        assert!(!declarations.kinds.is_empty());
        assert!(!edges.is_empty());

        for row in &declarations.kinds {
            let lifted = crate::kind::CustomKind::from_kind_fact_row(row).unwrap();
            let built_in = &population[&row.name];

            // The three row-less groups are the only ones the population may change: the
            // relationships are exactly the lock's own edge facts from this kind, and the
            // extraction and bare-root file are `builtin_kind`'s named exceptions. Erasing
            // them leaves the bare lift, so no row-carried fact is spelled a second time.
            let name = &row.name;
            let own_edges: Vec<_> = edges.iter().filter(|e| &e.from == name).cloned().collect();
            assert_eq!(built_in.relationships, own_edges, "{name}: relationships");
            let erased = crate::kind::CustomKind {
                extraction: lifted.extraction.clone(),
                relationships: Vec::new(),
                bare_root_file: None,
                ..built_in.clone()
            };
            assert_eq!(
                erased, lifted,
                "{name}: every row-carried fact is the lift's"
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
