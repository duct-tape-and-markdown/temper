# Engineering shape

How code enters this repository — the form standard the phases build
against. Product intent lives in the model; this page governs shape, and it
exists so consolidation is citable: a `per` into this file is how a refactor
becomes a pending entry.

## One job, one home

- A job the tree already performs is extended where it lives, never
  reimplemented beside itself. Before adding a function, module, or helper,
  search for the existing surface; the priority is delete or subsume, then
  extend the existing surface, then generalize a near-duplicate, then —
  last — add new.
- A new surface beside a near-duplicate names, in the commit body, the
  existing surface considered and why it did not fit.
- A second implementation of one job — two matchers, two normalizers, two
  encoders — is residue, fileable against this section whenever no pending
  entry consolidates it.
- Test scaffolding is a surface too: shared fixtures and builders live in
  one home (`tests/common`), never copy-pasted per file.

## Libraries before hand-rolls

- Where a crate in the sanctioned set (CLAUDE.md, "Tech stack") carries the
  mechanic — directly or transitively — adopt it; a hand-rolled
  reimplementation of a solved mechanic is the same residue class.
  Expanding the sanctioned set is a human call, proposed through the inbox.
- The exception is pinned semantics: where the corpus fixes exact behavior
  (byte-fidelity round-trips, charset mechanics), the implementation
  follows the corpus, and a library is adopted only where its semantics
  match the pinned contract.

## A shared concept is one type

Grounded in the field: every cross-feature defect the first consumer
harvest surfaced was a consumer iterating a **partial enumeration of a
shared concept** after a producer widened it — a parallel vec beside
the canonical edge set, an XOR branch where template layers are a
union, a deletable class the ledger never learned. The treatment is
shape, so it lives here:

- A new fact-shape enters a shared concept as a **variant of the one
  shared type, never a parallel structure**. The engine's shared
  enumerations — edges, members, template layers, lock rows, deletable
  things, discoverable paths — each have exactly one type, and a
  private partial copy of one is residue fileable against this section.
- Consumers of a shared enumeration hold **exhaustive matches**, so a
  new variant refuses to compile until every judge answers it — the
  `Format`/`project_bytes` precedent: the next variant answers the
  match by construction. A `_` arm over a shared concept is a seam
  defect waiting for its field report.
- An entry that widens a shared enumeration names that enumeration's
  **other consumers** in its own files[] — "who else reads this set?"
  is derivation's question, never the consumer's bug report.

## Derived state is computed, never stored beside its source

