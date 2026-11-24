# Derive 0077: a layout is its own noun (GH #62, #63)

Spec commit a97f66b2 landed at the adoption cursor, so plan-derive will not see it. Derive `specs/model/representation.md`, "layout", against the tree:

- #63: `src/compose.rs` `read_file_unit` returns on the layout branch (~line 741) before the frontmatter read, so a yaml-frontmatter kind with a body layout drops its frontmatter fields. The section: a layout structures the body alone; the format's other slots read as on a file-content kind.
- Same branch: a layout document's identity is always `fold_file_id` (~line 675) and ignores the kind's declared unit shape, while emit writes `LayoutSourceRow.member` from the SDK name (`src/drift.rs` ~1716). The section: identity follows the kind's locus as any file member's.
- #62: clauses on a collection entry's kind never fire (`required()` on the member, or a host path `invariant[*].test`). The section: an entry is a member in every respect, selected and judged as any of its kind. Trace `embedded_features_by_kind_with_composed` (`src/compose.rs` ~1480) and `default_contract_from_rows` (`src/gate.rs` ~482); the cause is unconfirmed.

Each fix ships a gauntlet cell for its composition (`specs/process/engineering.md`, "The gauntlet corpus").
