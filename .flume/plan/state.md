# Plan state

- Spec derived through: cd7135b
- Audited through: 37a92f0
- Residue swept through: 37a92f0
- This tick: Inbox (job 1). `<inbox>` was empty but `<refactor-captures>`
  held six live session captures (the harness's own listing command failed
  this tick; verified directly with `ls`/`cat`). Re-verified each claimed
  surface against HEAD (grep + read, not the capture alone) — all six still
  stand unchanged since their `observed at 0ccba8d` stamp (confirmed via
  `git log 0ccba8d..HEAD` touching zero of the named files). Routed each
  into a pending entry citing engineering.md ("One job, one home" /
  "Libraries before hand-rolls"): GLOB-ENGINE-CONSOLIDATE, CLAUSE-FROM-ROW-
  CONSOLIDATE, and SDK-SEAM-ENCODE-CONSOLIDATE (open); PATH-NORMALIZER-
  CONSOLIDATE and PLURAL-HELPER-CONSOLIDATE (blockedBy GLOB-ENGINE-
  CONSOLIDATE — both share edited files with it); TEST-SCAFFOLDING-
  CONSOLIDATE (blockedBy WINDOWS-VENDOR-SYMLINK-JUNCTION — shares
  tests/common/mod.rs and three test files with it; folded the previously
  unfiled `fixture()` duplication into it as the same job, found
  re-verifying). Close reads changed two captures' framing: clause-from-row
  are disjoint predicate subsets, not overlapping twins; plural/tmpdir
  found one extra duplicate site each (install.rs's two inline closures;
  no extra tmpdir site). Deleted all six capture files.
- Queue: WINDOWS-VENDOR-SYMLINK-JUNCTION (open) — GLOB-ENGINE-CONSOLIDATE
  (open) — CLAUSE-FROM-ROW-CONSOLIDATE (open) — SDK-SEAM-ENCODE-CONSOLIDATE
  (open) — PATH-NORMALIZER-CONSOLIDATE (blockedBy glob) —
  PLURAL-HELPER-CONSOLIDATE (blockedBy glob) — TEST-SCAFFOLDING-CONSOLIDATE
  (blockedBy windows-vendor) — PACKAGING-CHANNELS (parked, condition
  unchanged).

Plan continues: yes — spec delta (job 2) is still live: f87cc0c
(specs/process/engineering.md, "engineering shape") landed past the
`cd7135b` cursor and remains unrouted; this tick only cited it as a `per`
target, it did not derive the delta's full slice into entries or forks, so
the cursor does not advance.
