//! The nested-member display rule's byte contract.
//!
//! `render_member` compiles an `EmbeddedMember` (leaves + nested members) to projection
//! markdown — an anchored heading, capitalized leaf/collection labels, ordered leaves
//! then nested members, per-leaf anchors — as connective tissue only: every
//! meaning-carrying word is an authored leaf, rendered verbatim. The snapshots
//! below **are** the pinned contract the future custom-kind emit face must reproduce
//! byte for byte; the double-render and heading-only cases pin the determinism and the
//! empty-member shape the island guarantees.

use std::collections::BTreeMap;

use temper::display::render_member;
use temper::extract::{EmbeddedMember, EmbeddedMemberCollectionEntry};

/// A collection-entry nested member carrying one `because` leaf — the shape a
/// `rejected` collection's entries take.
fn rejected_entry(key: &str, because: &str) -> EmbeddedMember {
    EmbeddedMember {
        kind: "rejected".to_string(),
        key: key.to_string(),
        leaves: BTreeMap::from([("because".to_string(), because.to_string())]),
        members: Vec::new(),
    }
}

/// One `rejected`-collection entry, pairing its key with its nested member.
fn rejected_collection_entry(key: &str, because: &str) -> EmbeddedMemberCollectionEntry {
    EmbeddedMemberCollectionEntry {
        collection: "rejected".to_string(),
        key: key.to_string(),
        member: rejected_entry(key, because),
    }
}

/// A `decision`-kind nested member with two ordered leaves (`chosen`, then a second
/// leaf) and a `rejected` collection's authored-order entries — the shape the
/// floor's member fence extracts, the one the emit face renders back into the
/// projection.
fn decision_member() -> EmbeddedMember {
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
    let rejected = vec![
        rejected_collection_entry("baked-projection", "a stamping projector breaks law 5"),
        rejected_collection_entry(
            "read-only-lens",
            "you cannot compose a harness you only mirror",
        ),
    ];
    EmbeddedMember {
        kind: "decision".to_string(),
        key: "surface-authority".to_string(),
        leaves,
        members: rejected,
    }
}

#[test]
fn a_decision_member_renders_to_its_pinned_projection_markdown() {
    // The full contract: the anchored `### Decision:` heading, the `chosen`/`because`
    // leaves (key order), then the keyed `rejected` collection's two entries — each an
    // anchored, labelled paragraph carrying its authored leaf verbatim.
    insta::assert_snapshot!("decision_value", render_member(&decision_member()));
}

#[test]
fn a_leaves_only_member_renders_its_leaves_and_no_collection_block() {
    // A member with leaves but no nested members — the heading and the ordered leaf
    // paragraphs, with no `**Rejected:**` grouping to follow.
    let member = EmbeddedMember {
        kind: "decision".to_string(),
        key: "leaves-only".to_string(),
        leaves: BTreeMap::from([(
            "chosen".to_string(),
            "one authored leaf, no siblings".to_string(),
        )]),
        members: Vec::new(),
    };
    insta::assert_snapshot!("leaves_only_value", render_member(&member));
}

#[test]
fn a_collections_only_member_renders_its_nested_member_and_no_leaf_block() {
    // A member with a nested collection but no top-level leaves — the heading goes
    // straight to the `**Rejected:**` group; leaves-before-collections still holds
    // trivially (there are none).
    let rejected = vec![rejected_collection_entry(
        "baked-stance",
        "the tool would determine invasiveness on a surface it was invited onto",
    )];
    let member = EmbeddedMember {
        kind: "decision".to_string(),
        key: "collections-only".to_string(),
        leaves: BTreeMap::new(),
        members: rejected,
    };
    insta::assert_snapshot!("collections_only_value", render_member(&member));
}

#[test]
fn an_empty_member_renders_its_heading_with_no_body() {
    // No leaves, no nested members — the anchored heading alone, the heading-only shape
    // the island guarantees for a nested member carrying nothing.
    let member = EmbeddedMember {
        kind: "decision".to_string(),
        key: "empty".to_string(),
        leaves: BTreeMap::new(),
        members: Vec::new(),
    };
    insta::assert_snapshot!("empty_value", render_member(&member));
}

#[test]
fn a_double_render_is_byte_identical() {
    // Determinism is the contract: the same member
    // renders the same bytes, the property the emit face's double-emit comparison
    // stands on.
    let member = decision_member();
    assert_eq!(render_member(&member), render_member(&member));
}

#[test]
fn every_authored_leaf_is_rendered_verbatim_as_connective_tissue() {
    // Traceability: every meaning-carrying word is an authored leaf. Each leaf value
    // appears verbatim in the projection, and no leaf's words are reworded or dropped —
    // the display rule adds only heading, labels, and anchors around the author's text.
    let member = decision_member();
    let rendered = render_member(&member);
    for leaf in member.leaves.values() {
        assert!(rendered.contains(leaf.as_str()), "leaf missing: {leaf}");
    }
    for entry in &member.members {
        for leaf in entry.member.leaves.values() {
            assert!(rendered.contains(leaf.as_str()), "leaf missing: {leaf}");
        }
    }
}
