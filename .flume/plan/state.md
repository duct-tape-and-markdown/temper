# Plan state

- Spec derived through: 39a4833
- Audited through: 169e62f
- Residue swept through: 169e62f
- This tick: RECONCILE the f632c71..169e62f window — one src commit in it
  (7367c1e, INSTALL-REPRESENTED-SKIPS-THE-QUESTION: install.rs/main.rs/
  tests/install.rs; 169e62f already drained the entry). **Audit:** both halves
  verified on disk and re-probed live rather than read off the log —
  `harness_diagnostics`'s body lifted into `resolve_harness_path` returning a
  typed `HarnessPath` (main.rs:664-745), read by both `check`'s gate and
  `install`'s represent decision, so the folded-in half landed as consolidation
  rather than a second resolver. Probes on this repo: non-interactive
  `install . --dry-run` asks nothing, reports "already represented —
  ./.temper/lock.toml answers the represent question", and places exactly the 3
  managed-by notes `check .` demands; `install .temper` refuses, naming `.`.
  Nothing left to drop. PACKAGING-CHANNELS-REMAINDER's park re-tested — no
  version tag (era tags only), crate 0.1.0 vs npm 0.0.7, release.yml still
  linux-x64+win32-x64 with darwin/notarize/bundle absent — still unmet, stays
  parked. **Sweep:** one finding, filed as WORKSPACE-DIR-ONE-HOME — 7367c1e
  consolidated the lock *probe* but left the literals it reads split five ways:
  `.temper` is a private const in four modules under two names (`TEMPER_DIR` in
  main.rs/import.rs/coverage_note.rs, `WORKSPACE_DIR` in install.rs, plus
  main.rs's `DEFAULT_WORKSPACE = "./.temper"`), `lock.toml` in two (`LOCK_FILE`,
  `LOCK_FILENAME`). One datum, five homes — `engineering.md`, "One job, one
  home" — and coverage_note.rs:41's own doc comment confesses the mirror. No
  rider moved: the window touched only install.rs/main.rs/tests/install.rs,
  which no "rides X" record names, so every line cite in open-questions holds
  unshifted. One rider gained a carrier — coverage_note.rs:76's `SETTINGS_DOC`
  date lag now rides WORKSPACE-DIR-ONE-HOME, which opens that file and names the
  bump in its file description; the record is annotated, not resolved.
- Queue: 1 pickable (WORKSPACE-DIR-ONE-HOME); PACKAGING-CHANNELS-REMAINDER
  parked (John's Apple notarizing + the v0.1 lockstep tag) — condition
  re-tested, still unmet.

Plan continues: no — reconciliation was the last input in the order, and both
cursors now sit at HEAD. Spec cursor copied forward verbatim (delta empty;
39a4833 fully derived). Build takes over: the queue has one pickable entry.
