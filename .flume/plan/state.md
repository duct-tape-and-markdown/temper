# Plan state

- Spec derived through: b8396d4
- Audited through: d88ed75
- Residue swept through: d88ed75
- This tick: POST-SHIP RECONCILIATION of `18f219d..d88ed75` — inbox empty,
  `.flume/refactor/` empty (README only), spec delta empty, so the window was
  the first live input. Both motions ran; the spec cursor is copied forward
  verbatim.
  **Audit — nothing to drop, two routing pointers discharged.** The three
  entries the window shipped were already dropped at 9005fc1, so the drop
  motion was pre-paid. Read the files, not the log: 7e266ad ships
  `type("keywords", "list")` at `sdk/src/builtins.ts:655`, the `type`
  predicate's first shipped consumer; 6dbeb86 refuses an authored body on a
  `json-document` member in the projection loop; 0e7dca2 makes `bundle`'s two
  `.claude-plugin` manifests members of the kinds that type them, rendered
  through the one write dispatch (`drift::project_bytes`, now `pub`).
  Both open-question pointers named entries that have now **shipped**, so both
  conditions were re-tested against disk rather than left standing:
  `(closed-surface-predicate)`'s "TYPE-CLAUSE-CONSUMER is scoped to leave that
  bullet standing" — verified, the bullet stands at `builtins.ts:609`; and
  `(nested-field-addressing)`'s "the bar stays thin for `marketplace.json`" —
  now a live fact, not a prediction: `bundle` publishes the catalog through
  its kind and `check` reads it back, gated by
  `required("owner")`/`required("plugins")` alone.
  **Sweep — one find, and it forced the fork board's shape.** 7e266ad's own
  body states the remainder honestly: `type(field, kind: ValueType)`
  (`sdk/src/contract.ts:74`) declares one lattice kind, never a union, so six
  documented `string|array` fields of `plugin-manifest`'s strictest profile
  stay ungated. That is a **third distinct cause** of the same category —
  decidable, documented, unexpressible — which `specs/builtins.md` ("Default
  contracts") does not sanction; it names only "Undecidable properties are
  deliberately absent". Growing the closed predicate vocabulary is a corpus
  decision (`specs/model/contract.md`, "clause"), never a build tick's, so the
  find is fork-routed, not filed.
  **Registered `(clause-vocabulary-holds)` and retired the two instance keys
  into it.** Both prior records already said a category ruling "may settle the
  pair together" while each was keyed by its own mechanism — so the question
  was filed three ways and answerable once. One record now carries the ruling
  wanted plus its **four** instances, each with its own mechanism and its own
  answer (nested field addressing, closed-set allow-list, type alternation,
  and `skill`'s shape rules — the fourth was carrying no key at all). No
  pending entry declared either retired slug, so nothing dangles. This is
  consolidation of plan-registered forks, not a re-ruling: no instance's
  analysis was dropped, and the lifecycle rule's delete-on-resolution is
  untouched.
  All four accepted-debt riders re-verified unmoved on disk and restamped to
  d88ed75; the window touched none of their files and no queued entry carries
  them. Both parks re-tested: `git diff 18f219d..d88ed75` over `src/graph.rs`,
  `tests/graph.rs`, `release.yml` is empty; `MAX_IMPORT_HOPS` still 5 at 65
  under a cite claiming five; four era tags, no version tag, crate 0.1.0 vs
  npm 0.0.7.
- Queue: 3 entries — 1 pickable (CHECK-HARNESS-ONE-HOME, re-verified at
  d88ed75: `rg 'fn check_harness' tests/` still finds every definition at its
  cited line), 2 parked on human acts. No file appears in two entries —
  checked mechanically; the pickable entry is disjoint from both parks.

Plan continues: no — every input below post-ship reconciliation is dry (inbox
empty, no captures, spec delta empty) and this tick reconciled the window to
HEAD. Build takes over: CHECK-HARNESS-ONE-HOME is pickable.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.

**Waiting on a ruling:** `(clause-vocabulary-holds)` is now the fork board's
largest — four shipped contracts hold decidable, documented rules the algebra
cannot spell, and the corpus sanctions only "undecidable" as a reason a clause
is absent. Nothing is broken (every hold is named in its contract's header,
the marketplace one is pinned by a test), but the gate's reach is thinner than
`specs/builtins.md`'s "Strictest documented profile" stance reads.
