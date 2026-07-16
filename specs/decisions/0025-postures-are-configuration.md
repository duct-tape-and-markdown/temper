# 0025 — postures are configuration: the citation respell and the composed-body admission

- **Date:** 2026-07-16 · **Status:** accepted

## Context

A two-day consumer-lane field campaign (centercode, 07-14 → 07-16) authored
a real harness against the shipped surface and returned one design in four
cuts (`docs/proposals/posture-recursion.md`, with a worked prototype
beside it): the harness files content by posture at every scale — harness
→ member (posture picks the kind), member → block (posture picks the block
type) — and a machinery surface's natural grain is a short list of
posture-typed values, each carrying the properties its role needs. The
evidence was convergent hand-rolls: a `cite()` helper and eleven
template-literal bodies existing solely to thread positional mentions,
both live corpora minting posture-shaped embedded kinds independently, and
a `supportingDocs()` factory re-deriving per skill what the skill built-in
already declares. An adversarial review (same day, interactive) held four
riders; the proposal's later cuts absorbed two, and the session rulings
encoded here resolved the rest.

## Decision

- **A posture is a consumer-declared member type** — an embedded kind with
  the property shape its role requires. The vocabulary (orientation,
  directive, reference, step, any refinement) is configuration, never
  engine: the engine resolves the types without understanding them.
  **Host-agnostic**: a type declares no host; which types may compose a
  kind's body is the adopting corpus's admission, a contract declaration
  over the host kind — and a shipped kind's composed body admits
  corpus-declared types by the same declaration.
- **Edges are member properties at every grain**; a member's reference set
  is a derived view, never a second authored list. **Mention respells as a
  rendering claim on a field-declared edge** — one noun, three declaration
  loci, one resolution path, one degree semantics.
- **Rendering is the declaring kind's format, as data.** An embedded format
  may place a closed, engine-derived set of facts about its edge targets —
  name, address, kind, projection path relative to the host's own
  projection — so a rendered reference is true by construction; instance
  prose never spells its target. A format that omits a declared edge
  renders a contract the prose does not represent; that check is a clause.
- **Delivery is three layers**: `install` scaffolds ground-truth
  representation of the built-ins as they are; the composed-body machinery
  is unopinionated capability; the opinionated layer is guidance describing
  the climb from plain built-ins to the postured shape. No doctrine is
  placed by machinery.
- **Supporting docs need no new kind**: the skill built-in already declares
  its bundled reference documents as nested file members
  (`../builtins.md`, "The shipped kinds"); the SDK closing that gap is
  reconciliation toward the spec, and the documents become addressable
  reference targets for free. Runtime loading semantics stay unencoded
  until probed.

## Rejected

- **Engine-shipped posture kinds** (the proposal's first draft): the engine
  stays vocabulary-free. Honest cost, accepted: a corpus that replaces the
  guidance vocabulary forgoes fleet-level comparison; tooling keys on the
  guidance vocabulary when present.
- **A member-level `references` framework key** (the proposal's second
  draft): a second declared-edge mechanism beside edge fields recreates the
  multiplicity this decision exists to collapse.
- **Keep mention as its own locus, add ergonomic sugar** (the adversarial
  review's counter): the right instinct — no new mechanism — aimed at the
  wrong noun; the field mechanism already existed, so the collapse goes the
  other way.
- **Host-bound posture types** (`withinHosts` at declaration): the same
  type means the same thing everywhere; binding hosts at declaration is
  vocabulary coupling reborn one level down.
- **A smuggled-citation lint** (prose extraction; the fourth draft briefly
  held it): heuristic guessing inside a deterministic gate. 0020's line
  holds — prose spans stay prose; the rendered→declared direction is held
  by the block grain and intake judgment.
- **Authored display** (status quo): every authored display string restates
  a fact the engine knows — the shipped example's own
  `display: "src/main.js"` was a restatement bug in the flagship demo.
- **`install` scaffolds the doctrine** (the proposal's Element C as
  drafted): machinery placing opinion breaks the spine rule; guidance
  describes the climb instead, and "correct" stays defined by the declared
  contract, never by the shipped theory (invariant 4).
- **A separate skill-package or nesting kind for supporting docs**: the
  built-in already owns the shape; a parallel kind would be the
  duplicate-surface disease.

## Consequences

`contract.md` "edge" collapses to three loci with mention as a rendering
claim and gains the derived-reference-view sentence; `representation.md`
gains the derived-target-facts bound on embedded formats and the admission
sentence under nesting; `builtins.md` names the composed-body admission
for shipped kinds — same commit, this record. Mention machinery and wire
rows stay valid (0024 posture): a respell, nothing retires, no pre-state
tag. Code reconciliation routes through plan against the amended sections:
the composed-body admission over built-ins, format-placed derived target
facts, the format-omits-edge clause, and the skill built-in's nested
reference documents. The guidance climb and the example recut follow the
machinery. `docs/proposals/posture-recursion.md` remains the design
narrative — candidate intent, superseded as ruling by this record.
