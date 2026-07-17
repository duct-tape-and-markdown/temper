# Plan state

- Spec derived through: 832f015
- Audited through: 7629fb0
- Residue swept through: 7629fb0
- This tick: RECONCILE `da31f82..7629fb0` — both motions over 990715e's ship,
  the window's only commit touching `src/`/`tests/`/`sdk/`.
  **Audit: the join shipped as scoped, and it unblocks the queue.**
  Verified on disk, not off the log: `assemble_lock_family` (`src/main.rs`:1381)
  joins the committed rows with every local kind's derived `nested`/`satisfies`
  before any consumer reads, and both `gate` (782) and `explain` (478) range
  over the one family. The ship commit (7629fb0) removed
  LOCK-FAMILY-ASSEMBLED-ONCE, leaving TOML-DOCUMENT-READ-FACE's `blockedBy`
  naming a tag no entry carries — **re-tested and opened**: that gate rested on
  `src/main.rs` serialization alone, and the serialization is spent. It is now
  the queue's one pickable entry.
  **Cites re-stamped, re-read on disk, never carried:** 990715e moved
  TOML-DOCUMENT-READ-FACE's second `src/main.rs` pair (`local_document_rows`'s
  match 1412-1418→1422-1428, its decision-rule doc 1392-1399→1405-1411);
  `read_file_unit`'s dispatch (1270-1286) and its doc (1249-1258) are unmoved.
  The entry gains one fact it lacked: the join left `local_document_rows`
  exactly one caller, and `assemble_lock_family` mints no format match of its
  own, so it is not a third site the TOML face must answer. `src/kind.rs`'s
  four addresses re-read and hold — the window never touched the file.
  **One finding older than this window.** CHECK-JOINS-INVOCATION-LOCKS cited
  `engine::admissibility` at 811/850 — true when stamped at 399d8e3, falsified
  by 6d145fa's label lift (+18 → 829/868), and missed by last tick's sweep of
  that very window. Re-stamped against `git show 399d8e3:src/main.rs`, and the
  entry now names `assemble_lock_family` as the seam its join lands at.
  **Sweep: clean.** The commit's named demolition drained whole — the two
  per-call-site commitment branches are gone; the sole surviving
  `commitment != Some(Local)` is the join's own filter, not a per-call-site
  re-decision. Four `read_declarations` consumers still skip the join
  (`schema` 290, `mode_from_lock` 589, `guarded_manifests` 621,
  `coverage_note`:283) — each reads only `kinds`/`clauses`/`assembly`, row
  families a local member never contributes, so none is a second fail-open.
  Nothing filed; no second implementation surfaced. Both parks re-tested and
  hold: the window touches neither `src/graph.rs` nor `.github/`. The
  `src/roster.rs`:470 orphan cite still waits for a carrier, unmoved.
- Queue: 11 entries, **1 pickable** — TOML-DOCUMENT-READ-FACE. Eight chain
  behind it, serialized on shared files; no entry rests on a fork. Two parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at 7629fb0 with the window's audit and sweep complete. Build
takes over: TOML-DOCUMENT-READ-FACE is pickable, and eight entries queue
behind the read face 0034 ratified.
