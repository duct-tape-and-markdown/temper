# Plan state

- **Phase:** reconcile. HEAD 4765d5b.
- **Last shipped:** ADAPTER-EQUIVALENCE-PIN (4765d5b) — the byte-fidelity baseline
  (`tests/adapter_fidelity.rs`) the declared-adapter swap must not move.
- **This tick:** drained the inbox's sequencing correction. Verified on disk that
  `CustomKind::from_header` (kind.rs:111) rejects unknown header keys and the
  built-in KIND.md parse routes through that same guard (builtin_kind.rs:81) — so
  the human's format/unit KIND.md edit would redden main unless the parser accepts
  the keys first. Filed **FORMAT-KEY-PARSE** (open, pickable now: src/kind.rs +
  tests) to parse the inert `format`/unit-shape fields, and re-sequenced
  DECLARED-FRONTMATTER-ADAPTER behind it. Routed the MEMORY-KIND directional note
  to the CLAUDE.md-bootstrap kept-on-purpose line (do-not-file-yet honored).
- **Pickable now:** FORMAT-KEY-PARSE. Everything else parked/deferred.
- **Next:** FORMAT-KEY-PARSE ships → humans add `format`/unit lines to
  kinds/{skill,rule}/KIND.md (chore) → DECLARED-FRONTMATTER-ADAPTER un-parks.

Plan continues: no — queue reconciled, inbox drained, FORMAT-KEY-PARSE is
pickable; hand to build.
