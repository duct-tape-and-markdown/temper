# Plan state

- Spec derived through: f87cc0c
- Audited through: 37a92f0
- Residue swept through: 37a92f0
- This tick: Spec delta (job 2). Read `git show f87cc0c -- specs/` in full —
  the whole new file `specs/process/engineering.md` (32 lines), both
  sections. Checked each bullet against the current pending queue rather
  than re-deriving: "One job, one home"'s extend-before-adding priority and
  its "second implementation is residue" bullet are the `per` cite already
  carried by WINDOWS-VENDOR-SYMLINK-JUNCTION, PATH-NORMALIZER-CONSOLIDATE,
  PLURAL-HELPER-CONSOLIDATE, CLAUSE-FROM-ROW-CONSOLIDATE, and
  SDK-SEAM-ENCODE-CONSOLIDATE; its "test scaffolding is a surface too"
  bullet is TEST-SCAFFOLDING-CONSOLIDATE's cite; "Libraries before
  hand-rolls"'s sanctioned-crate bullet is GLOB-ENGINE-CONSOLIDATE's cite.
  The two remaining bullets (commit-body duty beside a near-duplicate; the
  pinned-semantics exception) are standing build-phase practice, not
  residue to file — nothing to derive from them. Every slice of this delta
  is either an entry's `per` or non-actionable process guidance; no new
  entry, no fork needed. Cursor advances to f87cc0c (HEAD of the spec
  tree); `<spec-delta>` is now empty. Queue and pending.json unchanged.
- Queue: WINDOWS-VENDOR-SYMLINK-JUNCTION (open) — GLOB-ENGINE-CONSOLIDATE
  (open) — CLAUSE-FROM-ROW-CONSOLIDATE (open) — SDK-SEAM-ENCODE-CONSOLIDATE
  (open) — PATH-NORMALIZER-CONSOLIDATE (blockedBy glob) —
  PLURAL-HELPER-CONSOLIDATE (blockedBy glob) — TEST-SCAFFOLDING-CONSOLIDATE
  (blockedBy windows-vendor) — PACKAGING-CHANNELS (parked, condition
  unchanged).

Plan continues: yes — residue sweep (job 4) is next live: job 3 (ship audit)
is quiet (`git log 37a92f0..HEAD -- src/ tests/ sdk/` is empty, no commits
touched those trees), but `Residue swept through` (37a92f0) trails HEAD, so
job 4 fires next tick.
