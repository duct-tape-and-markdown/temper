# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8 — unchanged; `git log d40a9f8..HEAD -- src/
  tests/ sdk/` is empty (all three commits since — 5af93d9, 3871eba,
  9e197d6 — are plan-only).
- Residue swept through: d40a9f8 — unchanged, same empty window.
- Posture swept through: 5af93d9 — copied forward, mid-rotation.
  foundation ticked (3871eba, quiet), model ticked (9e197d6, quiet),
  formats ticked this tick (below) — next: pipeline.
- This tick: POSTURE SWEEP — job 4, formats subsystem (frontmatter/
  document/json_manifest/toml_document), flagged not clean-skippable
  by last tick's note (87221b2 widened json_manifest.rs's and
  toml_document.rs's UnitShape matches since formats' own last sweep,
  07a9c04). Read all four files whole against engineering.md's lenses
  plus architecture.md's invariants. The two already-open entries from
  formats' last sweep (JSON-MANIFEST-READ-DECODE-CONSOLIDATE,
  TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE) are untouched by 87221b2 and
  still resolve — re-verification is job 3's, not re-done here. Two new
  findings, both filed as `.flume/refactor/` captures rather than
  pending entries — each needs a home/shape decision, not a mechanical
  fix, per the posture-sweep rule's routing split:
  - `plan-json-manifest-import-layering.md` — `json_manifest.rs`'s
    `Manifest::read_kind` (419-435) imports `crate::import` (pipeline)
    to do its own discovery walk, unlike the frontmatter path (main.rs
    does discovery, frontmatter.rs stays a pure parser) — the same
    upward-edge disease architecture.md's Invariants section has ruled
    on four times already (0040 x3, normalize_path this cycle), a
    fifth undocumented instance.
  - `plan-format-read-decode-duplication.md` — the fs::read+UTF-8-decode
    job the open JSON-MANIFEST-READ-DECODE-CONSOLIDATE entry
    consolidates only within json_manifest.rs has two more copies
    outside it: `frontmatter.rs`'s `from_source_rooted` (172-180) and
    `toml_document.rs`'s `read` (33-42) — four total copies of one job,
    two filed, two not; a cross-format fix needs a shared-primitive
    home decision the file-local entry's shape doesn't fit.
  No exhaustive-match gaps, no hand-rolled mechanics beyond the
  documented pinned-semantics exception (byte-fidelity `---` scan,
  ordered-key JSON render), no verb/judge imports (grep-verified zero
  hits for main/install/bundle/engine/graph/dial/coverage/display/
  reporter across all four files), no dead plumbing, no vacuous tests
  found beyond the two captures above.
- Queue: 27 pending, unchanged — no entries filed or shipped this tick.
  6 pickable OPEN, 19 chained blockedBy, 2 parked on human action (all
  unchanged from last tick, gates re-verified live). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. Refactor
  captures: 2 live (plan-json-manifest-import-layering,
  plan-format-read-decode-duplication — filed this tick; next inbox job
  verifies each at HEAD and drains). Inbox empty.

Plan continues: yes — posture sweep resumes at `pipeline` (drift/
import/read/builtin_lock/placement), the rotation's next subsystem.
