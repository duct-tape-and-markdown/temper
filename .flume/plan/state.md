# Plan state

- Spec derived through: a9f7b9e
- Audited through: c5df845
- Residue swept through: c5df845
- This tick: Inbox. Drained the one note (`coverage.unmodeled-surface`
  misstates settings.json, observed at 15a71ac) into a new `open` entry
  COVERAGE-PARTIAL-GOVERNANCE. Re-verified the gap forward of the note's sha:
  the mcp-server ship (1ffab8f, after 15a71ac) reworked coverage_note.rs's
  `governs` with `spans_whole_manifest` and retired .mcp.json's finding, but
  left settings.json's segment case — probe `cargo run -- check .temper` at
  HEAD (ff84346) still emits `hook (3)` under coverage.checked AND
  `.claude/settings.json ... no kind governs it — temper checks none of its
  members` under coverage.unmodeled-surface in one run: a live invariant-6
  false diagnostic. Not deferrable to parked MANIFEST-WRITE-SIDE (retires the
  finding only at full representation). Entry: open, per representation.md
  "Reach", edits src/coverage_note.rs + tests/coverage_note.rs. Both audit/
  residue cursors copied forward verbatim — reconciliation is next tick's job.
- Queue: COVERAGE-PARTIAL-GOVERNANCE (open, next) → MANIFEST-WRITE-SIDE
  (parked, phase 2) → PACKAGING-CHANNELS (parked). Disjoint: only the head is
  `open`. NB MANIFEST-WRITE-SIDE also edits src/coverage_note.rs but is parked
  (not pickable), so no merge conflict; its full-retirement work builds on
  this partial fix once re-scoped.

Plan continues: yes — post-ship reconciliation, window c5df845..HEAD.
MCP-SERVER-KIND shipped (1ffab8f/ff84346): audit drops nothing new from
pending (its chore already did) but must re-test MANIFEST-WRITE-SIDE's parked
reason (now stale — it says MCP-SERVER-KIND "has not" shipped) and sweep
c5df845..HEAD for residue.
