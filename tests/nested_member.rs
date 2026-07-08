//! The member-fence nested-member fold, end to end.
//!
//! A kind declaring an inner-layer `template` extracts each member fence (info string
//! `member.<kind> <key>`, TOML interior with leaf fields and keyed nested members) into
//! a typed [`EmbeddedMember`] at the embedded locus: its own identity (child kind +
//! key), its own prose leaves, and its own nested members one layer deeper — one
//! member shape, whatever locus it lives at. Every leaf is addressed **structurally**
//! (member + child kind + key + child path, stable under content edits), member
//! addressing where leaf addressing used to be; an unfenced block stays plain prose —
//! adoption is opt-in per block, never an error.

use std::collections::BTreeMap;
use std::path::PathBuf;

use temper::drift::KindFactRow;
use temper::kind::{CustomKind, Extraction, Governs, Primitive, Template, Unit};

/// A custom `decision` kind composing the `fenced` primitive and declaring one
/// inner-layer template — the shape (leaf fields, nested-member collections) is the
/// kind's, the predicates over it are the bound package's, out of this definition.
fn decision_kind() -> CustomKind {
    CustomKind {
        templates: vec![Template {
            kind: "decision".to_string(),
            leaves: vec!["chosen".to_string(), "because".to_string()],
            collections: vec!["rejected".to_string()],
        }],
        ..CustomKind::new(
            "decision",
            Governs {
                root: "docs/decisions".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(vec![Primitive::Fenced]),
        )
    }
}

/// A decision-document unit carrying `body` — a frontmatter-less surface document, the
/// floor carriage a member fence lives in.
fn decision_unit(body: &str) -> Unit {
    Unit {
        id: "05-surface-authority".to_string(),
        frontmatter: BTreeMap::new(),
        body: body.to_string(),
        source_path: PathBuf::from("docs/decisions/05-surface-authority.md"),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
        published_requirements: Vec::new(),
    }
}

/// A body with one member fence (leaf fields + a keyed nested-member collection) and one
/// plain `sh` block that opts into no nested member.
fn decision_body() -> &'static str {
    r#"# Decision: the surface is the source of truth

Leading prose that is only prose.

```member.decision surface-authority
chosen = """the composition surface is canonical"""
because = "law 7 needs an authored surface"

[rejected.baked-projection]
because = "a stamping projector breaks law 5"
```

An ordinary example, opting into no nested member:

```sh
cargo build
```

Trailing prose.
"#
}

#[test]
fn a_member_fence_extracts_a_nested_member_with_its_own_leaves_and_children() {
    let features = decision_kind().extract(&decision_unit(decision_body()));

    // Exactly one nested member — the `sh` block opted into none (opt-in per block),
    // and both raw blocks still ride `fenced_blocks` beside the typed layer.
    assert_eq!(features.nested_members.len(), 1);
    assert_eq!(features.fenced_blocks.len(), 2);
    let member = &features.nested_members[0];
    assert_eq!(member.kind, "decision");
    assert_eq!(member.key, "surface-authority");

    // Leaves are top-level authored strings, keyed by field name — the member's own
    // prose.
    assert_eq!(
        member.leaves.get("chosen").map(String::as_str),
        Some("the composition surface is canonical")
    );
    assert_eq!(
        member.leaves.get("because").map(String::as_str),
        Some("law 7 needs an authored surface")
    );

    // The nested-member collection is keyed at every level (`rejected` →
    // `baked-projection` → `because`), never positional — the entry is itself a
    // full nested member, one layer deeper.
    let entry = &member.members["rejected"]["baked-projection"];
    assert_eq!(entry.key, "baked-projection");
    assert_eq!(
        entry.leaves.get("because").map(String::as_str),
        Some("a stamping projector breaks law 5")
    );
}

#[test]
fn leaf_addresses_are_structural_member_kind_key_child_path() {
    let features = decision_kind().extract(&decision_unit(decision_body()));

    // Every leaf carries a full structural address — the member, the nested member's
    // identity, and the child path — the leaf-grain surface the read family consumes.
    // "member addressing" where "leaf addressing" used to be.
    let leaves = features.embedded_leaves();
    let paths: Vec<&str> = leaves
        .iter()
        .map(|(address, _)| address.child_path.as_str())
        .collect();
    assert!(paths.contains(&"chosen"));
    assert!(paths.contains(&"because"));
    // The nested entry's path is keyed by structure, not a positional `rejected.0.because`.
    assert!(paths.contains(&"rejected.baked-projection.because"));
    assert!(!paths.iter().any(|path| path.contains(".0.")));

    let (address, leaf) = leaves
        .iter()
        .find(|(address, _)| address.child_path == "rejected.baked-projection.because")
        .expect("the keyed nested-member leaf is addressed");
    assert_eq!(address.member, "05-surface-authority");
    assert_eq!(address.kind, "decision");
    assert_eq!(address.key, "surface-authority");
    assert_eq!(*leaf, "a stamping projector breaks law 5");
}

