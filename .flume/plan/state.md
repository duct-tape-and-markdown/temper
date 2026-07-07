# Plan state

- Spec derived through: f4189c3
- Audited through: 44b4f27
- Residue swept through: caed5cf
- This tick: Ship audit 75798a5..44b4f27. Three build entries shipped, verified
  on disk (not the log): EMIT-REAP-ORPHANS (drift.rs `reap_or_report_orphan`,
  `EmitOutcome::Reaped`/`OrphanDrift`, `emit_owned_targets`), UNCLAIMED-ENTRY-
  ADVISORY (coverage_note.rs `UNCLAIMED_RULE="coverage.unclaimed-entry"`, one
  advisory finding per stray `.claude/` entry), GUARD-OWNPATH (main.rs:343-347
  grounds the guard in `drift::emit_owned_targets`). Both were gate blockers —
  re-tested NOW: EMIT-LF-NORMALIZE (blockedBy EMIT-REAP) → open, SETTINGS-
  FORMAT-PRESERVING (blockedBy GUARD-OWNPATH) → open. The two shipped commits
  reworked drift.rs (+160) and install.rs (+66), drifting both entries' line-
  cites; re-verified against HEAD and re-scoped (drift.rs emit_one 516→654,
  desired :690, hash :711, write :723; install.rs project_settings 665→697,
  to_string_pretty :765, stale JSON comment 660-664→693-696) with scoped-at
  44b4f27. MODULE-RELATIVE-PATHS stays blockedBy SETTINGS (its install.rs cites
  drift again when SETTINGS ships — re-verify at unblock), copied forward.
  PACKAGING-CHANNELS parked reason re-checked (no release.yml, root package.json
  still private flume manifest) — unchanged. Spec cursor f4189c3 + Residue
  caed5cf copied forward verbatim (jobs 1/2 quiet; residue trails, job 4 live).
- Queue: 4 — two disjoint open fronts: EMIT-LF-NORMALIZE (drift.rs+tests/emit.rs),
  SETTINGS-FORMAT-PRESERVING (install.rs+tests/install.rs). MODULE-RELATIVE-PATHS
  blockedBy SETTINGS (install.rs chain tail; +SDK), PACKAGING-CHANNELS parked.

Plan continues: yes — residue sweep caed5cf..44b4f27 is live: the three build
commits (f8e039b/4049691/6fd37f5) sit past the residue cursor. A known class to
sweep: drift.rs:1405 "four declaration families" comment lags the six the code
extracts — comment staleness that rides EMIT-LF-NORMALIZE (next entry opening
drift.rs), verified next tick against the full range.