A value computable from existing state enters the tree as a
computation, never a second stored field kept in sync by discipline —
two copies of one truth is the shared-concept disease in miniature,
and sync is a bug class no compiler checks. The product already holds
this bar for its own artifacts (`pipeline.md`, "Emit": derived facts
are computed, never authored twice); the code holds it for itself.
Caching a derivation is the sanctioned exception, taken for a measured
cost (per the cost section's measure-first bar), and a cache is one
home with one invalidation — never ad-hoc copies at call sites.

## The fix lands at the mechanism

A special case layered on shared infrastructure — a path carve-out in
generic code, a kind-name test inside a kind-generic judge, a flag
threaded through to exempt one caller — is the signature of a change
pitched too shallow. The preferred fix generalizes the mechanism until
the case stops being special: the discovery override precedent was not
"also read this one gitignored path" but "a committed kind's declared
claim overrides discovery's presumptions, for exactly its scope."

- A branch on a *specific instance* inside code that is otherwise
  generic over its type is residue fileable against this section.
- A divergence that genuinely is the right depth — a documented
  per-instance fact of the external format, like the hooks
  collection's deeper nesting — is declared and cited at the site,
  never left looking like an accident.

## The gauntlet corpus

Single-feature fixtures cannot find composition seams; the field
harvests that surfaced them came from a real corpus using feature
*combinations*. One in-tree kitchen-sink fixture harness holds every
composition the model claims legal — composed bodies over templated
hosts, embedded edges with scopes on both endpoints, partially-declared
manifests, local members under ignore rules — with `check` and `emit`
snapshot-tested over it. A feature entry that adds a composable surface
extends the gauntlet where its feature meets the existing ones: each
addition pays its composition cost once, at ship time, instead of in a
consumer's repo later.

## Cost scale is hoisted, and pinned by count

Work that scales with the consumer's input — tree walks, file reads,
glob compilations — is hoisted by design: whole-input work computes
once per run and is shared, never recomputed per kind, member, or call
site. The expectation is enforced the only way this repository enforces
expectations — decidably:

- **The pin is a count, never a clock.** A test asserts the work count
  — one walk per run, one compilation per glob, no file read twice in a
  phase — deterministic and machine-independent, where a wall-clock
  threshold is the gate that cries wolf. Timing stays a manual signal a
  human reads; the field's session experience is the true bar, and no
  corpus number stands in for it.
- **Diagnosis is measure-first.** When the field reports a cost, a
  generated fixture — synthetic input at consumer scale, built at test
  time, never committed — names where the cost concentrates before any
  cut; the numbers pick the cut, never the guess, and the fix lands
  with its count-pin.

## A green verdict is proven non-vacuous

Three field incidents in one week were the same silence: a judge whose
input set collapsed to zero kept passing — a test asserting over a
derivation that yielded nothing, a bound reachability clause ranging
over an emptied edge family, a coverage advisory classifying a registry
instead of the file it spoke about. Loud-or-nothing (intent.md,
invariant 6) gets its code-level enforcement here:

- **A vacuity pin rides every judge test**: the fixture asserts the
  judged set was populated — `n > 0` of the thing the test exists to
  judge — before asserting the verdict. A test that passes over zero of
  its subject is not a test, and is residue fileable against this
  section.
- A judge whose selection may be legitimately empty asserts the empty
  case **explicitly**, in its own test — vacuous-by-design is spelled,
  never inherited.

## A fix ships the test that would have caught it

The cost section's "lands with its count-pin," generalized past
performance: every defect fix includes a test that **fails on the
pre-fix tree** — a walk fix with its walk-count pin, a seam fix with
its gauntlet cell, a false green with its real assertion. The entry's
tests[] names it and the commit body says what it pins. A fix whose
regression genuinely cannot be pinned decidably says so out loud in the
commit body — the named exception, never the default.

## A seam gate reads what the real writer wrote

A gate whose claim is "the two sides of a seam agree" proves nothing
when both sides come from the same hand: comparing a writer's bytes
against its own prior bytes pins self-agreement, and driving the
reader over hand-authored counterpart rows re-authors the writer's
vocabulary by the tester's hand. Either way a one-sided respell ships
green — a shipped release paid for this when a wire label its writer
serialized in one spelling and its reader demanded in another passed
both a writer-vs-writer byte pin and a hand-rowed reader suite.

- **An agreement gate drives the real writer's output through the real
  reader** — the actual producer runs and the actual consumer decodes
  what it wrote, however much cheaper the hermetic fixture would be
  (`tests/builtin_lock_frozen.rs` is the shape: a live SDK emit,
  decoded by the engine reader).
- **The scope is agreement claims, only.** Refusal and shape tests
  keep their hand-authored input — a real writer cannot produce the
  malformed row a reader's refusal is tested on. A hand fixture is the
  tool for "the reader refuses X", never for "writer and reader
  agree".

## An export earns its consumer

Public surface with no consumer is residue: an export born as
scaffolding outlives its scaffold and becomes API someone must
excavate later. Grep-verifiable, so the sweep holds it mechanically:

- An SDK root export, a `pub` item, a widened `pub(crate)` — each needs
  a caller outside its own module (a test counts). Zero-consumer
  surface is fileable against this section.
- A seam opened for a planned second consumer carries that consumer's
  name at the export site; when the plan dies, the export dies with it.

## Narration is the ladder's bottom rung

Every check lives at the most deterministic layer that can express it —
a type, then a test or pin, then a clause, and only at the bottom,
prose. A doc comment is where intent lives **while nothing mechanical
can hold it yet**, and its adjacency is the point: it loads exactly
when an agent opens the code it governs, and it moves in the same diff
as that code (measured: remote narration rots in days, adjacent
narration held). Three grades:

- **Cited external facts stay at point of use, permanently** — the
  collaboration rule already requires the cite at the claim.
- **Intent commitments** no rung above can express — declared
  leniencies, deliberate directions, cross-boundary co-ownership —
  are the local spec, warranted as long as they stay inexpressible.
- **Promoted scaffolding shrinks.** When a property gains its pin,
  type, or clause, the prose that hand-held it shrinks to a pointer
  **in the promoting commit** — narration is a queue for the ladder,
  never an archive beside it. Prose asserting a property a test now
  pins, and behavioral paraphrase of code an agent reads directly,
  are residue fileable against this section.
