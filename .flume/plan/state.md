# Plan state

- **Phase:** reconcile. HEAD 56732ee.
- **Last shipped:** FORMAT-KEY-PARSE (56732ee) — `CustomKind` now carries typed
  `format`/`unit_shape` fields (closed-vocab guarded, inert), so a human can add
  the two curated KIND.md lines without reddening main.
- **This tick:** confirmed FORMAT-KEY-PARSE landed on disk (kind.rs Format/UnitShape
  enums, parse_format/parse_unit_shape, allowed-key set kind.rs:147-159) and that
  kinds/{skill,rule}/KIND.md still lack the `format`/unit-shape lines. Rewrote
  DECLARED-FRONTMATTER-ADAPTER's park reason — both build prerequisites
  (FORMAT-KEY-PARSE, ADAPTER-EQUIVALENCE-PIN) now shipped, only the human KIND.md
  chore remains. Refreshed EXTRACTION-VOCAB-GAPS line cites (kind.rs:322-344/366).
  Inbox empty; no fork resolved.
- **Pickable now:** nothing — every entry parked/deferred (all on human action or
  no-consumer).
- **Next (human):** add `format="yaml-frontmatter"` + a unit shape to
  kinds/{skill,rule}/KIND.md → DECLARED-FRONTMATTER-ADAPTER un-parks → build picks
  it, then MEMORY-KIND files the tick after.

Plan continues: no — queue reconciled, inbox empty, no build-pickable work (all
human-gated); a re-plan would only re-emit an unchanged queue.
