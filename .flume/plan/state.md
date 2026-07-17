# Plan state

- Spec derived through: 832f015
- Audited through: af2a1f1
- Residue swept through: af2a1f1
- This tick: RECONCILE `15e5924..af2a1f1` — both motions over 6e7b958's ship,
  the window's only commit touching `src/`/`tests/`.
  **Audit: the override shipped as scoped, and it drains the queue's last
  blocker.** Verified on disk, not off the log: `LocalOverride`
  (`src/import.rs`:25-42) is the two-value enum, and `discoverable_paths` (395)
  hangs both waivers off it — `.git_ignore`/`.git_exclude` negated by
  `local_governs` (402-403) and the `WORKSPACE_DIR` skip fenced at 409 — while
  `.git/` and the nested-governed-root stop (412) stay whole for every walk,
  exactly the scoping 0034 names. The class is threaded, not re-derived:
  `local_governs` (256) reads the kind's own column, every read-side caller
  passes `Honored` (`main.rs`:1200/1218, `json_manifest.rs`:395), and only
  `install::discover` (329) withholds it. `cargo test` green on disk.
  **The head entry's gate re-tested and opened.** CHECK-JOINS-INVOCATION-LOCKS
  rested on LOCAL-GOVERNS-OVERRIDES-DISCOVERY alone; the ship commit (af2a1f1)
  removed that entry, so the gate is now `open`. It is the queue's one pickable
  entry.
  **Cites re-stamped, re-read on disk, never carried.** 6e7b958 threaded the
  override through `resolve_kind_units`'s two discovery calls, moving
  `src/main.rs`'s `assemble_lock_family` 1385→1395 — reaching the head entry.
  The `Check` command (106-123) and the admissibility pair (830/869) sit
  unmoved, re-read rather than assumed. `src/drift.rs` (3192/3215, 2760,
  3602-3603, 3647), `src/compose.rs`, and `src/kind.rs` (46-56, 563-568, 735,
  750) are outside the window and re-read where an entry cites them.
  **DIAL-KIND's upstream is now whole.** Its notes claimed two of 0034's three
  derivations had shipped; the third is 6e7b958, so nothing upstream of the dial
  is unbuilt — only the chain's file serialization holds it. Its `src/kind.rs`
  note gains the discovery half: without 6e7b958 the dial's own
  `.temper/dial.toml` sits under the skipped workspace, and the kind's rows
  would derive over an empty set.
  **Sweep: clean.** No second implementation — the override threads through the
  one existing walk (`discover_kind_files` already carried the `kind`), no
  parallel local-discovery path was minted, and `discoverable_paths` stays the
  sole home of both presumptions. Nothing filed. Both parks re-tested and hold:
  the window touches neither `src/graph.rs` nor `.github/` (`git diff` over both
  is empty). The `src/roster.rs`:470 orphan cite still waits for a carrier —
  re-read on disk rather than carried, still 470. One fork record restamped:
  `(settings-local-kind)`'s "can it" half is now built rather than merely ruled,
  so its ship-or-not ruling costs no upstream work.
- Queue: 9 entries, **1 pickable** — CHECK-JOINS-INVOCATION-LOCKS. Six chain
  behind it, serialized on shared files; no entry rests on a fork. Two parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at af2a1f1 with the window's audit and sweep complete. Build
takes over: CHECK-JOINS-INVOCATION-LOCKS is pickable, and with 0034's last
derivation shipped, six entries queue behind it carrying no unbuilt upstream.
