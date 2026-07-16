# Plan state

- Spec derived through: 39a4833
- Audited through: b3327fb
- Residue swept through: b3327fb
- This tick: RECONCILE the 27a2e28..b3327fb window — one src commit in it
  (a330b56, LOCK-FILENAME-ONE-HOME: drift.rs alone; b3327fb already drained the
  entry). **Audit:** verified on disk, not off the log — all five sites
  (1436/1812/1939/2075/2738) now read `workspace_dir.join(crate::LOCK_FILENAME)`,
  and the entry's own acceptance holds where its predecessor's did not:
  `rg '"lock\.toml"' src/` returns lib.rs:25's const alone, so with `.temper`
  already consolidated both halves of the one-home pass are complete in `src/`.
  The commit's two named scope calls are honest and I sustain them: drift.rs's
  prose doc comments naming `lock.toml`/`./.temper` name the concept to a reader,
  not the literal to the compiler, and the ~50 test-fixture spellings across
  tests/ are test data, not a const surface. PACKAGING-CHANNELS-REMAINDER's park
  re-tested — no version tag (era tags only), crate 0.1.0 vs npm 0.0.7,
  release.yml still linux-x64+win32-x64 with darwin/notarize/bundle absent —
  still unmet, stays parked; its notes restamped to b3327fb. **Sweep:** one
  finding, filed as SETTINGS-PATH-ONE-HOME — the same "One job, one home" class,
  third sibling: install.rs spells `root.join(".claude").join("settings.json")`
  at 559 and 583, one job (where install's settings write lands) with two
  spellings and no home. Scoped to a private helper, NOT a lib.rs const beside
  WORKSPACE_DIR/LOCK_FILENAME — those two are temper's own names, while
  `.claude`/`settings.json` are harness format facts already homed in the kind's
  own source (builtins.md, "The shipped kinds"), so a const would mint the very
  second home the entry exists to remove. Three near-misses checked and excluded
  as different jobs, not re-spellings: coverage_note.rs:240 (the unclaimed-entry
  scan's dir), builtin_kind.rs + builtin_lock.toml (`governs` data), fixtures.
  No rider moved: the window touched drift.rs alone, which no "rides X" record
  names — builtins.ts's two survivors (565/611) and session_start.rs's `+++`
  fixtures (128/133/146) re-verified unshifted on disk, every other cite
  untouched.
- Queue: 1 pickable (SETTINGS-PATH-ONE-HOME); PACKAGING-CHANNELS-REMAINDER
  parked (John's Apple notarizing + the v0.1 lockstep tag) — condition
  re-tested, still unmet.

Plan continues: no — reconciliation was the last input in the order, and both
cursors now sit at HEAD. Spec cursor copied forward verbatim (delta empty;
39a4833 fully derived). Build takes over: the queue has one pickable entry.
