# Plan state

- Spec derived through: 474835b
- Audited through: 50e7094
- Residue swept through: e32ed53
- This tick: Ship audit 0594038..HEAD (job 3). COMMAND-KIND shipped (efd6caa,
  chore 50e7094) — verified on disk: four kinds now (`claude_code_command()`
  builtin_kind.rs:86, `all_kinds()` :157, module doc "four kinds"; lock narrates
  four; command floor = skillFloor minus nameMatchesDir). AGENT-KIND's core work
  (named-field identity, the third mode) confirmed still unbuilt: `UnitShape`
  enum kind.rs:111-119 is File/Directory only, `from_source` :152 derives id
  from stem/dir, SDK `identityField` kind.ts:69-72 still projection-only. Gate
  `blockedBy: COMMAND-KIND` clears → AGENT-KIND recut to `open` with refreshed
  line refs (all_kinds 109→157, export block shifted) + scoped 50e7094. Audit
  cursor → HEAD.
- Queue: 2 — AGENT-KIND (open, pickable; touches builtin_kind.rs/kind.rs/
  frontmatter.rs/builtin_lock.toml + sdk) → PACKAGING-CHANNELS (parked on human
  release creds + engine-binary workflow; touches package.json/.github).
  Disjoint: one open entry, PACKAGING file-disjoint from it.

Plan continues: yes — residue sweep (job 4): Residue swept through e32ed53
trails HEAD; COMMAND-KIND's ship (efd6caa) is unswept.
