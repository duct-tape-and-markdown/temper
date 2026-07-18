# Plan state

- Spec derived through: 4adb1fb
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: formats done — pipeline next
- This tick: POSTURE SWEEP. Jobs 1-3 quiet: inbox and refactor-captures
  empty; `git log 4adb1fb..HEAD -- specs/` empty (no spec delta); `git log
  60faee0..HEAD -- src/ sdk/src/ tests/` empty (no post-ship window to
  audit or sweep). Job 4 was live — rotation mid-way (`model done —
  formats next`) makes it live regardless of window content. Swept the
  `formats` subsystem (architecture.md codemap: `frontmatter`, `document`,
  `json_manifest`, `toml_document`), reading all four files against every
  engineering.md posture:
  - `frontmatter.rs`'s test-only `builtin_kind` dependency is already
    queued (FRONTMATTER-TEST-SYNTHETIC-KINDS, 0040) — not re-filed.
  - `json_manifest.rs`'s dead `Discovery(#[from] ImportError)` variant is
    already queued (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE) — not re-filed.
  - `toml_document.rs` is clean — read-only face, no residue.
  - `document.rs` yielded a new finding: grep (src/, tests/, sdk/) shows
    its `+++`-fenced `Document`/`DocumentError` type and
    `add_clause`/`add_satisfies`/`add_provenance`/`clauses`/`provenance`/
    `child_table` have zero consumers outside the file's own `mod tests`.
    664a522 ("retire the pre-0016 own-path surface-document mechanism")
    removed every real caller (`surface_overlay`,
    `Member::from_surface`/`to_document`) but left the machinery itself
    standing. `Satisfies` and `item_to_json`/`value_to_json` are the
    file's only live surface (frontmatter.rs, read.rs, kind.rs,
    builtin_kind.rs, main.rs, toml_document.rs). Filed
    DOCUMENT-RETIRED-FENCE-SURFACE-PRUNE (`engineering.md`, "An export
    earns its consumer") — gate open, disjoint from every other entry's
    files (only src/document.rs).
  Rotation advances: `formats` done, `pipeline` next
  (`drift`/`import`/`read`/`builtin_lock`/`placement`).
- Queue: 13 pending — 4 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS, ROSTER-BUILTIN-KIND-
  NARROWING-RELOCATE, DOCUMENT-RETIRED-FENCE-SURFACE-PRUNE; all disjoint
  files), 7 chained blockedBy (DRIFT-LOCK-ROW-WALK-CONSOLIDATION →
  DRIFT-EMIT-LOCK-PARSE-HOIST → PLACEMENT-MODULE-EXTRACTION →
  EXTRACT-FOUNDATION-BOUNDARY-RESTORE → KIND-ZERO-CONSUMER-EXPORTS-PRUNE →
  CONTRACT-DECLARED-KEYS-EXHAUSTIVE-MATCH →
  CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — posture sweep resumes at `pipeline`
(`drift`/`import`/`read`/`builtin_lock`/`placement`), once nothing above
it is live.
