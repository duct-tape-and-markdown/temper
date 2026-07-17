# Plan state

- Spec derived through: b8396d4
- Audited through: 8913b59
- Residue swept through: 8913b59
- This tick: INBOX — two human-routed notes drained, both verified on disk at
  c370924 and filed as pickable entries. Cursors copied forward verbatim: the
  spec delta (3c1a58c, decision 0030) and the 8913b59..c370924 window are
  untouched by this tick, and inbox outranks both.
  **Note 1 — `emit --into` re-bases every lock row ⇒
  LOCK-ROW-PATHS-HARNESS-RELATIVE.** Verified deeper than the report: the
  corruption is not a writer slip but *both faces agreeing on the wrong base*.
  `harness_root = workspace_dir.parent()` (drift.rs:839) is cwd-relative, so
  `harness_root.join(...)` (955) bakes the cwd prefix into every provenance
  row; readers then take rows raw (`fs::read(&row.source_path)` 2174,
  `remove_file` 1173). Writer and reader therefore agree — but only while cwd
  IS the harness root, which is why the suite is green and the field report
  saw exit 0. Two disk facts settle the fix-vs-refuse fork the note leaves
  open, so it needed no ruling: (a) the lock's own two row families already
  disagree in one emit pass — include rows go through `harness_relative`
  (1003, the helper at 1508), member rows do not; (b) pipeline.md ("Install")
  binds the verbs to "one project's harness at an explicit path", which a
  cwd-relative committed row cannot survive. Refusal is rejected in the entry:
  `--into`'s only non-default use is naming a workspace off cwd. Ripple
  measured, not guessed — `common::write_lock` passes `members: Vec::new()`,
  writes no provenance rows, so the shared-fixture fan-out the entry rule
  warns about does not reach its callers; scope is drift.rs + tests/emit.rs.
  **Note 2 — `file()` inside `blocks()` raw-TypeErrors ⇒
  SDK-BLOCKS-FILE-REFUSAL.** The note names legality as the entry's first
  question; **the SDK's own surface has already answered it**, so this is
  derivation, not a fork: `blocks(...values: (Text | EmbeddedMemberValue)[])`
  (prose.ts:242) excludes `File` (130) by type — strict tsc rejects the
  spelling, and only a JS or cast caller reaches `isTextSpan` (251) falling
  through to `renderMemberBlock` (emit.ts:353) → `resolveMemberLeaves` (288)
  dereferencing absent `leaves`. The entry makes the runtime hold the line the
  type draws, mirroring `text`'s existing throw (205). Legalizing it was
  weighed and rejected: `include()` (232) already homes file-bytes-in-a-body
  and is the path the lock fingerprints — a second home would be
  unfingerprinted.
  **The `prose.ts` narration rider has a carrier at last.** Nine entries'
  worth of precedent says it discharges when an entry NAMES it, never when the
  file is merely opened — SDK-BLOCKS-FILE-REFUSAL edits `blocks()`, whose own
  doc comment (238) is one of the ten stale lines, so the entry names all ten
  and the fork record now points at it.
  **Friction captures left alone** — `.flume/friction/` is the human's triage
  surface, not plan's to drain; only `.flume/refactor/` (empty, README only)
  is. The `--into` capture's product half is now queued, which is what its own
  "Suggested fix: inbox item" asked for.
- Queue: 4 entries — 2 pickable (LOCK-ROW-PATHS-HARNESS-RELATIVE and
  SDK-BLOCKS-FILE-REFUSAL, both scoped at c370924), 2 parked on human acts. No
  file appears in two entries — checked mechanically; the two pickable entries
  are disjoint (`src/drift.rs`+`tests/emit.rs` vs `sdk/src/prose.ts`+
  `sdk/test/refusals.test.ts`), so they fan out in parallel safely.

Plan continues: yes — the spec delta is the next live input: 3c1a58c
(decision 0030, "review is the price of softening") sits past the
`Spec derived through: b8396d4` cursor, un-derived; its Consequences section
is the next tick's derivation checklist. Post-ship reconciliation of
8913b59..c370924 (eb9674c, the fixture fold) follows it.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
(c86d649 scoped that gate to plan's own commits, so the misfire cannot recur —
the orphaned record itself still needs a human hand.)

**Waiting on a ruling:** `(clause-vocabulary-holds)` is still the fork board's
largest — four shipped contracts hold decidable, documented rules the algebra
cannot spell, and the corpus sanctions only "undecidable" as a reason a clause
is absent. Unmoved this window. Nothing is broken; what it costs is that the
gate's reach is thinner than `specs/builtins.md`'s "Strictest documented
profile" stance reads.
