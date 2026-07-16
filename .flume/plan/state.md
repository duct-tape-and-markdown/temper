# Plan state

- Spec derived through: 39a4833
- Audited through: 27a2e28
- Residue swept through: 27a2e28
- This tick: RECONCILE the 169e62f..27a2e28 window — one src commit in it
  (23c31c4, WORKSPACE-DIR-ONE-HOME: lib.rs/main.rs/install.rs/import.rs/
  coverage_note.rs; 27a2e28 already drained the entry). **Audit:** verified on
  disk, not off the log — `WORKSPACE_DIR`/`LOCK_FILENAME` now sit beside
  `VERSION` in lib.rs:21/25, `pub` because main.rs is a separate bin crate, and
  all four modules read them at their use sites; `rg '"\.temper"' src/` returns
  lib.rs's const alone, so the workspace-dir half is complete. The entry's two
  named judgement calls hold: `DEFAULT_WORKSPACE` is a `LazyLock<String>`
  deriving `./` + the const (main.rs:51 — `concat!` takes only literals, and the
  `./` prefix pins the path-doubling regression), and import.rs's
  discovery-fencing rationale moved to its use site (266-270) rather than riding
  the shared const. The rider it carried is discharged: `SETTINGS_DOC` bumped
  @07-02 → @07-16 on a live re-verify (coverage_note.rs:70-72); the record is
  annotated. PACKAGING-CHANNELS-REMAINDER's park re-tested — no version tag (era
  tags only), crate 0.1.0 vs npm 0.0.7, release.yml still linux-x64+win32-x64
  with darwin/notarize/bundle absent — still unmet, stays parked. **Sweep:** one
  finding, filed as LOCK-FILENAME-ONE-HOME — the lock-filename half of the same
  consolidation stopped short: drift.rs re-spells `join("lock.toml")` inline at
  five sites (1436/1812/1939/2075/2738) while `LOCK_FILENAME` now holds the one
  home. It escaped the shipped entry twice over — the scope named the four
  modules holding *consts*, which drift.rs is not, and the acceptance
  (`rg '= "lock\.toml"'`) matched const definitions only, so five inline
  literals passed it green. `engineering.md`, "One job, one home"; code
  literals, not comment staleness, so it stands alone. No rider moved: the
  window touched lib.rs/main.rs/install.rs/import.rs/coverage_note.rs, and of
  the "rides X" records only coverage_note.rs's names a line in that set — every
  other line cite in open-questions holds unshifted.
- Queue: 1 pickable (LOCK-FILENAME-ONE-HOME); PACKAGING-CHANNELS-REMAINDER
  parked (John's Apple notarizing + the v0.1 lockstep tag) — condition
  re-tested, still unmet.

Plan continues: no — reconciliation was the last input in the order, and both
cursors now sit at HEAD. Spec cursor copied forward verbatim (delta empty;
39a4833 fully derived). Build takes over: the queue has one pickable entry.
