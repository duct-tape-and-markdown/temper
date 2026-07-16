# Plan state

- Spec derived through: abe5d5d
- Audited through: cc8d823
- Residue swept through: cc8d823
- This tick: RECONCILED 4a33911..cc8d823 — audit plus sweep in one tick, the
  window being one build commit wide. **Audit:** f60e1ff's consolidation is
  real on disk, not merely logged — `common::wire_sdk_harness(label, program)`
  lives at `tests/common/mod.rs:127` beside the ingredients it composes
  (`tmpdir`, `vendor_sdk`), and all three named suites now call it:
  `builtin_lock_frozen.rs:80`, `projection_path_seam.rs:176`, and `emit.rs`
  through its thin four-caller `wire_sdk_harness(label)` binding (1215-1216)
  plus three direct `common::` calls. SDK-FIXTURE-WIRING-ONE-HOME shipped at
  7947235 and was already absent from pending — a verification, not a drop.
  Both parks re-tested on disk at this HEAD and hold verbatim: no version tag
  (only the four era tags), crate 0.1.0 vs npm 0.0.7, release.yml:7-9 still
  deferring darwin + channel 3; the hop constant still reads 5 at
  `src/graph.rs:62` under a cite claiming five, and nothing ruled its
  semantics. LOCK-LEGACY-TEMPLATES-READ's cites re-resolve unmoved
  (`templates_from_table` 3139-3157, the refusal at 3150).
  **Sweep — one finding, filed:** the consolidation left a fourth copy it
  never named. `tests/nested_member.rs:262-266` re-derives the exact four
  steps `wire_sdk_harness` now owns (tmpdir → create `.temper` → write
  `harness.ts` → vendor the SDK). Proven a miss rather than a regression: the
  copy was written at 91c288c, predating f60e1ff, whose body claimed "three
  copies" while naming only `install.rs` as the justified exception — and that
  exception re-verified as sound, since install scaffolds its own `harness.ts`
  and then rewrites it (install.rs:732/879), a different job from wiring an
  authored fixture program. Routed to FIXTURE-WIRING-FOURTH-COPY (`per`
  engineering.md, "One job, one home"), open. Structural residue, so it is
  standalone-fileable — not the comment-staleness class that may only ride.
  Every named rider re-verified on disk and unmoved, each still carrier-less:
  session_start.rs 121/140, read.rs 270/495/633/770/1172, prose.ts (law 5 at
  6/141/258, law 8 at 11, posture at 126/156/161/188/238), Cargo.toml 42-45,
  compose.rs:233 — stamps advanced to cc8d823. One fork record corrected
  rather than restamped: `(seam-rows-public-face)` cited `PayloadMember` inside
  the 84-94 block, but it exports at 97 off `./emit.js` — the ten-export count
  the record turns on is unchanged, the cite now resolves.
  Closing checklist: two open entries, file-disjoint (drift.rs +
  lock_declaration_rows.rs vs nested_member.rs), so build's parallel fan-out
  cannot collide; the two parks share no file with either. No fork resolved or
  opened. Field lengths validated. `Spec derived through:` copied forward
  verbatim — this tick derived no spec; the delta is empty.
- Queue: 2 pickable (LOCK-LEGACY-TEMPLATES-READ, the 0024 read-robustly gap;
  FIXTURE-WIRING-FOURTH-COPY, filed this tick); 2 parked on human acts
  (IMPORT-HOP-CAP-CITE: a hop-depth probe. PACKAGING-CHANNELS-REMAINDER: Apple
  notarizing + the v0.1 tag). No gate is stale — every one was tested this tick.

Plan continues: no — inbox drained and empty, no refactor captures, spec delta
empty at abe5d5d, and 4a33911..cc8d823 is now reconciled on both motions with
both cursors advanced. Build takes over: two pickable entries, disjoint.
