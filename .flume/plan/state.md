# Plan state

- Spec derived through: f4189c3
- Audited through: 75798a5
- Residue swept through: 2b82ebd
- This tick: Ship audit 2b82ebd..75798a5. Only code change in range is
  PATH-SEP-NORMALIZE (402e5cc, merge c4116a4) — verified on disk
  (`install.rs:967` `replace('\\', "/")`, lock `to_lock_path`:293, test
  `:1323`); already dropped from the queue. Its dependent gate cleared:
  GUARD-OWNPATH flipped blockedBy PATH-SEP-NORMALIZE -> open, fully rewritten
  (drifted line refs corrected post-churn; mechanism recut to bind the runtime
  guard to the existing pub `drift::emit_owned_targets`:874 — which already
  excludes own_path rows — dropping the spurious kind.rs edit and any drift.rs
  edit, so it stays disjoint from the open EMIT-* drift.rs chain). session_start
  KIND.md/PACKAGE.md `+++` fixture debt UNCHANGED — session-start path untouched
  this range; accepted debt copied forward. Spec cursor f4189c3 (no delta past
  it); Residue cursor 2b82ebd copied forward verbatim (job 4 not serviced).
- Queue: 7 — GUARD-OWNPATH now open (install.rs+main.rs, head of the install
  chain), EMIT-REAP-ORPHANS open (drift.rs), UNCLAIMED-ENTRY-ADVISORY open
  (coverage_note.rs): three disjoint open fronts. EMIT-LF-NORMALIZE blockedBy
  EMIT-REAP (drift.rs), SETTINGS-FORMAT-PRESERVING blockedBy GUARD-OWNPATH,
  MODULE-RELATIVE-PATHS blockedBy it (install.rs chain tail; +SDK),
  PACKAGING-CHANNELS parked (human release creds).

Plan continues: yes — residue sweep (Residue swept through 2b82ebd trails HEAD;
2b82ebd..HEAD carries PATH-SEP-NORMALIZE's src changes + plan commits to sweep
for retired vocab/symbols the corpus no longer sanctions).
