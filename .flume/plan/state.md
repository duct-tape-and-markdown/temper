# Plan state

- Spec derived through: cd7135b
- Audited through: a112dbe
- Residue swept through: 91fb466
- This tick: Spec delta (job 2) — routed 469e508 (0016 "install installs the
  tool, whole") and e48e33b (0017 "no middles"). 0016: the shallow lift's
  `file()`-verbatim own-path scaffold is confirmed still live on disk
  (src/install.rs's `member_module_source`, src/drift.rs's
  `resolves_to_own_path`/`Projection.own_path`, src/import.rs's
  `RollupEntry.own_path` lock column) — filed as a two-entry blockedBy chain:
  INSTALL-WHOLE-CONVERSION (scaffold hoists every present frontmatter field
  to a typed property, prose moves module-side) then RETIRE-OWN-PATH-
  MACHINERY (deletes the now-structurally-dead own_path detection/column/
  guard-exemption once nothing emits it). Both are gate-sized; splitting
  avoids one commit that both changes scaffold output and deletes the
  drift/lock machinery reading it. `.temper/`'s own reconversion (the
  decision's third consequence) is outside build's writable fence
  (chain.ts BUILD_WRITABLE_PATHS excludes `.temper/**`) — human/ratification
  territory, not filed. 0017: fully routed already — its five prose recuts
  are reflected in the corpus text itself; its two named code consequences
  (inert Template columns, legacy import_hash fallback) were filed last tick
  as RETIRE-DEAD-DECLARED-SURFACE; its `sdk/src/assembly.ts` residual-
  settings evidence dies with the already-open (manifest-authoring-surface)
  fork. Both new entries chain behind SDK-VERSION-LOCKSTEP (blockedBy) since
  all three declare `src/install.rs`/`tests/install.rs` — disjointness rule.
- Queue: 6 — CI-DOCUMENTED-TWO-LINE-JOB, SDK-VERSION-LOCKSTEP,
  RETIRE-DEAD-DECLARED-SURFACE (open, pickable, disjoint); INSTALL-WHOLE-
  CONVERSION → RETIRE-OWN-PATH-MACHINERY (chained behind SDK-VERSION-
  LOCKSTEP); PACKAGING-CHANNELS (parked).

Plan continues: yes — job 3 (ship audit) is not live this tick (no commits
past `Audited through: a112dbe` touch src/, tests/, or sdk/ — `git log
--oneline a112dbe..HEAD -- src/ tests/ sdk/` is empty), so job 4 (residue
sweep) is next: `Residue swept through: 91fb466` trails HEAD with jobs 1-3
quiet.
