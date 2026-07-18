# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: c1b0f51 — advanced from 4e46eac.
- Residue swept through: c1b0f51 — advanced from 4e46eac.
- Posture swept through: fe3ff3f — verbs ticked last cycle and closed the
  rotation pass. Stale as of this tick: the forward window fe3ff3f..c1b0f51
  touches src/ and tests/, so a fresh cycle re-arms next tick.
- This tick: POST-SHIP RECONCILIATION — window 4e46eac..c1b0f51 (three
  build commits: dcdfbff, 0062150, 112b188), shipped as c1b0f51
  (KIND-DECLARED-FIELDS-EXHAUSTIVE-MATCH, MAIN-READ-FILE-UNIT-FORMAT-
  EXHAUSTIVE-MATCH, DRIFT-EMIT-LOCK-PARSE-HOIST). Audit: all three ship
  commits read on disk and verified against their entries' acceptance
  (kind.rs's declared_fields now pipe-groups the six non-Field Primitive
  variants with an inline test pinning it; main.rs's read_file_unit names
  `Some(Format::YamlFrontmatter) | None` instead of a wildcard; drift.rs's
  emit() reads/parses lock.toml once via a new read_lock_document, feeding
  both read_prior_provenance_from_doc and declarations_from_doc, pinned by
  thread-local lock_read_count/lock_parse_count and
  tests/check_cost.rs::emit_lock_parse_is_hoisted_and_pinned_once_per_run).
  All three already dropped from pending.json by the ship commit. Metrics
  glanced (turns 19/34/73) — no bail/revert markers in the window.
  Re-tested stale gate: PLACEMENT-MODULE-EXTRACTION was blockedBy
  DRIFT-EMIT-LOCK-PARSE-HOIST — shipped, unblocked to `open`. Its cites
  were stale across several ship windows since its f404e48 scoping, not
  just this one (62559ef and 516f8f6 shifted install.rs by -1 line before
  this window opened) — fully re-verified against HEAD and corrected:
  install.rs's placement_lines/is_placement_comment/project_modeline/
  project_note/project_banner/test cites, and drift.rs's emit_one call
  site (2034→2154) and matches_projection doc pointer (2603→2706).
  Sweep: diffed the window against the three commits already read for the
  audit. drift.rs's split checked for duplicate-job residue: the old bare
  `read_prior_provenance` was fully removed (renamed to `_from_doc`,
  zero remaining bare references), `walk_lock_rows` itself survives with
  its own real consumers (config_stale, emit_owned_targets) untouched by
  this hoist — no second implementation. One comment staleness found:
  drift.rs's `RawLockRow` doc comment (1947-1952) still intra-doc-links
  `[read_prior_provenance]`, now renamed. Per the ride-only rule
  (open-questions.md, "One stale cite") this rides the next entry that
  names it rather than filing standalone — recorded as a second tracked
  orphan there, alongside the standing json_splice.rs one.
- Queue: 28 pending (-0 shipped this tick, +0 filed) — 1 pickable OPEN
  (PLACEMENT-MODULE-EXTRACTION, newly unblocked), 25 chained blockedBy
  (all resolving to live tags), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks
  unchanged: (multi-harness-projection), (lazy-grounds), neither touched.
  Refactor captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep re-arms (fe3ff3f..c1b0f51 touched
src/ and tests/, closing the quiet window the last rotation left); next
tick sweeps the first subsystem of a fresh cycle.
