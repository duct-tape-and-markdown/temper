# Plan state

- Spec derived through: f87cc0c
- Audited through: 85fdffd
- Residue swept through: e120e66
- This tick: Residue sweep (job 4), range 6d00c14..HEAD. Jobs 1-3
  reconfirmed quiet first: inbox empty, no refactor captures; no specs/
  commits past f87cc0c; no src/tests/sdk-touching commit past 85fdffd
  (only this tick's own plan commit follows) — job 3 not live. Swept the
  one src-touching commit since the last sweep (726769d,
  KIND-NAME-COLLISION-ADMISSIBILITY): read the full diff — the three
  formerly-duplicated `if builtin_defs.contains_key(...) { continue }`
  dispatch sites (explain/gate/collect_directive_members) now route
  through one `partition_kind_rows`, confirmed no stray copy survives
  (`rg builtin_defs` shows only the consolidated call sites). No new
  residue: no TODO/FIXME/XXX in src/tests/sdk. Both standing accepted
  debts reconfirmed untouched since the last sweep (`git log
  6d00c14..HEAD` empty for both paths): tests/session_start.rs still
  writes `+++`-format stray fixtures (last touch 0735474); sdk/src/
  builtins.ts still cites deleted packages/*/PACKAGE.md files (last
  touch 706139a) — both still ride the next entry touching their file,
  per the citation-staleness carve-out. RETIRE-DEAD-OWN-PATH-SURFACE-
  OVERLAY already operationalizes the one live retirement (surface_
  overlay/Workspace); nothing new to file. Cursor advanced to HEAD.
- Queue: SATISFIES-CLAUSES-RATIONALE-FROM-LOCK open (sole pickable
  entry); RETIRE-DEAD-OWN-PATH-SURFACE-OVERLAY blockedBy it;
  PACKAGING-CHANNELS parked, unchanged. Queue is disjoint.

Plan continues: yes — jobs 1-4 all quiet/current as of this tick; job 5
(quiet closing pass) is the next live input.
