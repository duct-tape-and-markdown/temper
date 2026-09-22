# Drift severity is a clause — decision stub

Status: **proposed, unratified** (session draft 2026-09-22 from the human's
ruling "make drift findings dialable"). Lands as `specs/decisions/NNNN-…`
only on review. Numbered then, not now, to stay clear of 0050–0053.

## Context

Five disk-vs-lock findings carry a severity fixed in code, always `warn`:
`config.stale`, `prose.include-stale`, `layout.import-stale`,
`locus.undeclared-member`, `layout.undeclared-member` (`src/drift.rs`).
Nothing an author declares moves them. The dial reaches clause labels only
(`src/dial.rs:95`). The enforcement mode acts at the `guard` boundary, not
on `check`'s exit code (`src/main.rs:353`). So no placement can make a
drifted content pin fail CI, and a pin that can't block is a comment. The
corpus already says the severity is the author's to set: the spine rule
("every check whose severity anyone could want to dial is a clause") and
`pipeline.md` "Drift" ("how loudly a hand-edit is treated is the author's
declared severity, never the tool's own determination"). `contract.md`
"clause" set the precedent when whole-graph reachability became a clause
in the root member's default contract. What's missing is the mechanism, so
this is a decision, not a horizon. Ruled by John 2026-09-22.

## Decision (proposed)

**Every drift freshness fact is judged by a clause.** One each-grain
predicate, `fresh`, holds of a member when each lock row it owns matches
disk: its projection's byte fingerprint, and each fingerprinted source
dependency (include, layout import). The default contract binds `fresh`
by kind to every kind at `warn`, which is today's behavior byte-for-byte
in the findings. An author hardens it by composition, a committed and
reviewed override, for one kind or all of them. A per-kind binding means
a `pin` kind can block while prose projections stay advisory. The dial
reaches it like any clause, and block-mode inertness of dial softening is
unchanged (`authoring.md`, "Layers"). The two undeclared-member facts get
the same treatment as a sibling clause over the governed locus, shipped
advisory as invariant 5 requires of a coverage clause.

## Rejected

- **Widening the dial to raw rule ids.** The dial is local and
  uncommitted, so CI's severity would live on one machine. It would also
  open a second severity channel outside the contract.
- **A global `check --strict` that makes warnings fail.** One baked
  severity for every advisory at once, not per fact or per kind.
- **Promoting `prose.include-stale` to `error`.** It swaps one baked
  judgment for another.

## Open for the ratifying session

- One `fresh` predicate that ranges over every row family, or one
  predicate per family. My lean is one: the selection narrows, not the
  predicate.
- Whether `config.stale` keeps its own rule id as the finding's family
  label, or the clause label replaces it in output. Adopters may have
  scripted against the old ids, so the answer needs checking against
  distribution's stability promises.

## Consequences (for plan, once ratified)

`contract.md`: `fresh` joins the clause section's whole-graph-context
examples beside reachability. `pipeline.md` "Drift": one clause states the
severity is declared. The engine lowers the five fixed-`warn` pushes to
clause judgments, and the SDK gains the constructor. The default contract
gains the bindings. `(declared-input)` in `docs/horizons.md` depends on
this.
