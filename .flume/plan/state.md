# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8 — unchanged; `git log d40a9f8..HEAD -- src/
  tests/ sdk/` is empty (all seven commits since — 5af93d9, 3871eba,
  9e197d6, 7a5f86c, 69e7571, 662cf07, 5d7e712 — are plan-only).
- Residue swept through: d40a9f8 — unchanged, same empty window.
- Posture swept through: 662cf07 — unchanged, mid-rotation. This tick
  swept `provider`; foundation/model/formats/pipeline/judges/provider
  stay ticked, next: verbs (the rotation's last subsystem — closing it
  next tick stamps the cursor for the first time this cycle).
- This tick: POSTURE SWEEP — provider subsystem (builtin.rs,
  builtin_kind.rs, both read whole: 71 and ~1100 lines) swept against
  every section of `specs/process/engineering.md`, cross-checked
  against the queue to avoid re-flagging in-flight work (the two open
  BUILTIN-KIND-DEFINITION(S)-RESULT-COLLAPSE entries already own
  `definition`/`definitions`'s Result wrapper; EXTRACT-FOUNDATION-
  BOUNDARY-RESTORE already owns the `nested_members_from_rows`
  requalification inside `features` at builtin_kind.rs:566-569).
  Quiet — no new finding:
  - Every pub/pub(crate) export (`builtin::contract`, `builtin::
    contracts`, `builtin_kind::definition`, `definitions`,
    `skill_features`, `rule_features`, `features`) grep-verified to a
    live outside-module consumer across src/main.rs, src/install.rs,
    src/compose.rs, and a dozen-plus test files.
  - No `_ =>`/wildcard arm over a shared enum in either file (grep
    empty) — nothing for "A shared concept is one type" to catch.
  - `builtin_lock::declarations()` is `LazyLock`-cached (parsed once);
    `builtin::contract_for_kind`'s per-kind clause filter and
    `builtin_kind::all_kinds()`'s per-call `Vec<CustomKind>`
    construction are both fixed-size built-in-roster work (14 kinds, a
    few dozen clauses) — constant, never scaling with the harness
    under check, so neither is a "Cost scale is hoisted" candidate.
  - The `dial` kind's placement inside builtin_kind.rs despite its own
    doc comment naming it "not a claude-code kind" is a documented,
    deliberate exception (the constructing fn's own doc comment states
    why) — not undocumented residue, and the sweep never files against
    a stated asymmetry.
  - builtins.md's "Twelve kinds ship" reconciles exactly against
    `all_kinds()`'s 14-entry roster (12 + `supporting-doc` + `dial`,
    both explicitly excluded from that enumeration by their own spec/
    doc-comment text) — no drift between the spec count and the code
    roster.
- Queue: 34 pending, unchanged — quiet tick, pending.json untouched. 6
  pickable OPEN, 26 chained blockedBy, 2 parked on human action. Open
  forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: 0 live. Inbox empty.

Plan continues: yes — posture sweep resumes at `verbs` (main.rs,
install.rs, bundle.rs, lib.rs, test_support.rs), the rotation's last
subsystem before this cycle closes.
