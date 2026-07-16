# Plan state

- Spec derived through: abe5d5d
- Audited through: ca4e866
- Residue swept through: ca4e866
- This tick: DRAINED the inbox — two notes, both filed by 25e9317 (which
  resolved `(supporting-doc-reach-clause)` and deleted its fork record; the
  file already holds no such key, so nothing to remove there).
  **Note 1 — John's ruling, routed as three entries.** Its own two named
  entries, plus the clause the ruling exists to license:
  (1) **INDETERMINATE-NEVER-SILENT** (`open`, the ruling's own "land this
  first") — verified on disk: `evaluate` (`src/engine.rs:214-219`) maps
  `Outcome::Indeterminate` onto the empty violation vec, and the judge-less
  arm (492-509: `Count`/`Unique`/`Membership`/`Degree`/`Kind`) is admissible
  in a per-artifact `expect` contract, so those five silently read green —
  `intent.md` invariant 6's silent degrade. The disposition is already in the
  tree as precedent, not invention: `DependencyExists` is fenced at
  `inadmissibilities` (110-140), its comment stating the exact rule ("must
  therefore fail admissibility, not degrade to a working no-op"). **The trap
  the entry names:** `inadmissibilities` is `pub(crate)` so
  `roster::admissibility` (`src/roster.rs:173-176`, 219) reuses it for a
  *requirement's* clauses, where all five DO have a live judge
  (`roster`/`graph`) — a naive push onto the shared list breaks the facet that
  works, so the fence is facet-split.
  (2) **PREDICATE-SELECTION-ALGEBRA** — the five move onto the selection
  algebra `contract.md`, "selection" already states (one algebra, the
  quantifier is the clause's grain). No decision owed, as the ruling says: the
  closed vocabulary does not grow, code lags explicit spec text; `engine.rs`'s
  own comments (492-504, 943-947) name it REQUIREMENT-CLAUSES-RECUT.
  (3) **SUPPORTING-DOC-REACH-CLAUSE** — the ruled clause itself: by-kind
  universal binding, each-grain incidence, locus-agnostic reach, advisory
  (invariant 5). It cannot ship before (2) — an incidence clause outside a
  requirement is inadmissible — hence the chain.
  **Note 2 — moot on disk, and one half plan cannot route.** Re-verified
  rather than filed: the SKILL.md 500-line budget **already ships** —
  `clause(maxLines(500), …)` at `sdk/src/builtins.ts:491`, in
  `skillDefaultContract`, severity `advisory`, its guidance already carrying
  the stays-in-context fact verbatim ("its body stays in context across
  turns — every line is a recurring token cost"). The cite differs
  (agentskills.io#progressive-disclosure @07-15 vs the note's
  code.claude.com/docs/en/skills @07-16) — corroboration of a shipped clause
  on the same fact, not date-staleness on a wrong one, so it is not even a
  citation rider. Nothing filed. The digest half (`docs/market-formats.md`)
  is `docs/` — human territory, fence-excluded, written by no autonomous
  phase; recorded in the commit body, not routed.
  **Two existing entries rewritten, not patched:** UNFILLED-EDGE-FIELD-NO-EDGE
  re-gated `open` → `blockedBy INDETERMINATE-NEVER-SILENT` (that entry settles
  how `format-places-edges`' absent fact reads, and both touch
  `tests/contract_template.rs` — two `open` over one file revert the wave);
  SKILL-NESTED-REFERENCE-DOCS's `builtins.ts` description no longer cites the
  dead `(supporting-doc-reach-clause)` slug, naming its successor entry.
  Closing checklist: the queue is disjoint — one chain of six
  (INDETERMINATE-NEVER-SILENT → UNFILLED-EDGE-FIELD-NO-EDGE →
  NESTED-FILE-LOCUS → NESTED-FILE-DISCOVERY → SKILL-NESTED-REFERENCE-DOCS →
  SDK-FIXTURE-WIRING-ONE-HOME), a second forking off it after
  UNFILLED-EDGE-FIELD-NO-EDGE (PREDICATE-SELECTION-ALGEBRA →
  SUPPORTING-DOC-REACH-CLAUSE, file-disjoint from the NESTED-FILE chain so the
  two run in parallel), plus the park. Fork board: four open, none blocking a
  queued entry; the fifth was resolved by 25e9317 and its record is already
  gone. Every rider record re-verified unmoved at ca4e866 last tick and
  untouched this one — this tick wrote no `src/`, `tests/`, or `sdk/` file.
  **One honest gap, surfaced not papered:** a `gate` names one tag, but
  SUPPORTING-DOC-REACH-CLAUSE rests on two real blockers —
  PREDICATE-SELECTION-ALGEBRA (semantic: the clause is otherwise inadmissible)
  and SKILL-NESTED-REFERENCE-DOCS (the `supporting-doc` kind itself, plus
  shared `builtins.ts`/`builtin_lock.toml`). The gate names the first; the
  entry's `notes` name the second. If build reaches it with the kind absent,
  the single-tag gate is the shape to fix, not the entry.
- Queue: 1 pickable (INDETERMINATE-NEVER-SILENT); seven serialized behind it
  as two chains forking after UNFILLED-EDGE-FIELD-NO-EDGE;
  PACKAGING-CHANNELS-REMAINDER parked (John's Apple notarizing + the v0.1
  lockstep tag).

Plan continues: no — every input below the inbox is dead. The spec delta is
empty (cursor abe5d5d is the last `specs:` commit), and no `src/`/`tests/`/
`sdk/` commit exists past ca4e866: the five commits since are four `plan:`
and one `chore(flume):`, none touching code. Build takes over with
INDETERMINATE-NEVER-SILENT.
