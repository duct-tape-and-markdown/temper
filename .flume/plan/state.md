# Plan state

- Spec derived through: 6b80e24
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: absent — rotation never initialized, owed once jobs
  1-3 are quiet
- This tick: INBOX/REFACTOR-CAPTURES. Inbox empty; but a live refactor
  capture existed on disk — `.flume/refactor/plan-drift-lock-row-walk-
  triplication.md`, filed by the prior tick's side-read of src/drift.rs
  (committed at 7132213, after that tick's own job-order check had already
  passed). That capture is this tick's first live input, ahead of spec
  delta (empty, 6b80e24..HEAD touches no specs/) and post-ship
  reconciliation (empty, 60faee0..HEAD touches no src/tests/sdk).
  Re-verified the capture's claim on disk before draining: `src/drift.rs`
  unchanged since observed (b8fc7ca..HEAD touches no drift.rs), and all
  three cited fns read verbatim as claimed — `read_prior_provenance`
  (1918-1949), `config_stale` (2342-2386), `emit_owned_targets`
  (2614-2643) each independently open+parse `lock.toml` and walk
  `(kind, item)`/`as_array_of_tables()`/`row.get(..).and_then(Item::as_str)`.
  Consumer check confirmed no ripple: only `src/install.rs`, `src/main.rs`,
  `tests/install.rs` call the three public fns, never the internals — grep
  shows no other call sites. Drained into DRIFT-LOCK-ROW-WALK-CONSOLIDATION
  (per engineering.md, "One job, one home"), the capture file deleted.
- Queue: 4 pending — 2 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; disjoint files, no serialization
  needed), 2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks: (multi-harness-projection),
  (lazy-grounds).

Plan continues: yes — posture sweep is owed next: jobs 1-3 are now quiet
(inbox and refactor-captures both empty, spec cursor already covers HEAD
with no new specs/ commits, audit window 60faee0..HEAD still touches no
src/tests/sdk) and `Posture swept through:` is still absent — rotation
starts at its first subsystem, `drift`.
