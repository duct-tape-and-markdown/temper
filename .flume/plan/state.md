# Plan state

- Spec derived through: 5945405
- Audited through: b1cecfe
- Residue swept through: 8aeb64e
- This tick: Ship audit c7595d3..HEAD — the four build commits verified on disk
  (never the log): REQUIREMENT-GATE f0e1e74 (harness_diagnostics() now shared by
  `check --harness` + session-start, reads .temper/lock.toml's requirement/
  satisfies rows), WIN-INSTALL-SPAWN da6b127 (npm_program()->npm.cmd on windows,
  dependency assured before scaffold), WIN-EMIT-PATH 575aa2d (drift.rs), BUNDLE-
  REPORT-SHIPS 6fe273f (bundle.rs). No pending entry's work was completed by them.
  Unblocked two whose blockers shipped: SCAFFOLD-OUTPUT-VALID (SDK_VERSION_RANGE
  still ^0.0.2 :58, scaffold still omits Skill.description — live) and
  EXPLAIN-RESOLVER (retired-`import`-verb strings still in read.rs — live) ->
  both open. Audited cursor to HEAD.
- Queue: 9 — 5 open/disjoint (HELP-TEXT-RECUT main.rs, COVERAGE-KIND-AWARE
  coverage_note.rs, DISCOVERY-SKIPS-SURFACE import.rs, SCAFFOLD-OUTPUT-VALID
  install.rs, EXPLAIN-RESOLVER read.rs), 3 blocked (PATH-SEP-NORMALIZE on
  SCAFFOLD-OUTPUT-VALID, GUARD-OWNPATH on PATH-SEP-NORMALIZE — install.rs chain;
  MENTION-EDGE-LANDS on HELP-TEXT-RECUT for shared main.rs), PACKAGING-CHANNELS
  parked.

Plan continues: yes — residue sweep (Residue swept through 8aeb64e trails HEAD;
the 8aeb64e..HEAD window carries the four build commits' src work, unswept).
