<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- Field defect (centercode, 16,814-file platform repo; session-verified at
  src/import.rs:311,182): **check's standing cost scales with the
  consumer's tree times the kind count** — plain `check .` runs ~40s warm
  (42.4/39.2/40.1) at ~36 members where the dogfood repo is instant, and
  `check` rides the SessionStart hook, so a real-repo consumer pays it at
  every session open. Mechanism confirmed: `discoverable_paths` — the
  full-tree gitignore-honoring walk — recomputes per `discover_kind_units`
  call (one per kind) and again per nested-file host (:182), while
  `scan_locus`'s own doc comment says one scanner serves every kind off "the
  same already-computed `discoverable` set" — the scan is shared, the walk
  is not. Fix direction (derivable, engineering.md "one job, one home"): the
  walk is one job — compute the discoverable set once per run (per
  local-governs flavor) and share it across every kind and host; any-depth
  globs then cost one walk, not N. Separately: CLAUDE.md's "I/O-bound over
  tiny files — no performance pressure" premise is field-falsified at
  consumer scale; the quality bar needs the sentence to survive contact
  (correctness and clarity still outrank micro-opts; a 40s session open is
  not a micro-opt). observed at 4cc3081

- Field demand, parked (same report — the postscript withdrew the driver):
  **lazy grounds + content anchors.** An eager read-only ground (`src`,
  `**/*.{cs,vb}`) materialized 2250 members to answer seven mention
  addresses (+45s); the wants were on-demand address resolution (a stat per
  cited address) and an optional content **needle** the gate asserts the
  resolved file still contains (the citation's meaning — where a content
  hash is alarm-fatigue and line numbers rot fastest). The consumer then
  ruled their standards exemplar-free — durable fact, no live-tree
  citations — so the demand has no live driver; other doc classes (a
  base-harness-style implemented-by mapping) still would. Parked under the
  0035 evidence bar: lazy grounds change coverage/narration semantics
  (2250 members vs 7 resolved addresses is a model choice, not an
  optimization), so it is ratified against a real driver or it waits. The
  needle's design taste rides this note for that day. observed at 4cc3081
