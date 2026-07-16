# Plan state

- Spec derived through: cc5a9b3
- Audited through: 74f4e62
- Residue swept through: 74f4e62
- This tick: RECONCILED the d97a704..74f4e62 window (1ca1f9b,
  EMBEDDED-FORMAT-TARGET-FACTS) — the audit motions 341717b left unrun. The
  window touches four files, all SDK-side (`emit.ts`, `kind.ts`, and two test
  files); no Rust file moved, so every Rust-side record on the fork board is
  unshifted by construction.
  **Audit:** the four target facts match `representation.md`'s closed set
  exactly — `EdgeTargetFacts` is `{name, address, kind, path}`
  (`sdk/src/kind.ts:330-340`), nothing fifth. `edgeFields` reaches a value off
  `definition.facts.edgeFields` at construction, the way `render` does
  (`kind.ts:381`) — kind-declared, never instance-authored, so "the edge
  fields — which fields are references" holds. Two of the three shipped
  refusals check out against `pipeline.md`'s "Refusing" bullet: a dangling edge
  refuses, and the mention deferral is scoped to mentions by its own words, so
  an edge target refusing is licensed rather than a collision. No pending
  entry's work shipped; neither queued entry drops.
  **Sweep:** the third refusal is a real divergence — an *unfilled* edge field
  refuses at emit (`emit.ts:186-193`), baking a presence floor the spine rule
  and invariants 1/4 make a dialable clause. Registered as
  **`(edge-field-floor)`**, not an entry: the clause layer cannot reach an
  embedded value's leaves today (`has_field` reads host frontmatter,
  `extract.rs:287`), so every fix picks a spec outcome plan does not write. The
  window's only other residue is the `law 5` fixture strings in emit.test.ts —
  the excluded body-text class, not a cite.
  Closing checklist: FORMAT-OMITS-EDGE-CLAUSE's cites re-read on disk and
  unmoved (contract.rs:81/475, engine.rs:97/114); PROJECTION-PATH-SEAM-GATE's
  test file is still absent, gate stays `open`. PACKAGING-CHANNELS-REMAINDER's
  park re-tested at 74f4e62 and true verbatim: no version tag (4 tags, all
  era-named), crate 0.1.0 vs npm 0.0.7, release.yml:7-9 still defers darwin.
  `(nested-file-child)`'s three cites re-verified against the window's own
  file — kind.ts:158, kind.ts:57, declarations.ts:438, all unshifted.
- Queue: 2 pickable and disjoint by file (FORMAT-OMITS-EDGE-CLAUSE;
  PROJECTION-PATH-SEAM-GATE — one test file); PACKAGING-CHANNELS-REMAINDER
  parked (John's Apple notarizing + the v0.1 lockstep tag). Fork board grows by
  one: `(edge-field-floor)`, blocking nothing. `(nested-file-child)` still
  blocks the supporting-doc adoption; `(guidance-climb)` blocks nothing.

Plan continues: no — every input is drained. The inbox is empty, no refactor
capture is live, the spec delta is empty at cc5a9b3, and both reconciliation
cursors now sit at the window's last shipped commit. Build takes over: two
entries are pickable.
