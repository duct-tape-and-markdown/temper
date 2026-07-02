<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- COMMENT-DIET (human-queued, appetite: TRIM): sweep `src/` to the revised
  comment etiquette in `.claude/rules/rust.md` (researched + sourced — RFC 1574,
  Ousterhout APoSD, antirez). Keep: one-line-summary `///` docs (+ Errors/Panics
  where reachable), why/invariant/checklist comments, terse spec pointer tags,
  ~10-line `//!` module overviews. Cut: module-head narrative restating specs/
  (the heavy offenders: compose.rs ~900 comment lines, main.rs ~47%, roster.rs,
  drift.rs, graph.rs, import.rs, kind.rs, contract.rs), what-comments narrating
  ordinary code, compliance narration. Comments-only, behavior-neutral — the
  diff must contain zero code changes and the full gate suite proves it. Fan
  out per-module if that makes cleaner entries.
