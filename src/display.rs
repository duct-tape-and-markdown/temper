//! The per-genre **display rule** — a genre value's projection markdown.
//!
//! [`render_genre`] compiles a [`GenreValue`] (a genre fence's extracted leaves and
//! keyed collections) into the markdown a projection carries — a heading, the leaf
//! labels, deterministic ordering, and per-leaf anchors — as **connective tissue
//! only** (`specs/architecture/20-surface.md`, "The display rule owns connective
//! tissue"): every meaning-carrying word is an authored leaf, rendered verbatim
//! (law 5); the heading, labels, and anchors are the value's declared structure in
//! rendered form — the markdown analogue of the manifest's TOML syntax, never
//! synthesized prose. One rule serves every genre: the labels and heading derive
//! from the value's own declared genre name and field keys, so it is per-genre by
//! data, not by a hard-coded genre.
//!
//! A **standalone formatter island**: the future custom-kind emit face calls it and
//! must reproduce these exact bytes; it does not wire into `src/drift.rs`'s
//! skill/rule projection (that projection carries no genre today). Byte-deterministic
//! — a double-render of the same value is byte-identical (law 5), the property
//! `tests/display_rule.rs` pins as the emit face's contract.

use std::fmt::Write;

use crate::extract::GenreValue;

/// Render a [`GenreValue`] to its projection markdown — the anchored heading, then its
/// **leaves before its collections** (`specs/architecture/20-surface.md`, the display
/// rule; the ordering the read family's [`GenreValue::addressed_leaves`] also walks),
/// each in `BTreeMap` key order so the output is byte-deterministic. Every leaf value
/// is rendered verbatim (law 5); the heading, the capitalized labels, and the anchors
/// are the value's declared structure, not prose. An empty value renders its heading
/// with no body.
///
/// Anchors are addressed by the leaf's `<genre>.<key>.<field-path>` — the `LeafAddress`
/// spelling of `src/read.rs`, minus the `member` segment the emit face namespaces
/// projections by. Cross-member anchor uniqueness is therefore the emit face's to own;
/// this island renders one value's self-consistent block (the accepted island risk).
#[must_use]
pub fn render_genre(genre: &GenreValue) -> String {
    // Each block is a self-contained markdown paragraph; joined by a blank line so the
    // rendering is one deterministic assembly with no trailing-whitespace ambiguity.
    let mut blocks: Vec<String> = Vec::new();

    // The value's heading, anchored by its `<genre>.<key>` identity — the deep-link
    // target a citation resolves to. `###` matches the corpus's decision-heading level.
    blocks.push(format!(
        "<a id=\"{anchor}\"></a>\n### {genre_label}: {key}",
        anchor = value_anchor(genre),
        genre_label = capitalize(&genre.genre),
        key = genre.key,
    ));

    // Top-level leaves first, in key order — each an anchored, labelled paragraph whose
    // body is the author's untouched string.
    for (field, value) in &genre.leaves {
        let mut block = String::new();
        let _ = write!(
            block,
            "<a id=\"{anchor}\"></a>\n**{label}:** {value}",
            anchor = leaf_anchor(genre, field),
            label = capitalize(field),
        );
        blocks.push(block);
    }

    // Then the keyed collections, in key order — each a labelled group (`**Rejected:**`)
    // over its keyed entries, one anchored paragraph per collection leaf. The leaf's
    // field path is `<collection>.<entry>.<field>`, keyed at every level so the anchor
    // survives insertion and reorder (leaf addresses are structural and keyed).
    for (collection, entries) in &genre.collections {
        blocks.push(format!("**{}:**", capitalize(collection)));
        for (entry, leaves) in entries {
            for (field, value) in leaves {
                let field_path = format!("{collection}.{entry}.{field}");
                let mut block = String::new();
                let _ = write!(
                    block,
                    "<a id=\"{anchor}\"></a>\n*{entry}* — **{label}:** {value}",
                    anchor = leaf_anchor(genre, &field_path),
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

/// The anchor id of a genre value as a whole — its `<genre>.<key>` identity, the
/// deep-link target the value's heading carries.
fn value_anchor(genre: &GenreValue) -> String {
    format!("{}.{}", genre.genre, genre.key)
}

/// The anchor id of one leaf — `<genre>.<key>.<field-path>`, the `LeafAddress` spelling
/// minus the member segment (the emit face namespaces projections by member).
fn leaf_anchor(genre: &GenreValue, field_path: &str) -> String {
    format!("{}.{}.{}", genre.genre, genre.key, field_path)
}

/// Capitalize a declared field/collection/genre key into its display **label** — the
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
