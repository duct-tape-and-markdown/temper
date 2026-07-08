# Plan state

- Spec derived through: fa7f87c
- Audited through: 1e5d7ec
- Residue swept through: 31cd09a
- This tick: Spec delta (job 2). Fork-routed fa7f87c (0013, format two one-way
  faces): the ruling fixes the format-layer *semantics* (injective-render
  admissibility, declared leniency, no-logic bar, schema-slot delegation,
  total-with-errors) but the `format` fact is today an inert string label
  (`Format` in sdk/src/kind.ts + src/kind.rs, lock `KindFactRow.format`, consumed
  by nothing) — recutting it to template data + a composed renderer/extractor
  needs the SDK/lock/emit-payload spelling John hasn't set. Registered
  `(format-template-spelling)`; wired `(json-projection-format)` downstream of it.
  Cursor 9a61c2b → fa7f87c. bdc938d (0014, command+agent kinds) left un-derived.
- Queue: 2 — EMBEDDED-FENCE-MEMBER-NOUN open/pickable; PACKAGING-CHANNELS parked
  (human release creds + engine-binary workflow). Disjoint file sets. No new
  pending entry this tick — 0013 is fork-routed, not build-derivable.

Plan continues: yes — bdc938d (0014, command+agent kinds) is un-derived spec
delta, the next live input below job 2.
