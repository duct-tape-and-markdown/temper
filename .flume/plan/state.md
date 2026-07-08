# Plan state

- Spec derived through: cd7135b
- Audited through: d8405d7
- Residue swept through: aea39c3
- This tick: Inbox. Routed the one line (dogfood re-conversion finding,
  2026-07-07) into a new pending entry, INSTALL-FINGERPRINT-SETTLES-
  FIRST-RUN. Re-verified the claimed gap against disk before scoping (per
  job 1's rule): traced `run_represented` (src/install.rs) — it calls
  `drift::emit_program` (writes lock.toml's `emit_hash`/`source_hash` rows
  from pre-placement bytes) then `evaluate_placements` (mutates those same
  emit-owned files: managed-by note, schema modeline) with no re-stamp
  after, so `drift::config_stale` genuinely reports a fingerprint mismatch
  on a fresh install's own output. Also found: `emit_one` (src/drift.rs)
  already folds on-disk placement lines into its render+hash on a later
  pass, and tests/install.rs's
  `re_representing_never_re_scaffolds_and_settles_by_the_second_run`
  already documents+accepts the two-run settle as by-design — the inbox
  finding supersedes that acceptance with new intent (first run must
  settle). Entry cites specs/model/pipeline.md "Install" (the exact
  emit-then-place sequence), scopes both the install.rs fix and the
  test's rewrite. Drained the inbox line.
- Queue: INSTALL-FINGERPRINT-SETTLES-FIRST-RUN (open, ready) ahead of
  PACKAGING-CHANNELS (parked, condition unchanged this tick).

Plan continues: yes — residue sweep trails HEAD (aea39c3 vs f684b56); the
one commit between them (f684b56, chore(harness) dogfood re-conversion)
touches no src/tests/sdk so a sweep should be quick, but that job was not
taken this tick (inbox took priority) and needs its own tick.
