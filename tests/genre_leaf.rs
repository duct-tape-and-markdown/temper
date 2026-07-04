//! The genre-fence leaf schema, end to end (`specs/architecture/20-surface.md`, "Genre
//! values — prose that declares its own anatomy"; the two leaf Decisions).
//!
//! A kind declaring `genres` extracts each genre fence (info string
//! `genre.<genre> <key>`, TOML interior with leaf fields and keyed sibling
//! collections) into a typed genre value: its leaves addressed **structurally**
//! (member + genre key + field path, stable under content edits) and its siblings
//! **keyed** (`rejected.baked-projection.because`), never positional. The value
//! serializes whole into the `[[member]]` manifest table and round-trips
//! byte-identically through `toml_edit`; an unfenced block stays plain prose — genre
//! adoption is opt-in per block, never an error.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use temper::compose::{AuthorLayer, ManifestMember, write_manifest_members};
use temper::kind::{CustomKind, Unit};
use toml_edit::DocumentMut;

/// A custom `decision` kind whose `KIND.md` header composes the `fenced` primitive and
/// declares one genre — the shape (leaf fields, keyed collections) is the kind's; the
/// predicates over it are the bound package's, out of this definition.
fn decision_kind() -> CustomKind {
    let header = r#"governs = { root = "docs/decisions", glob = "*.md" }

[[extraction]]
primitive = "fenced"

[[genres]]
name = "decision"
leaves = ["chosen", "because"]
collections = ["rejected"]
"#;
    let doc: DocumentMut = header.parse().expect("the KIND.md header is valid TOML");
    CustomKind::from_header(
        doc.as_table(),
        "decision",
        Path::new(".temper/kinds/decision/KIND.md"),
    )
    .expect("the genre-declaring KIND.md header parses")
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

#[test]
fn a_genre_value_round_trips_through_the_manifest_byte_identically() {
    let features = decision_kind().extract(&decision_unit(decision_body()));
    let member = ManifestMember {
        kind: "decision".to_string(),
        features,
    };

    // Serialize the member whole into the manifest, then reparse: the typed genre value
    // returns exactly — leaf fields as strings, sibling collections as keyed sub-tables.
    let mut doc = DocumentMut::new();
    write_manifest_members(&mut doc, std::slice::from_ref(&member));
    let emitted = doc.to_string();
    let layer = AuthorLayer::parse(&emitted, Path::new("temper.toml")).unwrap();
    assert_eq!(layer.members(), std::slice::from_ref(&member));

    // And re-emitting the reparsed member is byte-for-byte identical — the round-trip is
    // idempotent, the property drift and the read family stand on.
    let reparsed = layer.members()[0].clone();
    let mut doc2 = DocumentMut::new();
    write_manifest_members(&mut doc2, std::slice::from_ref(&reparsed));
    assert_eq!(emitted, doc2.to_string());
}
