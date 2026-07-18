# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: 73c76ca — advanced from 785abd7. Three build commits
  in the window (0282dc7, be8c8e6, 1ef765e, shipped by chore 88da37f)
  touched src/tests/; a fourth commit (73c76ca, a friction-drain, landed
  after this tick's job was chosen) touches none of src/tests/sdk
  (`git diff --stat 88da37f..73c76ca -- src/ tests/ sdk/` empty), so the
  cursor advances through it too. Verified on disk, not just the log:
  `definition` already returns `Option<CustomKind>` (builtin_kind.rs:503),
  the shared `walk_lock_rows` helper backs `read_prior_provenance`/
  `config_stale`/`emit_owned_targets` (drift.rs:1916), `read_utf8` backs
  frontmatter/toml_document/json_manifest's reads (hash.rs), and
  `ANNOUNCEMENT_HEADING` is private (check.rs). `cargo test`/
  `clippy -D warnings`/`fmt --check` all green. Dropped
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE and
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION from pending — both fully shipped
  (the former landed combined with the latter's build session, per
  0282dc7's own commit body). Re-tested three now-stale blockedBy gates
  whose blocker shipped: BUILTIN-KIND-DEFINITIONS-RESULT-COLLAPSE (blocked
  on the just-shipped DEFINITION-RESULT-COLLAPSE), JSON-MANIFEST-TOP-LEVEL-
  OBJECT-PARSE-CONSOLIDATE (blocked on the just-shipped FORMAT-READ-UTF8-
  DECODE-CONSOLIDATE), DRIFT-EMIT-LOCK-PARSE-HOIST (blocked on the
  just-shipped DRIFT-LOCK-ROW-WALK-CONSOLIDATION) — none had its own work
  already done, so all three flip to `open`, with line citations
  re-verified at HEAD (drift.rs and json_manifest.rs both reflowed by the
  window's edits) and `scoped at 88da37f` stamped.
- Residue swept through: 73c76ca — advanced with the audit (small window,
  no split needed). Checked the read+decode consolidation for leftover
  duplicates: the remaining `String::from_utf8` call sites (drift.rs,
  install.rs) decode subprocess stdout, not file reads, so no residue.
  No retirement/demolition named in this window; nothing else fileable.
- Posture swept through: judges next (mid-rotation) — unchanged, copied
  forward verbatim; this window touched no judges-subsystem file
  (engine/graph/dial/coverage/coverage_note/display/reporter).
- This tick: POST-SHIP RECONCILIATION over 785abd7..88da37f (src/tests/),
  the live job at orientation (inbox and spec-delta were both empty at
  that point). Audit + sweep, no split. 2 entries dropped (shipped), 3
  unblocked (gate flipped blockedBy → open, descriptions/citations
  refreshed), 26 untouched. Mid-tick, an interactive session landed
  73c76ca — draining four friction captures (touching docs/ledger.md and
  .flume/prompts/build.md, both outside this phase's writable set) and
  appending two new notes to `.flume/inbox.md`. Verified it does not
  touch src/tests/sdk (folded into the cursor advance above) and does not
  conflict with this tick's pending.json/state.md edits. Its two inbox
  notes are **not drained this tick** — one job per tick, and this tick's
  job was already chosen and underway when they landed; job 1 (inbox)
  now leads next tick's ordering.
- Queue: 29 pending — 3 pickable OPEN (BUILTIN-KIND-DEFINITIONS-RESULT-
  COLLAPSE, JSON-MANIFEST-TOP-LEVEL-OBJECT-PARSE-CONSOLIDATE,
  DRIFT-EMIT-LOCK-PARSE-HOIST — pairwise file-disjoint, re-verified this
  tick), 24 chained blockedBy (all resolve to live tags, re-verified this
  tick), 2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER — reasons unchanged; the inbox's dropped
  hop-cap note was a duplicate of the former, not a change to it). Open
  forks: (multi-harness-projection), (lazy-grounds) unchanged. Refactor
  captures: 0 live (unchanged). Friction: 0 live — the one entry this
  cursor last saw (build-worktree-commits-land-on-main-branch.md) was
  drained by 73c76ca, concurrently with this tick, as a prompt stopgap in
  build.md. Inbox: **2 notes, not yet drained** (a ruled emit --into
  cwd-corruption defect, and a discharge/drop note for two other drained
  captures) — both stamped `observed at 88da37f`. Disjointness re-checked:
  no two OPEN entries share a file; 29 unique tags, no duplicates.

Plan continues: yes — job 1 is live (`.flume/inbox.md` carries 2 undrained
notes) and outranks everything else in the ordering; the posture sweep is
also still mid-rotation (judges next) independent of it.
