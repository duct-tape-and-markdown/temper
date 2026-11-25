# Reference renders kind topics

Shipped: `kind-<k>.md` per committed kind in play, beside `SKILL.md`, as a
`supporting-doc` of the reference (sdk/src/reference.ts `kindTopic`).

Seam (sdk/src/declarations.ts): `kindEdges` is now the one edge-intent
assembly; `guidanceBody` reads it and filters shipped sources itself.
`kindLocusGlobs` exposes the guidance rule's `locusGlobs`. `clauseField` and
`targetList` exported. One visible change to guidance rules: outbound edge
bullets now follow `edgeFields` order, not `fieldGuidance` key order (no
test or dogfood kind had two counselled edges, so nothing moved).

Judgment calls plan may want to ratify:
- The kind topic states a built-in's *shipped* counsel and edge intent;
  the guidance rule suppresses those (it would project unasked rules), but
  the reference topic exists regardless and "each kind's counsel" reads as
  the kind's, shipped or not.
- Fields = leaf set ∪ counselled fields, edge fields listed under Edges.
- Clauses are listed by predicate/field/severity/guidance, not by compiled
  label: the `when` label field is composed Rust-side
  (`when_label_field`) and the SDK has no copy. An unbound kind says "any
  default contract the kind ships is what holds it" — the SDK can't see
  the rows-or-default floor.
