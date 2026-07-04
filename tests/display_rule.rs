//! The per-genre display rule's byte contract (`specs/architecture/20-surface.md`,
//! "The display rule owns connective tissue").
//!
//! `render_genre` compiles a `GenreValue` (leaves + keyed collections) to projection
//! markdown — an anchored heading, capitalized leaf/collection labels, ordered leaves
//! then keyed collections, per-leaf anchors — as connective tissue only: every
//! meaning-carrying word is an authored leaf, rendered verbatim (law 5). The snapshots
//! below **are** the pinned contract the future custom-kind emit face must reproduce
//! byte for byte; the double-render and heading-only cases pin the determinism and the
//! empty-value shape the island guarantees.

use std::collections::BTreeMap;

use temper::display::render_genre;
use temper::extract::{GenreCollections, GenreValue};

/// A `decision`-genre value with two ordered leaves (`chosen`, then a second leaf) and
/// a keyed `rejected` collection — the shape the floor's genre fence extracts, the one
/// the emit face renders back into the projection.
fn decision_value() -> GenreValue {
    let leaves = BTreeMap::from([
        (
            "chosen".to_string(),
            "the composition surface is canonical".to_string(),
        ),
        (
            "because".to_string(),
            "law 7 needs an authored surface".to_string(),
        ),
    ]);
    let mut rejected: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    rejected.insert(
        "baked-projection".to_string(),
        BTreeMap::from([(
            "because".to_string(),
            "a stamping projector breaks law 5".to_string(),
        )]),
    );
    rejected.insert(
        "read-only-lens".to_string(),
        BTreeMap::from([(
            "because".to_string(),
            "you cannot compose a harness you only mirror".to_string(),
        )]),
    );
    let collections: GenreCollections = BTreeMap::from([("rejected".to_string(), rejected)]);
    GenreValue {
        genre: "decision".to_string(),
        key: "surface-authority".to_string(),
        leaves,
        collections,
    }
}

#[test]
fn a_decision_value_renders_to_its_pinned_projection_markdown() {
    // The full contract: the anchored `### Decision:` heading, the `chosen`/`because`
    // leaves (key order), then the keyed `rejected` collection's two entries — each an
    // anchored, labelled paragraph carrying its authored leaf verbatim.
    insta::assert_snapshot!("decision_value", render_genre(&decision_value()));
}

#[test]
fn a_leaves_only_value_renders_its_leaves_and_no_collection_block() {
    // A value with leaves but no collections — the heading and the ordered leaf
    // paragraphs, with no `**Rejected:**` grouping to follow.
    let value = GenreValue {
        genre: "decision".to_string(),
        key: "leaves-only".to_string(),
        leaves: BTreeMap::from([(
            "chosen".to_string(),
            "one authored leaf, no siblings".to_string(),
        )]),
        collections: GenreCollections::new(),
    };
    insta::assert_snapshot!("leaves_only_value", render_genre(&value));
}

#[test]
fn a_collections_only_value_renders_its_keyed_collection_and_no_leaf_block() {
    // A value with a keyed collection but no top-level leaves — the heading goes
    // straight to the `**Rejected:**` group; leaves-before-collections still holds
    // trivially (there are none).
    let mut rejected: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    rejected.insert(
        "baked-stance".to_string(),
        BTreeMap::from([(
            "because".to_string(),
            "the tool would determine invasiveness on a surface it was invited onto".to_string(),
        )]),
    );
    let value = GenreValue {
        genre: "decision".to_string(),
        key: "collections-only".to_string(),
        leaves: BTreeMap::new(),
        collections: GenreCollections::from([("rejected".to_string(), rejected)]),
    };
    insta::assert_snapshot!("collections_only_value", render_genre(&value));
}

#[test]
fn an_empty_value_renders_its_heading_with_no_body() {
    // No leaves, no collections — the anchored heading alone, the heading-only shape the
    // island guarantees for a genre value carrying nothing.
    let value = GenreValue {
        genre: "decision".to_string(),
        key: "empty".to_string(),
        leaves: BTreeMap::new(),
        collections: GenreCollections::new(),
    };
    insta::assert_snapshot!("empty_value", render_genre(&value));
}

#[test]
fn a_double_render_is_byte_identical() {
    // Determinism is the contract (law 5): the same value renders the same bytes, the
    // property the emit face's double-emit comparison stands on.
    let value = decision_value();
    assert_eq!(render_genre(&value), render_genre(&value));
}

#[test]
fn every_authored_leaf_is_rendered_verbatim_as_connective_tissue() {
    // Traceability: every meaning-carrying word is an authored leaf. Each leaf value
    // appears verbatim in the projection, and no leaf's words are reworded or dropped —
    // the display rule adds only heading, labels, and anchors around the author's text.
    let value = decision_value();
    let rendered = render_genre(&value);
    for leaf in value.leaves.values() {
        assert!(rendered.contains(leaf.as_str()), "leaf missing: {leaf}");
    }
    for entries in value.collections.values() {
        for leaves in entries.values() {
            for leaf in leaves.values() {
                assert!(rendered.contains(leaf.as_str()), "leaf missing: {leaf}");
            }
        }
    }
}
