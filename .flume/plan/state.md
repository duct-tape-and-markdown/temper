# Plan state

- Spec derived through: 5945405
- Audited through: 2b82ebd
- Residue swept through: 2b82ebd
- This tick: Spec delta f4189c3, fifth slice — 0011 unclaimed-entry advisory
  → UNCLAIMED-ENTRY-ADVISORY (open, collision-free in src/coverage_note.rs).
  coverage_note's check:103 iterates only the curated KNOWN_SURFACES:64 list, so
  a stray `.claude/` entry on no kind's locus and no known surface (a bogus
  `.clauignore`) is examined by nothing and passes silent; add an ignore-honoring
  scan of the actual `.claude/` entries emitting a warn `coverage.unclaimed-entry`
  per unclaimed stray (distinct from `coverage.unmodeled-surface` — no
  double-report). Uses the existing check(root,kinds,member_counts) inputs — no
  main.rs plumbing. 0011's command/agent kinds + `user-invoked` registration value
  stay unrouted (derivable-later slices, hold the cursor); hook/settings JSON kinds
  route via the open (json-projection-format) fork — f4189c3 NOT fully routed, spec
  cursor stays 5945405. Audited/Residue cursors copied forward verbatim (not
  serviced this tick).
- Queue: 7 — EMIT-REAP-ORPHANS open (drift.rs), EMIT-LF-NORMALIZE blockedBy
  EMIT-REAP-ORPHANS (same drift.rs+emit.rs), UNCLAIMED-ENTRY-ADVISORY open
  (coverage_note.rs, independent), GUARD-OWNPATH blockedBy PATH-SEP-NORMALIZE
  (blocker SHIPPED 2efd00b — ship audit unblocks it), SETTINGS-FORMAT-PRESERVING
  blockedBy GUARD-OWNPATH, MODULE-RELATIVE-PATHS blockedBy SETTINGS-FORMAT-PRESERVING
  (install.rs chain tail; also SDK prose.ts+emit.ts+emit.test.ts), PACKAGING-CHANNELS
  parked (human release creds). Chains: drift.rs / coverage_note.rs / install.rs+kind.rs
  (+SDK) / package.json.

Plan continues: yes — spec delta still live (f4189c3 0011 command/agent kinds +
user-invoked value unrouted); ship audit also trailing (PATH-SEP-NORMALIZE shipped
past Audited-through 2b82ebd).
