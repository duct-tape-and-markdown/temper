# Plan state

- Spec derived through: 832f015
- Audited through: 832f015
- Residue swept through: 832f015
- This tick: INBOX (refactor channel) — drained the one live capture,
  `build-local-rows-miss-the-embedded-corpus.md`, filed by
  LOCAL-LOCUS-IS-READ-SIDE-ONLY's own build tick, observed at 732dbcd. All
  three claims re-verified on disk at HEAD rather than trusted from the
  capture's line numbers (bce89b7 moved them): (1) `embedded_features_by_kind`
  (`src/main.rs`:1555) folds `declarations.nested_members` alone — the
  committed lock — and both call sites (523, 913) pass only `&declarations`;
  (2) `kind_features` (1364) lands `local_document_rows`' `nested` half only in
  the host's own `Features` (1384), so a clause over an embedded kind selects
  **zero** members for a local host, while `satisfies` (1228-1233) has no such
  gap; (3) the test that should pin this (`tests/local_locus.rs`:285) is
  false-green — `common::clause("count", "error")` (297) declares a severity
  outside the closed `required`/`advisory` vocabulary (`src/compose.rs`:214-219,
  `severity_from_label`), so the lock refuses at load and `!ok` (305) passes on
  the refusal while the count clause never fires. Filed whole as
  **LOCK-FAMILY-ASSEMBLED-ONCE** per `engineering.md`, "One job, one home": two
  row sources, one job, never joined — and the fail-open is what the split buys.
  Capture deleted; git is the archive.
  **Gates re-tested (closing checklist, not the audit motion):**
  TOML-DOCUMENT-READ-FACE's `blockedBy` dangled — LOCAL-LOCUS-IS-READ-SIDE-ONLY
  shipped at bce89b7 and was dropped by its own ship commit. Retirement verified
  on disk, not from the log: `local_locus_fault` (`src/kind.rs`:743) now fences
  the `governs`-less locus alone, the layout-only arm gone. Re-gated behind
  LOCK-FAMILY-ASSEMBLED-ONCE, which subsumes the `local_document_rows` format
  match (1412-1418) that face must answer — engineering.md's own
  subsume-before-extend priority. DIAL-KIND's `src/kind.rs` description
  re-stamped for the same ship. Both parks re-read and hold.
- Queue: 12 entries, **1 pickable** — LOCK-FAMILY-ASSEMBLED-ONCE. Nine chain
  behind it, serialized on shared files; no entry rests on a fork. Two parked.

Plan continues: yes — post-ship reconciliation of `832f015..da31f82` (bce89b7's
ship across `src/kind.rs`, `src/main.rs`, `src/drift.rs`, `sdk/`,
`tests/local_locus.rs`) is untaken. This tick verified only what the capture and
the dangling gate touched; the window's audit and residue sweep are both still
owed, so all three cursors are copied forward unmoved.
