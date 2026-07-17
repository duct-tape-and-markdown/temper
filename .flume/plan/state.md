# Plan state

- Spec derived through: 832f015
- Audited through: 80685db
- Residue swept through: 80685db
- This tick: RECONCILE `af2a1f1..80685db` — both motions over b753358's ship,
  the window's only commit touching `src/`/`tests/`.
  **Audit: the join shipped as scoped, and it opens the head of the chain.**
  Verified on disk, not off the log: `with_joined_clauses` (`src/compose.rs`:149)
  appends a layer's rows to the host contract and never replaces one, which is
  what makes hardening unbounded and softening structurally impossible;
  `read_layer_clauses` (`main.rs`:1514) refuses a named-but-unreadable layer
  fail-closed, and `joined_kind_admissibility` (1560) catches the clause naming
  a kind no dispatcher would lift. `cargo test` green on disk (0 failed).
  **The head entry's gate re-tested and opened.** TYPE-ACCEPTS-A-SET rested on
  CHECK-JOINS-INVOCATION-LOCKS alone; the ship commit (80685db) removed that
  entry, leaving the gate pointing at a tag no longer in the queue. It is now
  `open`, and the queue's one pickable entry.
  **Cites re-stamped — and my own were the finding.** The window moved
  `src/compose.rs` (+29 at 131), but the deeper miss predates it: 6d145fa
  shifted `src/contract.rs` ~+37 and `src/engine.rs` ~-16 *after* the wave's
  four entries were last stamped at 399d8e3, and no tick since re-read them.
  So four entries carried false addresses for six ships. All re-read on disk at
  80685db: contract.rs (enum 118, `Type` 134-139, `kind` 138,
  `predicate_from_row` 358), engine.rs (`Type` 610, `Optional` 605,
  `ForbiddenKeys` 679, empty-`forbidden_keys` 216, `AllowedChars` 692),
  compose.rs (`clause_from_row` 216; the three `value_type: None` sites 273/301/
  324, all inside `mod tests` — recorded, since the entries called them
  row-build sites without saying they are fixtures). Verified *unmoved* rather
  than assumed: schema.rs (62-67, 110-120, 176), drift.rs (2704, 2760,
  3602-3603, 3647), the oracle (34, 135, 141, 181, 217), reporter.rs (65, 141,
  172), builtins.ts (606-620, 802-818, 887-894, 914), kind.rs. Two builtins.ts
  cites corrected: `required("owner")`/`required("plugins")` were stamped at
  their closing braces (866/872), not their clause heads (861/867).
  **Two entries gained a rider from the window, neither rescoped.**
  `with_joined_clauses` is a second consumer of `clause_from_row`, so
  CLOSED-KEYS-CLAUSE's and SHAPE-PREDICATE's new predicates reach a joined
  layer for free; and `assemble_lock_family` (1457) / `LockFamily` (1427-1436)
  is the one assembled read DIAL-KIND and CHECK-ANNOUNCES-THE-LOCK-FAMILY must
  ride rather than mint a second seam beside.
  **Sweep: clean.** No second implementation — `read_layer_clauses` reuses
  `drift::parse_declarations`, and `with_joined_clauses` reuses
  `clause_from_row`; neither a second lock reader nor a second clause lift was
  minted. Nothing filed. Both parks re-tested and hold: the window touches
  neither `src/graph.rs` nor `.github/` (`git diff` over both is empty). The
  `src/roster.rs`:470 orphan cite still waits for a carrier — re-read on disk,
  still 470. No fork record moved: the audit resolved none.
- Queue: 8 entries, **1 pickable** — TYPE-ACCEPTS-A-SET. Five chain behind it,
  serialized on shared files; no entry rests on a fork. Two parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at 80685db with the window's audit and sweep complete. Build
takes over: TYPE-ACCEPTS-A-SET is pickable, opening 0033's four-widening wave,
with five entries queued behind it carrying no unbuilt upstream.
