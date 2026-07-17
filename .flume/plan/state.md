# Plan state

- Spec derived through: 832f015
- Audited through: 399d8e3
- Residue swept through: 399d8e3
- This tick: SPEC DELTA — decision 0034 (`f1d97e4..832f015`), derived whole.
  The ruling: emit's codomain is the committed tree, so the local locus is
  **read-side only under any declared format** and "layout-only" retires as
  the wrong fence. **Consequences checklist, every bullet routed.** (1) The
  same-commit corpus edits (pipeline/representation re-cuts, 0030 + 0032
  errata) are the human's own act in 8d794ba — not plan's. (2)
  `(local-locus-toml-face)` **resolves and deletes** — verified already done
  on disk: 8d794ba deleted the record itself. (3) `(settings-local-kind)`
  **reduces to ship-or-not** — likewise already re-cut in that commit; its
  record says so. (4) `local_locus_fault` re-cut → **filed**
  LOCAL-LOCUS-IS-READ-SIDE-ONLY, carrying the errata's coupled landmine in
  the same entry as ruled (`local_document_rows`' silent none,
  `src/main.rs`:1402 — the fence's own doc calls that silence licensed, and
  0034 revokes the licence). (5) The `toml-document` read face → **filed**
  TOML-DOCUMENT-READ-FACE. (6) The errata's walk carve-out → **filed**
  LOCAL-GOVERNS-OVERRIDES-DISCOVERY. (7) "DIAL-KIND and
  CHECK-ANNOUNCES-THE-LOCK-FAMILY unblock" → DIAL-KIND's `dependsOnForks`
  **cleared**, its `src/kind.rs` description, schemaDelta, and notes
  rewritten off the ruling; CHECK-ANNOUNCES inherits through the chain.
  **Verified moot, not filed:** 0034's "emit writes no uncommitted path"
  needs no guard — `emit_program` already skips a local member before any
  content/format branch and books its path owned (`src/drift.rs`:992-995),
  so the codomain holds structurally today; named in
  LOCAL-LOCUS-IS-READ-SIDE-ONLY's notes so build does not mint a second one.
  **Gate repair (closing checklist, not the job):**
  CHECK-JOINS-INVOCATION-LOCKS pointed `blockedBy` at
  CLAUSE-LABEL-IS-AN-ADDRESS, which shipped (6205707) and was dropped by its
  own ship commit — a dangling tag the reference gate rejects; re-gated onto
  the 0034 chain.
- Queue: 12 entries, **1 pickable** — LOCAL-LOCUS-IS-READ-SIDE-ONLY. Nine
  form one `blockedBy` chain behind it (0034's three → join → 0033's four
  widenings → dial → announcement), serialized on shared files; **no entry
  rests on a fork any more** — the board's four survivors have no dependents.
  Two parked on human acts.

Plan continues: yes — post-ship reconciliation of `399d8e3..832f015`
(6d145fa shipped CLAUSE-LABEL-IS-AN-ADDRESS into `src/`; both the audit and
sweep motions are unrun over that window, and only the dangling gate it left
was repaired here).
</content>
