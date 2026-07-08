# Plan state

- Spec derived through: cd7135b
- Audited through: d8405d7
- Residue swept through: eb88ffe
- This tick: Residue sweep aea39c3..eb88ffe. Only two commits in range
  (f684b56 chore(harness) dogfood re-conversion, eb88ffe plan: inbox) and
  neither touches src/tests/sdk, so re-swept the known retirement classes
  against the whole tree rather than just the diff. floor: still the
  established internal name for "default contract" everywhere it appears
  (src/builtin.rs, src/contract.rs, tests/*) — never corpus vocabulary, so
  no mismatch to file; BUNDLE-DESCRIPTION-DROPS-FLOOR-VOCAB's own
  tests/bundle.rs assertion (no "floor" in emitted plugin description
  text) still passes. posture: remaining hits (src/drift.rs:192,
  src/main.rs:126/144/508, src/read.rs:767/798, sdk kind.d.ts "posture 3")
  are the plain-English stance/locus-tier sense, not the retired
  enforcement-mode posture field — confirmed distinct, same finding as
  605575a. own_path: remaining hits are historical comments describing the
  retired lift, no live passthrough. temper.toml: one comment
  (src/compose.rs:12) noting the retired manifest-era reader, no live
  parser. kinds/+packages/ fixture debt in tests/session_start.rs remains
  the one standing accepted debt, unchanged, still riding the next
  session-start touch per open-questions' "Kept on purpose" note — not a
  new fileable gap. Nothing new to file; cursor to HEAD.
- Queue: INSTALL-FINGERPRINT-SETTLES-FIRST-RUN (open, ready) ahead of
  PACKAGING-CHANNELS (parked, condition unchanged this tick) — confirmed
  disjoint (install.rs/tests/install.rs vs release.yml/package.json, no
  shared paths).

Plan continues: yes — job 5 (quiet closing pass) is next: inbox, spec
delta, and ship audit are all empty/current, and residue is now swept
through HEAD, but that closing verification wasn't taken this tick to keep
the sweep its own atom.
