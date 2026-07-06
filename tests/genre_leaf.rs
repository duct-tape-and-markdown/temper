//! The genre-fence leaf schema, end to end (`specs/architecture/20-surface.md`, "Genre
//! values — prose that declares its own anatomy"; the two leaf Decisions).
//!
//! A kind declaring `genres` extracts each genre fence (info string
//! `genre.<genre> <key>`, TOML interior with leaf fields and keyed sibling
//! collections) into a typed genre value: its leaves addressed **structurally**
//! (member + genre key + field path, stable under content edits) and its siblings
//! **keyed** (`rejected.baked-projection.because`), never positional; an unfenced
//! block stays plain prose — genre adoption is opt-in per block, never an error.

use std::collections::BTreeMap;
use std::path::PathBuf;

use temper::kind::{CustomKind, Extraction, Genre, Governs, Primitive, Unit};

/// A custom `decision` kind composing the `fenced` primitive and declaring one genre —
/// the shape (leaf fields, keyed collections) is the kind's; the predicates over it are
/// the bound package's, out of this definition.
fn decision_kind() -> CustomKind {
    CustomKind {
        genres: vec![Genre {
            name: "decision".to_string(),
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
/// floor carriage a genre fence lives in.
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

/// A body with one genre fence (leaf fields + a keyed sibling collection) and one plain
/// `sh` block that opts into no genre.
fn decision_body() -> &'static str {
    r#"# Decision: the surface is the source of truth

Leading prose that is only prose.

```genre.decision surface-authority
chosen = """the composition surface is canonical"""
because = "law 7 needs an authored surface"

[rejected.baked-projection]
because = "a stamping projector breaks law 5"
```

An ordinary example, opting into no genre:

```sh
cargo build
```

Trailing prose.
"#
}

#[test]
fn a_genre_fence_extracts_structural_leaves_and_keyed_siblings() {
    let features = decision_kind().extract(&decision_unit(decision_body()));

    // Exactly one genre value — the `sh` block opted into none (opt-in per block), and
    // both raw blocks still ride `fenced_blocks` beside the typed layer.
    assert_eq!(features.genres.len(), 1);
    assert_eq!(features.fenced_blocks.len(), 2);
    let value = &features.genres[0];
    assert_eq!(value.genre, "decision");
    assert_eq!(value.key, "surface-authority");

    // Leaves are top-level authored strings, keyed by field name.
    assert_eq!(
        value.leaves.get("chosen").map(String::as_str),
        Some("the composition surface is canonical")
    );
    assert_eq!(
        value.leaves.get("because").map(String::as_str),
        Some("law 7 needs an authored surface")
    );

    // The sibling collection is keyed at every level (`rejected` → `baked-projection` →
    // `because`), never positional.
    assert_eq!(
        value.collections["rejected"]["baked-projection"]
            .get("because")
            .map(String::as_str),
        Some("a stamping projector breaks law 5")
    );
}

#[test]
fn leaf_addresses_are_structural_member_genre_key_field_path() {
    let features = decision_kind().extract(&decision_unit(decision_body()));

    // Every leaf carries a full structural address — the member, the genre value's
    // identity, and the field path — the leaf-grain surface the read family consumes.
    let leaves = features.genre_leaves();
    let paths: Vec<&str> = leaves
        .iter()
        .map(|(address, _)| address.field_path.as_str())
        .collect();
    assert!(paths.contains(&"chosen"));
    assert!(paths.contains(&"because"));
    // The sibling's path is keyed by structure, not a positional `rejected.0.because`.
    assert!(paths.contains(&"rejected.baked-projection.because"));
    assert!(!paths.iter().any(|path| path.contains(".0.")));

    let (address, leaf) = leaves
        .iter()
        .find(|(address, _)| address.field_path == "rejected.baked-projection.because")
        .expect("the keyed sibling leaf is addressed");
    assert_eq!(address.member, "05-surface-authority");
    assert_eq!(address.genre, "decision");
    assert_eq!(address.key, "surface-authority");
    assert_eq!(*leaf, "a stamping projector breaks law 5");
}

#[test]
fn an_unfenced_block_stays_plain_prose_no_genre_no_error() {
    // A body whose only fence is a plain code block — no genre fence at all. Extraction
    // yields the raw block and *no* genre value, never an error: adoption is opt-in per
    // block, and no check quantifies over genre completeness.
    let body = "# Notes\n\nProse.\n\n```sh\ncargo test\n```\n\nMore prose.\n";
    let features = decision_kind().extract(&decision_unit(body));
    assert!(features.genres.is_empty());
    assert_eq!(features.fenced_blocks.len(), 1);
}

#[test]
fn a_fence_naming_an_undeclared_genre_stays_raw() {
    // A `genre.`-prefixed fence whose genre the kind does not declare is matched by no
    // shape — it stays a raw block, not a typed value. The declared genre set is the
    // gate on which fences fold.
    let body = "# X\n\n```genre.law fearless-refactoring\nstatement = \"law 6\"\n```\n";
    let features = decision_kind().extract(&decision_unit(body));
    assert!(features.genres.is_empty());
    assert_eq!(features.fenced_blocks.len(), 1);
}
