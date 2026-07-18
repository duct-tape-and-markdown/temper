# Plan state

- Spec derived through: 2d66fc9 — advanced from 53df138. The only
  intervening specs/ commit, decision 0041 ("when joins the vocabulary"),
  is now fully routed.
- Audited through: c1b0f51 — advanced from 4e46eac.
- Residue swept through: c1b0f51 — advanced from 4e46eac.
- Posture swept through: pipeline next (mid-rotation) — formats read and
  swept this tick (touched: json_manifest.rs by 8f96918's top-level-
  object-parse consolidation) — quiet-on-clean.
- This tick: POSTURE SWEEP — formats subsystem (`src/frontmatter.rs`,
  `src/document.rs`, `src/json_manifest.rs`, `src/toml_document.rs`),
  touched by 8f96918 (ship-audited two ticks ago). All four files read
  whole against every `specs/process/engineering.md` section plus the
  sweep's cohesion/dead-plumbing lenses. Zero-consumer check: every
  checked `pub`/`pub(crate)` item `rg`-confirmed a consumer outside its
  own module — `fold_file_id` (main.rs), `closing_delimiter` (install.rs,
  three call sites), `item_to_json` (toml_document.rs), `Satisfies::new`
  (main.rs, read.rs, tests), `write_document`/`write_manifest`/
  `CollectionSegment` (drift.rs, bundle.rs, tests), `DocumentMember`'s
  read/parse/to_unit and `Manifest`'s read/parse/read_kind (main.rs,
  drift.rs, install.rs, coverage_note.rs, tests), `toml_document::read`
  (main.rs). Exhaustive-match check: all three `UnitShape` matches
  (frontmatter.rs:186-223, json_manifest.rs:168-190, toml_document.rs:
  53-64) name every variant explicitly, no `_` arm; the one bare `_ =>`
  in the subsystem (frontmatter.rs:307, inside `parse_frontmatter`) is
  over `gray_matter`'s own `Pod` enum, not a temper-owned shared type —
  out of the section's scope, consistent with last tick's closed-
  vocabulary-parser exclusion. No duplicate matcher/normalizer beyond
  the one already consolidated by 8f96918 (`parse_top_level_object`,
  shared by `Manifest::parse`/`DocumentMember::parse`). One stale-cite
  orphan surfaced (not a pending entry — ride-only per the fork-lifecycle
  rule): `document.rs`'s `item_to_json` doc comment cites a
  `json_to_toml_value` function that `664a522` deleted before `6618b47`
  even wrote the citing sentence; recorded as a third live orphan in
  `open-questions.md` alongside the standing `json_splice.rs`/`drift.rs`
  pair, riding whichever entry next opens `document.rs`. Verdict:
  quiet-on-clean, no new pending entry. Checked ahead: pipeline is not
  clean-skippable — `git log 662cf07..HEAD -- src/drift.rs src/import.rs
  src/read.rs src/builtin_lock.rs` shows three touches since its own
  last full sweep (0282dc7's lock-row-walk consolidation, 72daab3's
  harness_relative canonicalization, 112b188's lock.toml parse hoist,
  all in drift.rs) — next tick reads it whole rather than skipping.
- Queue: 28 pending — unchanged (no entry filed, dropped, or edited this
  tick; posture sweep found nothing new beyond the ride-only orphan
  above). 1 pickable OPEN (PLACEMENT-MODULE-EXTRACTION), 25 chained
  blockedBy (all resolving to live tags), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks
  unchanged: (multi-harness-projection), (lazy-grounds), neither
  touched. Refactor captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture sweep is still mid-rotation (pipeline
next, already known touched by 0282dc7/72daab3/112b188) and no commit
past c1b0f51 has landed to re-trigger reconciliation, so the sweep is
the next live input.
