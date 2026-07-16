# Plan state

- Spec derived through: 39a4833
- Audited through: a2f4a1c
- Residue swept through: a2f4a1c
- This tick: RECONCILE the b3327fb..a2f4a1c window — one src commit in it
  (c5307f8, SETTINGS-PATH-ONE-HOME: install.rs alone, +7/-2; a2f4a1c already
  drained the entry). **Audit:** verified on disk — `settings_path(root)` is a
  private helper at install.rs:556, and both call sites read it
  (`place_settings_only` 564, `evaluate_placements` 588). The commit's named
  divergence from the entry's literal acceptance is honest and I sustain it:
  the acceptance demanded `rg 'join\(".claude"\)\.join\("settings.json"\)'`
  return nothing, but a one-home entry's home must spell the literal
  somewhere — the regex cannot tell the one home from a re-spelling of it, so
  the fault is the acceptance's (mine, at scoping), not the build's. The intent
  it was written for holds exactly: line 557 is now the sole spelling in `src/`.
  The two rejections the body records are both right — a lib.rs const would
  mint the second home the entry existed to remove, and deriving the path from
  builtin_kind.rs's hook `Governs` conflates a discovery glob with a write
  path. PACKAGING-CHANNELS-REMAINDER's park re-tested — era tags only (no
  version tag), crate 0.1.0 vs npm 0.0.7, release.yml still linux-x64+win32-x64
  with darwin deferred in its own header comment (lines 7-8) — still unmet,
  stays parked; notes restamped to a2f4a1c.
  **Sweep: nothing filed — the "One job, one home" vein in `src/` is drained.**
  Its three siblings (WORKSPACE-DIR 23c31c4, LOCK-FILENAME a330b56,
  SETTINGS-PATH c5307f8) have no fourth: nothing outside install.rs computes a
  settings write path, and every surviving `.claude`/`.mcp.json`/`settings.json`
  spelling is a different job, not a re-spelling — builtin_kind.rs:212/233/240
  + builtin_lock.toml:44/54/58 (`governs` address data), coverage_note.rs:240
  (the unclaimed-entry scan's dir), and test fixtures across
  coverage_note.rs/import.rs/json_manifest.rs (test data, not a const surface).
  No rider moved: the window touched install.rs alone, which no "rides X"
  record names — builtins.ts's two survivors (565/611) and session_start.rs's
  `+++` fixtures (128/133/146) re-verified unshifted on disk.
- Queue: 0 pickable; PACKAGING-CHANNELS-REMAINDER parked (John's Apple
  notarizing + the v0.1 lockstep tag) — condition re-tested, still unmet.

Plan continues: no — reconciliation was the last input in the order, and all
three cursors now sit at HEAD. Spec cursor copied forward verbatim (delta
empty; 39a4833 fully derived). **The loop hibernates: the queue has no
pickable entry.** Every remaining input is John's — the two release actions
the park names.
