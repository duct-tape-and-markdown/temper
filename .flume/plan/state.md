# Plan state

- Spec derived through: f4189c3
- Audited through: 75798a5
- Residue swept through: caed5cf
- This tick: Residue sweep 2b82ebd..HEAD. Only src change in range is
  PATH-SEP-NORMALIZE (402e5cc) — `to_lock_path`/`add_provenance`/
  `relative_to_workspace` all `/`-normalize; corpus-consistent, no residue.
  f4189c3's `embed`->`include` spec rider has NO code residue — every
  surviving `embed` in src/tests is the sanctioned "engine embeds a built-in
  lock" sense (distribution.md), not the retired include-verb. One residue
  class carried forward: the drift.rs "five families" docstrings (:262/:269)
  still lag the 6th (mention) declaration family — pure comment staleness, the
  residue-sweep exception. Previously non-routable (no entry opened drift.rs);
  now rides EMIT-REAP-ORPHANS (head of the drift.rs chain), attached to its
  drift.rs edit as a cut-on-contact rider. Also fixed EMIT-REAP-ORPHANS's stale
  "0008-0011 unrouted" note (f4189c3 fully routed). install.rs:951-953 cwd
  doc-comment staleness already routed by MODULE-RELATIVE-PATHS. Spec cursor
  f4189c3 + Audited 75798a5 copied forward verbatim (jobs 2/3 quiet: no delta,
  no src past 75798a5). session_start KIND.md/PACKAGE.md `+++` fixture debt
  UNCHANGED (session-start path untouched this range).
- Queue: 7 — three disjoint open fronts: EMIT-REAP-ORPHANS (drift.rs, +comment
  rider), UNCLAIMED-ENTRY-ADVISORY (coverage_note.rs), GUARD-OWNPATH
  (install.rs+main.rs). EMIT-LF-NORMALIZE blockedBy EMIT-REAP (drift.rs),
  SETTINGS-FORMAT-PRESERVING blockedBy GUARD-OWNPATH, MODULE-RELATIVE-PATHS
  blockedBy it (install.rs chain tail; +SDK), PACKAGING-CHANNELS parked.

Plan continues: no — every input current (inbox empty, no spec delta past
f4189c3, no src past 75798a5, residue swept to HEAD). Pickable open entries
exist (EMIT-REAP-ORPHANS, UNCLAIMED-ENTRY-ADVISORY, GUARD-OWNPATH) — build
takes over.
