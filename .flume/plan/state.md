# Plan state

- **Phase:** reconcile. HEAD 4765d5b.
- **Last shipped:** ADAPTER-EQUIVALENCE-PIN (4765d5b) — the byte-fidelity
  baseline (`tests/adapter_fidelity.rs`) the declared-adapter swap must not move.
  Disk audit: `src/skill.rs`/`src/rule.rs` still present (786/735 lines), used
  **only** at the import/apply/re-add/drift/bundle/check adapter faces; the
  read/check path is already generic.
- **This tick:** verified the human gate on DECLARED-FRONTMATTER-ADAPTER is still
  open — `kinds/skill/KIND.md` + `kinds/rule/KIND.md` carry no `format`/unit-shape
  keys (build's fence excludes `kinds/`). Refined that entry: its
  ADAPTER-EQUIVALENCE-PIN prerequisite has shipped, so the sole remaining gate is
  the human KIND.md edit. Confirmed EXTRACTION-VOCAB-GAPS's two gaps still exist on
  disk (`Primitive` = field/headings/sections/line_count/placement; `Field` reads
  flat `frontmatter.get(key)`, no key-path). Inbox empty; other entries unchanged.
- **Pickable now:** none — every entry is parked or deferred; the queue is fully
  gated on human action.
- **Human-gated:** add `format = "yaml-frontmatter"` + a unit-shape line to
  `kinds/skill/KIND.md` (directory+companions) and `kinds/rule/KIND.md` (flat file)
  → un-parks DECLARED-FRONTMATTER-ADAPTER. Also gated: the `(edge-representation-unify)`
  fork; release creds (PACKAGING-CHANNELS); the chain.ts fence-widen + private vuln
  reporting (COMMUNITY-DOCS).

Plan continues: no — queue reconciled, inbox empty, no pickable work; the
substantive wave is human-gated on the curated KIND.md `format` declarations.
