# Plan state

- Spec derived through: 4adb1fb
- Audited through: 2f44341
- Residue swept through: 2f44341
- Posture swept through: foundation done — model next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  2f44341..HEAD -- src/ sdk/src/ tests/` empty (no post-ship window to
  audit or sweep — the sole intervening commit, fb49bc4, is plan-only).
  Job 4 was live: `Posture swept through: ad66e46` named a rotation-closed
  sha, but `ad66e46..HEAD -- src/ sdk/src/ tests/` touched src/ (the four
  build commits the prior tick's post-ship audit already reconciled), so
  the condition for opening a new cycle held — swept `foundation`
  (architecture.md codemap: `check`, `extract`, `hash`, `address`, `tap`,
  `json_splice`), the new cycle's first subsystem. Read all six files;
  `hash.rs`/`json_splice.rs`/`check.rs` clean on every engineering.md
  lens — every `pub`/`pub(crate)` item grep-verified to a real external
  caller (`sha256_hex` from drift/frontmatter/json_manifest/toml_document;
  every `json_splice` fn from install.rs/json_manifest.rs;
  `Diagnostic`/`Announcement`/`Severity`/`render`/`any_error` all
  consumed outside check.rs). `extract.rs`'s upward imports into
  drift/kind are the already-tracked architecture.md debt
  (EXTRACT-FOUNDATION-BOUNDARY-RESTORE, chained) — not re-filed. Two
  fresh zero-consumer exports surfaced, both verified on disk (symbol,
  line, grep for consumers) before filing:
  - `tap::LOG_FILENAME` (26) — zero consumer anywhere in src/, tests/,
    sdk/, not even its own tests: `tests/tap.rs` (66,99,173,198) each
    hardcode the literal `"tap.jsonl"` rather than importing the const →
    filed `TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE`.
  - `address::FieldPath::spelling` (83-85) — pub accessor called only
    from this file's own `mod tests` (365,373); every internal use reads
    the private `spelling` field directly → filed
    `ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE`, same
    export-earns-its-consumer bar the queue's three existing
    visibility-narrow entries already establish as precedent.
  Both new entries are single-file, `open`, and share no path with any
  other queued entry (tap.rs and address.rs touched nowhere else in the
  queue) — disjoint per pending-entry.md.
- Queue: 21 pending — 7 pickable OPEN (READ-EXPLAIN-STRAND-VISIBILITY-
  NARROW, ENGINE-SELECTOR-LABEL-ZERO-CONSUMER-PRUNE,
  REPORTER-SEVERITY-WORD-CONSOLIDATE, BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-
  PRUNE, TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE (new),
  ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE (new),
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; all disjoint files), 12 chained
  blockedBy (DRIFT-LOCK-ROW-WALK-CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-
  HOIST → PLACEMENT-MODULE-EXTRACTION → {EXTRACT-FOUNDATION-BOUNDARY-
  RESTORE → {KIND-ZERO-CONSUMER-EXPORTS-PRUNE →
  CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH →
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP, DRIFT-SOURCE-DEP-PARSE-HOIST →
  GRAPH-RESOLVED-EDGE-WALK-CONSOLIDATE → GATE-KIND-UNITS-DOUBLE-RESOLVE-
  HOIST}, INSTALL-PROJECTION-MATCH-CONSOLIDATE → INSTALL-GUARD-MANIFEST-
  MESSAGE-PRUNE → INSTALL-PLACEMENT-KIND-ENUM}), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — posture sweep resumes at `model`
(`kind`/`contract`/`compose`/`schema`/`roster`), the roster's next
subsystem, once nothing above it is live.
