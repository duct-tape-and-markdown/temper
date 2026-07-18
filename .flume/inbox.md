<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- RULED (session, under John's 07-18 delegation): NORMALIZE-PATH-SUBSYSTEM-PLACEMENT's parked
  condition is resolved — architecture.md's Invariants section now names the fourth edge and
  its direction: `normalize_path` moves from graph.rs (judges) to address.rs (foundation),
  where its shape already lives. Unpark the entry; serialize against whatever open entries
  share graph.rs/drift.rs/import.rs. observed at 07a9c04

- RULED (session, same delegation): GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE's semantics
  question — the shared extractor holds declared_globs' trim/filter semantics, and BOTH
  judges consume the one filtered set: one world, two judges, diverging in verdicts, never
  in what the declared-glob set IS. Rationale from glob-valid's own documented mission
  ("a pattern that silently matches nothing becomes a finding"): a whitespace-padded entry
  judged raw compiles as a literal-space glob that silently matches nothing yet passes —
  judging the trimmed set serves the mission better, and it is the semantics reachability
  already documents and pins (tests/graph.rs an_absent_or_blank_paths_field_is_reachable).
  Blank/whitespace-only entries stay dropped-not-flagged (the documented fallback), and NO
  new blank-entry finding class is added — that is a check-surface widening with zero field
  demand (0035 evidence bar). Observable delta is limited to whitespace-padded globs being
  judged trimmed. Unpark the entry. observed at 07a9c04
