# Plan state

- Spec derived through: 832f015
- Audited through: a2e48aa
- Residue swept through: a2e48aa
- This tick: RECONCILE `0c3cbcb..a2e48aa` — one build commit (SHAPE-PREDICATE,
  0927979) touching `src/`, `tests/`, and `sdk/`. Both motions ran; both
  cursors advance.
  **Audit.** Verified on disk, not from the log. `Predicate::Shape`
  (`contract.rs`:241) with its closed `Shape` enum, `from_name` decode, the
  `drift.rs` `shape` column (2768), the `engine.rs` judge (828), and
  `tests/shape_predicate.rs` all present — SHAPE-PREDICATE is genuinely shipped
  and was already dropped from the queue. No pending entry's work is done.
  Every gate re-tested against disk rather than carried: CHECK-ANNOUNCES still
  chains behind DIAL-KIND on `src/main.rs`; IMPORT-HOP-CAP-CITE's park holds —
  nothing ruled the hop semantics and `graph.rs`:59 still reads 5;
  PACKAGING's park holds on every clause — `git tag -l` carries the four era
  tags and no version tag, crate 0.1.0 vs npm 0.0.7, `release.yml`:7-9 states
  the deferral verbatim, and `git diff 0c3cbcb..a2e48aa -- .github/` is empty.
  **The two SDK cites this tick owed are re-stamped**, which is the whole
  reason last tick deferred them here rather than smuggling them into a drain:
  `sdk/src/builtins.ts` is now 1421 lines with the two `shape` clauses at 1091
  and 1127 and the marketplace header rewritten at 960 — and **nothing DIAL-KIND
  needs moved**, since its cite is the subpath rule the file states at 9;
  `sdk/src/index.ts` holds `closedKeys` at 29 and `shape` at 50 as read.
  DIAL-KIND's staleness note is gone rather than re-worded — the entry now
  reads as scoped, because it is.
  **Sweep — one gap filed, ADDRESSED-FIELD-FENCE-EXHAUSTIVE.** The window's
  own diff is the evidence: SHAPE-PREDICATE hand-added `Predicate::Shape` to
  **three** parallel matches answering one relation — which predicates carry a
  field. Two are exhaustive methods on `Predicate` (`target` 619,
  `documented_field` 663) and would have failed the build if missed; the third,
  `engine.rs`'s `addressed_field` (197), ends `_ => None` (210) and would not.
  That wildcard sits on the admissibility path — `unaddressable` (176) reads it
  to refuse a clause whose field leaves the declared RFC 9535 subset, and its
  own doc states the stake: "a bound that is not enforced is not a bound". A
  forgotten arm there is silently admitted, which is the failure the fence
  exists to prevent. `addressed_field` is also the newest of the three — added
  at aaf70f1 as a free function beside two existing methods, naming neither,
  which `engineering.md` requires of a new surface beside a near-duplicate.
  **Filed honestly as structural, not live**: every current predicate is listed
  correctly, so nothing is broken today. The entry forbids the tempting merge
  into `documented_field` (identical arm lists, distinct intents — collapsing
  them conflates two facts free to diverge) and fences `bodyless`/`judgeless`
  out of scope as opt-in lists rather than duplicates.
  **The fork board moved without a new fork.** `(source-union-predicate)`
  became what it predicted: the skill's two holds retired, so the marketplace's
  `source` union is now the **last hold anywhere in the provider face** —
  verified as the sole surviving "pending a vocabulary addition" sentence
  (`builtins.ts`:960). It stays OPEN: a vocabulary addition is a deliberate
  language change, never plan's to derive. Recorded that it now has no sibling
  hold to ride a wave beside — it is ratified deliberately or it stands.
  The ride-only cite record paid out a **third** time and shrank again:
  SHAPE-PREDICATE carried the marketplace-header cite it was given and landed
  it, never becoming an entry. `src/roster.rs`:473 remains the class's last
  orphan, re-read at a2e48aa; no queued entry opens that file, so it waits.
- Queue: 6 entries, **3 pickable and disjoint** —
  SCHEMA-DOCS-CHANNEL-ACCUMULATES (`src/schema.rs`, a live defect in a shipped
  channel), ADDRESSED-FIELD-FENCE-EXHAUSTIVE (`src/engine.rs`), DIAL-KIND.
  CHECK-ANNOUNCES chains behind the dial on `src/main.rs`; no entry rests on a
  fork. Two parked.

Plan continues: no — every input is serviced. Inbox empty, no refactor
captures, spec delta empty (cursor at 832f015 with no `specs/` commit past it),
and `0c3cbcb..a2e48aa` is reconciled on both motions with both cursors
advanced. Build takes over: three pickable, disjoint entries.
