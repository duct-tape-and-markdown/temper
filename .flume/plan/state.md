# Plan state

- Spec derived through: cd7135b
- Audited through: 37a92f0
- Residue swept through: 37a92f0
- This tick: Inbox (job 1). Routed the sole inbox note — the Windows
  `cargo test` compile break (observed at baac085, junction-vs-`symlink_dir`
  correction verified 2026-07-08, both landed via prior `chore(flume)`
  edits to inbox.md, not a plan tick) — into WINDOWS-VENDOR-SYMLINK-JUNCTION
  (open). Re-verified the claimed gap against HEAD: all three unconditional
  `std::os::unix::fs::symlink` call sites (tests/emit.rs:699,
  tests/install.rs:84, tests/builtin_lock_frozen.rs:105) still stand, and
  their `sdk_root`/`ensure_sdk_built` helpers are byte-identical
  triplicates — confirmed by reading each file, not the report alone.
  Scoped the fix to consolidate into tests/common alongside the platform
  branch (engineering.md "One job, one home" / "Test scaffolding is a
  surface too" — the fix touches all three duplicates regardless, so
  unifying them is the sanctioned expansion, not creep) using
  `cmd /C mklink /J` on Windows — no new crate, so no sanctioned-crate-set
  human call is needed. Drained the inbox line.
- Queue: WINDOWS-VENDOR-SYMLINK-JUNCTION (open, next for build) —
  PACKAGING-CHANNELS (parked, condition unchanged).

Plan continues: yes — spec delta (job 2) is still live: f87cc0c
(specs/process/engineering.md, "engineering shape") landed past the
`cd7135b` cursor and remains unrouted; this tick only borrowed one of its
sections as a citation, it did not derive the delta's full slice into
entries or forks, so the cursor does not advance.