#[test]
fn an_unfenced_block_stays_plain_prose_no_nested_member_no_error() {
    // A body whose only fence is a plain code block — no member fence at all. Extraction
    // yields the raw block and *no* nested member, never an error: adoption is opt-in
    // per block, and no check quantifies over completeness.
    let body = "# Notes\n\nProse.\n\n```sh\ncargo test\n```\n\nMore prose.\n";
    let features = decision_kind().extract(&decision_unit(body));
    assert!(features.nested_members.is_empty());
    assert_eq!(features.fenced_blocks.len(), 1);
}

/// The `decision` kind's declaration row a lock would carry, its `templates`
/// column recording the same child kind `decision_kind`'s live SDK declaration
/// composes (`LOCK-NESTING-TEMPLATES`).
fn decision_kind_fact_row() -> KindFactRow {
    KindFactRow {
        name: "decision".to_string(),
        provider: None,
        governs_root: "docs/decisions".to_string(),
        governs_glob: "*.md".to_string(),
        format: None,
        unit_shape: None,
        registration: None,
        templates: vec!["decision".to_string()],
    }
}

#[test]
fn a_lock_reconstructed_kind_folds_the_same_embedded_members_as_its_live_declaration() {
    // `from_kind_fact_row` lifts the row's `templates` column into the same
    // `Template.kind` set `fold_members` keys on — a lock-reconstructed kind must
    // fold the identical nested members its live SDK declaration does, closing the
    // residual gap the row used to drop.
    let live = decision_kind().extract(&decision_unit(decision_body()));
    let reconstructed = CustomKind::from_kind_fact_row(&decision_kind_fact_row())
        .extract(&decision_unit(decision_body()));

    assert_eq!(reconstructed.nested_members.len(), 1);
    assert_eq!(reconstructed.nested_members, live.nested_members);
}

#[test]
fn a_fence_naming_an_undeclared_child_kind_stays_raw() {
    // A `member.`-prefixed fence whose child kind the host kind does not declare a
    // template for is matched by no shape — it stays a raw block, not a typed nested
    // member. The declared template set is the gate on which fences fold.
    let body = "# X\n\n```member.law fearless-refactoring\nstatement = \"law 6\"\n```\n";
    let features = decision_kind().extract(&decision_unit(body));
    assert!(features.nested_members.is_empty());
    assert_eq!(features.fenced_blocks.len(), 1);
}

#[test]
fn a_genre_prefixed_fence_is_now_a_plain_block_not_a_nested_member() {
    // The read fold speaks the kernel noun: a fence still carrying the retired
    // `genre.` prefix parses as no member at all (`parse_embedded_info` matches only
    // `member.`), so it rides `fenced_blocks` raw, never `nested_members`.
    let body = "# X\n\n```genre.decision surface-authority\nchosen = \"x\"\n```\n";
    let features = decision_kind().extract(&decision_unit(body));
    assert!(features.nested_members.is_empty());
    assert_eq!(features.fenced_blocks.len(), 1);
    assert_eq!(
        features.fenced_blocks[0].info,
        "genre.decision surface-authority"
    );
}

#[test]
fn a_blocks_body_the_sdk_writes_folds_back_to_an_identical_embedded_member() {
    // The write↔read round trip: the exact bytes `sdk/src/emit.ts`'s `resolveBody`
    // renders for a `blocks()` value (`member.<kind> <key>` fence, leaves as quoted
    // TOML strings, a keyed collection as its own `[collection.entry]` table) fold
    // back through this same kind's extractor to the identical `EmbeddedMember` the
    // authored-fence fixture above (`decision_body`) yields.
    let written_body = "# Decision: the surface is the source of truth\n\n\
```member.decision surface-authority\n\
chosen = \"the composition surface is canonical\"\n\
because = \"law 7 needs an authored surface\"\n\
\n\
[rejected.baked-projection]\n\
because = \"a stamping projector breaks law 5\"\n\
```\n";
    let written = decision_kind().extract(&decision_unit(written_body));
    let authored = decision_kind().extract(&decision_unit(decision_body()));

    assert_eq!(written.nested_members.len(), 1);
    assert_eq!(written.nested_members, authored.nested_members);
}
