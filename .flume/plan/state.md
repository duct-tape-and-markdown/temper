# Plan state

- Spec derived through: abe5d5d
- Audited through: 74f4e62
- Residue swept through: 74f4e62
- This tick: ROUTED the spec delta's last commit — abe5d5d (0027, the nested
  file child composes its path from its host). The delta is now empty.
  **0027's Consequences, bullet by bullet:** (1) "`representation.md`'s locus
  section goes ternary and `builtins.md`'s skill bullet names the
  honest-subset claim — same commit, this record" — moot, verified in
  abe5d5d's own diff (both files: the locus list's third spelling, the skill
  bullet's markdown-subset sentence). (2) "the composition surface" —
  **NESTED-FILE-LOCUS**: `Locus` is binary in both layers (`sdk/src/kind.ts:57`;
  `Governs`, `src/kind.rs:30`, a required root+glob), and `projectionPath`
  refuses every non-`at` locus (`sdk/src/emit.ts:129`), so a file-owning child
  has no spelling. (3) "discovery off the template pattern" —
  **NESTED-FILE-DISCOVERY**: `discover_kind_units` (`src/import.rs:145`) keys
  every scan on `governs`, and a nested-file kind has none;
  TEMPLATE-FILE-CHILD-FACT's declared-fact-only bound (`src/kind.rs:576-579`)
  retires there. (4) "the built-in adoption — SKILL-NESTED-REFERENCE-DOCS
  re-enters buildable" — filed under that tag: `skill`'s template plus the
  `supporting-doc` kind.
  **The one half that does not re-enter buildable, stated for the veto:**
  0027 ships `supporting-doc`'s advisory "reachable from its skill's body"
  clause, and no spelling fires it. `degree` is the only edge-count predicate
  (`src/contract.rs:222`) and `graph::degree` judges it over a *requirement's*
  satisfiers alone (`src/graph.rs:240-246`, wired off `assembly_requirements`,
  `src/main.rs:938`); a default contract is a by-kind `expect` array, and that
  path returns `Indeterminate` (`src/engine.rs:506-509`) — no finding. The
  clause would ship green and silent: invariant 6. `contract.md`'s "selection"
  already licenses the by-kind binding, so this is code lagging intent — but
  the fix is the node-set/edge-scope recut the engine's own comments name
  (`src/engine.rs:492-504`), a language change a decision ratifies before it
  is built (the `glob-valid` precedent). Registered
  `(supporting-doc-reach-clause)`; the adoption ships without the clause,
  exactly as 0027's "re-enters buildable" says.
  Closing checklist: the queue serializes as one chain —
  UNFILLED-EDGE-FIELD-NO-EDGE → NESTED-FILE-LOCUS (shared `sdk/src/kind.ts`,
  `sdk/src/emit.ts`, `sdk/test/emit.test.ts`) → NESTED-FILE-DISCOVERY (shared
  `src/drift.rs`, `tests/nested_member.rs`) → SKILL-NESTED-REFERENCE-DOCS (no
  shared path; serialized because the locus is what a discovered child is
  classified as). PACKAGING-CHANNELS-REMAINDER's park re-tested at 4ba483f and
  true verbatim: 4 tags, all era-named, crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9 still defers darwin. Every cited line resolves on disk
  (kind.ts:49-59, emit.ts:114-140, kind.rs:29-37/567-580, drift.rs:569-605,
  import.rs:114-167, builtins.ts:142-158, builtin_kind.rs:70-291). Two riders
  found carriers and were re-routed in open-questions: drift.rs:570's "retired
  `projectionPath`" onto NESTED-FILE-LOCUS, and builtins.ts:565/611's two dead
  `PACKAGE.md` cites onto SKILL-NESTED-REFERENCE-DOCS — nine entries had opened
  that file and left them. Fork board: five open, none blocking a queued entry.
- Queue: 1 pickable (UNFILLED-EDGE-FIELD-NO-EDGE); three serialized behind it
  (NESTED-FILE-LOCUS → NESTED-FILE-DISCOVERY → SKILL-NESTED-REFERENCE-DOCS);
  PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag).

Plan continues: yes — the spec delta is drained, and the post-ship
reconciliation window (74f4e62..ca4e866 — two build commits, 13c58ed and
e76ec85) is the next live input.
