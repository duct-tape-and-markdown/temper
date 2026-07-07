# Plan state

- Spec derived through: d5d1b37
- Audited through: c685a93
- Residue swept through: 32f0c32
- This tick: residue sweep c349cb2→HEAD. Only code commit past the cursor is the
  MODE ship (cb0814f); SATISFIER was swept at e9dec1c. Verified on disk:
  compose::Authority is fully renamed to EnforcementMode (no `Authority` symbol
  survives in src/ — case-sensitive grep empty), install.rs reads the guard
  posture off the field, and the SDK's fabricated `authority`/`shared` emit fact
  is gone. One inert trace: tests/emit.rs:235 uses `fact:"authority"` as a
  generic AssemblyFactRow sample (asserts only the row-family header, nothing
  about the retired concept; `value:"shared"` is still the live mode value) — a
  root field may be named anything, so not corpus-forbidden vocabulary, stale
  only in flavor. Rides the next entry to open tests/emit.rs; no fileable class.
  Residue cursor → HEAD.
- Queue: 2 entries — GENRE-FOLD (open, pickable, sole open), PACKAGING-CHANNELS
  (parked on release creds). Disjoint: GENRE-FOLD's src/tests vs PACKAGING's
  release.yml/package.json — no shared path.

Plan continues: no — inbox/spec-delta/ship-audit quiet and residue is at HEAD;
the one open entry (GENRE-FOLD) is pickable, so build takes over.
