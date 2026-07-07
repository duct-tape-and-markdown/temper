//! The nested-member **display rule** — an [`EmbeddedMember`]'s projection markdown.
//!
//! [`render_member`] compiles an [`EmbeddedMember`] (a genre fence's extracted leaves
//! and nested members) into the markdown a projection carries — a heading, the leaf
//! labels, deterministic ordering, and per-leaf anchors — as **connective tissue
//! only** (`specs/architecture/20-surface.md`, "Emit — total, byte-reproducible,
//! refusing"): every meaning-carrying word is an authored leaf, rendered verbatim
//! (law 5); the heading, labels, and anchors are the member's declared structure in
//! rendered form — the markdown analogue of the lock's TOML syntax, never
//! synthesized prose. One rule serves every child kind: the labels and heading derive
//! from the member's own declared kind and field keys, so it is per-kind by data, not
//! by a hard-coded genre.
//!
//! A **standalone formatter island**: the future custom-kind emit face calls it and
//! must reproduce these exact bytes; it does not wire into `src/drift.rs`'s
//! skill/rule projection (that projection carries no nested member today). Byte-deterministic
//! — a double-render of the same member is byte-identical (law 5), the property
//! `tests/display_rule.rs` pins as the emit face's contract.

use std::fmt::Write;

use crate::extract::EmbeddedMember;

/// Render an [`EmbeddedMember`] to its projection markdown — the anchored heading, then
/// its **leaves before its nested members** (`specs/architecture/20-surface.md`, the
/// display rule; the ordering the read family's [`EmbeddedMember::addressed_leaves`]
/// also walks), each in `BTreeMap` key order so the output is byte-deterministic. Every
/// leaf value is rendered verbatim (law 5); the heading, the capitalized labels, and the
/// anchors are the member's declared structure, not prose. An empty member renders its
/// heading with no body.
///
/// A nested member's own further-nested members (should the fold ever populate one) are
/// not recursed into here — today's fold produces one collection layer, so this island
/// renders that layer only.
///
/// Anchors are addressed by the leaf's `<kind>.<key>.<child-path>` — the `MemberAddress`
/// spelling of `src/read.rs`, minus the outer `member` segment the emit face namespaces
/// projections by. Cross-member anchor uniqueness is therefore the emit face's to own;
/// this island renders one member's self-consistent block (the accepted island risk).
#[must_use]
pub fn render_member(member: &EmbeddedMember) -> String {
    // Each block is a self-contained markdown paragraph; joined by a blank line so the
    // rendering is one deterministic assembly with no trailing-whitespace ambiguity.
    let mut blocks: Vec<String> = Vec::new();

    // The member's heading, anchored by its `<kind>.<key>` identity — the deep-link
    // target a citation resolves to. `###` matches the corpus's decision-heading level.
    blocks.push(format!(
        "<a id=\"{anchor}\"></a>\n### {kind_label}: {key}",
        anchor = member_anchor(member),
        kind_label = capitalize(&member.kind),
        key = member.key,
    ));

    // Top-level leaves first, in key order — each an anchored, labelled paragraph whose
    // body is the author's untouched string.
    for (field, value) in &member.leaves {
        let mut block = String::new();
        let _ = write!(
            block,
            "<a id=\"{anchor}\"></a>\n**{label}:** {value}",
            anchor = leaf_anchor(member, field),
            label = capitalize(field),
        );
        blocks.push(block);
    }

    // Then the nested members, in key order — each a labelled group (`**Rejected:**`)
    // over its keyed entries, one anchored paragraph per entry leaf. The leaf's child
    // path is `<collection>.<entry>.<field>`, keyed at every level so the anchor
    // survives insertion and reorder (leaf addresses are structural and keyed).
    for (collection, entries) in &member.members {
        blocks.push(format!("**{}:**", capitalize(collection)));
        for (entry, entry_member) in entries {
            for (field, value) in &entry_member.leaves {
                let child_path = format!("{collection}.{entry}.{field}");
                let mut block = String::new();
                let _ = write!(
                    block,
                    "<a id=\"{anchor}\"></a>\n*{entry}* — **{label}:** {value}",
                    anchor = leaf_anchor(member, &child_path),
                    label = capitalize(field),
                );
                blocks.push(block);
            }
        }
    }

    let mut out = blocks.join("\n\n");
    out.push('\n');
    out
}

/// The anchor id of a nested member as a whole — its `<kind>.<key>` identity, the
/// deep-link target the member's heading carries.
fn member_anchor(member: &EmbeddedMember) -> String {
    format!("{}.{}", member.kind, member.key)
}

/// The anchor id of one leaf — `<kind>.<key>.<child-path>`, the `MemberAddress`
/// spelling minus the outer member segment (the emit face namespaces projections by
/// member).
fn leaf_anchor(member: &EmbeddedMember, child_path: &str) -> String {
    format!("{}.{}.{}", member.kind, member.key, child_path)
}

/// Capitalize a declared field/collection/kind key into its display **label** — the
/// first character uppercased, the rest untouched (`chosen` → `Chosen`, `rejected` →
/// `Rejected`). A label is the declared vocabulary in rendered form, not synthesized
/// prose: the word is the author's field name, only its case is the display rule's.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
