<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Six engine predicates are gate-checkable but not authorable — the SDK
  ships no constructor for them. Engine (`src/contract.rs` Predicate enum):
  `SectionContains`, `MustDefine`, `DependencyExists`, `Enum`, `Optional`,
  `Range` all evaluate in conformance; `sdk/src/contract.ts` exports 15
  constructors and none of these six, so a corpus can declare them only by
  hand-editing a lock it doesn't own. Entry shape: six constructor exports
  mirroring the existing ones (the ClauseRow wire already carries them —
  the engine's own hand-written built-in contracts prove the read side).
  First customer waiting: a counterpart corpus's "every member of kind K
  carries marker M" clause (their per-Decision `Rejected:` law is the
  named consumer) — expressible today as an expect binding on K the
  moment `sectionContains`/`mustDefine` are authorable; no new
  machinery, no ratification. Observed at 9833f20.

- First-consumer field report (counterpart corpus, relayed): a declared
  relationship edge field targeting an **embedded-locus kind** fires an
  admissibility finding — **with the host's templates declared in the
  lock** (repro: their `serves` -> `domain`), so template presence does
  not populate the target set. Code-side hypothesis, to re-verify: `by_kind`
  ranges each kind's file-locus units (`resolve_kind_units` walks a
  governs locus embedded kinds don't have), so an embedded kind presents
  as unmodeled-or-empty to `graph::admissibility`/route resolution — yet
  the body rules a nested member is "a full member with its own kind — one
  member type in the model" (`representation.md`, "nesting"), and nothing
  scopes edges to file-locus targets. The gap is adjacent to
  LAYOUT-RELATIONSHIP-EDGES (in build now): the same widened enumeration
  should decide whether embedded members join the route-resolution target
  set, one ruling not two. Observed at 9833f20.
