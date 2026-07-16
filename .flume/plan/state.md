# Plan state

- Spec derived through: abe5d5d
- Audited through: 9862b2e
- Residue swept through: 9862b2e
- This tick: DRAINED the inbox — one line, human-routed at 58efe11 from the
  friction capture `plan-declarations-ts-nul-byte-greps-blind.md` (the capture
  file is already gone; that commit filed it here). Routed to pending as
  **PLACEMENT-KEY-NUL-DELIMITER**, `open`.
  **Re-verified at HEAD, not taken on report.** The note was stamped `observed
  at 9862b2e`, so the premise was diffed forward: 91c288c did touch
  `sdk/src/declarations.ts` (715 → 729 lines, the nested-file child path), and
  the trap survives it — two literal NULs at line 451, and `grep -c
  placementKey` exits 1 on the file that *defines* the symbol.
  **The capture's argument checked, not assumed.** Its claim that the key is
  internal is true on disk: `emit.ts` fills the map (432, importing at 34),
  `declarations.ts` reads it back (492), and both `.temper/lock.toml` and
  `src/builtin_lock.toml` carry zero NUL bytes — nothing round-trips through
  the lock, so the swap is behavior-preserving and `schemaDelta` is none.
  **Scoped to the verified class, not the reported instance.** Swept every
  tracked `.ts`/`.rs`/`.toml`/`.json`/`.md`: `declarations.ts` is the only
  file carrying a NUL, so the one-line entry is the whole class.
  **The `per` is a rule, not a spec section** — the honest owner. No
  `specs/process/engineering.md` section fits (this is neither a duplicate
  surface nor a hand-rolled mechanic). `.claude/rules/sdk.md`, "The engine
  seam" is what the NUL actively defeats: that section instructs "`rg` both
  trees for the seam before concluding the fix is one-sided" and names
  `sdk/src/declarations.ts` as the row-builder home — the rule's own
  instruction silently returns nothing on the file it names. Entry filed at
  queue head, file-disjoint from all six existing entries (mechanically
  checked: no entry touches `declarations.ts` or `emit.ts`), hence `open`.
  Closing checklist: no fork owed — the fix is settled and human-agreed, so
  open-questions is untouched (four open forks, none blocking a queued entry).
  Cursors copied forward verbatim: this tick took the inbox job only and wrote
  no `src/`, `tests/`, or `sdk/` file. **Two gates are knowingly left stale** —
  NESTED-FILE-DISCOVERY names NESTED-FILE-LOCUS and PREDICATE-SELECTION-ALGEBRA
  names EMBEDDED-KIND-CONFORMANCE-DISPATCH, and b745415 shipped both blockers.
  Re-testing them is the reconciliation job's, not the inbox job's; one tick =
  one job.
- Queue: 1 pickable (PLACEMENT-KEY-NUL-DELIMITER); five gated, two of them on
  blockers that have already shipped (next tick's audit re-gates them);
  PACKAGING-CHANNELS-REMAINDER parked (Apple notarizing + v0.1 tag).

Plan continues: yes — post-ship reconciliation. Three commits sit past both
code cursors (5c34ced, 91c288c, b745415: EMBEDDED-KIND-CONFORMANCE-DISPATCH
and NESTED-FILE-LOCUS shipped), so the 9862b2e..b745415 window owes an audit
and a sweep — including the two stale gates named above.
