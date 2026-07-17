# 0032 — local is a locus, not a layer

- **Date:** 2026-07-16 · **Status:** accepted

## Context

The `(layer-delivery-format)` fork: 0030 ruled the layer stack in but
never said what a layer IS on disk. A dialed clause is a declaration row;
the gate reads declarations from nowhere but the lock; check is offline;
the payload is internal — so every delivery candidate amended a kernel
sentence, and the routed options (an uncommitted lock-shaped sidecar, a
narrow authored override format, payload-as-check-input) were all
mechanisms bolted beside the model. The human's question dissolved the
fork: why is "local" a unique case instead of a property of a kind? The
model already carries every needed part — the layout kind (the document
IS the governed source; emit writes nothing at its path), kind-declared
loci, and compiled row labels. Session-argued, human-ruled 2026-07-16:
no special mechanisms — where the model must stretch, widen it.

## Decision

Three widenings, no new mechanism:

- **The file locus gains a commitment class: `local`.** A local locus is
  per-machine and uncommitted; the *kind* is declared, committed, and
  reviewed — its members' documents are not. A local locus is
  **layout-only** today: the document is the governed source, read at
  check under the declared layout, emit writes nothing there, and its
  rows never enter the lock — they derive at read time under the shape
  the committed lock declares. The trust story is structural: review
  fixes what a local file *may say*; the gate reads what it *says*.
  Check-side-only (0030) stops being a rule to enforce and becomes true
  by construction. The layout-only coupling is the honest subset; a
  consumer who needs program-authored local projections is the condition
  that widens it.
- **A clause's compiled label becomes its address.** The lock already
  writes one per row and refuses collisions; it widens from
  opaque-to-everyone to a documented, deterministic, human-legible
  spelling, printed by every finding and by `explain` — see the annoying
  finding, read its label, dial it. The shipped **dial** kind consumes
  it: a temper-owned, local-locus TOML document (`.temper/dial.toml`)
  whose entries name a clause by label and declare the severity this
  machine reads it at. **The dial's schema is 0030's envelope**:
  severity is the only verb, deletion is unspellable, a dialed clause
  still reports (0023), and dialed softening stays inert in block mode.
  What was an admissibility contract over a layer's effect is now the
  shape of a kind.
- **A policy layer is a lock.** `check --layer <path>` joins the
  declaration rows of locks the invocation names over this corpus:
  kinds travel by name, the joined clauses range over the host's
  selections, and 0030's uncommitted rules bind the join — hardening
  unbounded in every mode, softening visible-only and gate-inert. The
  org authors an ordinary temper corpus (a bindings-only `harness()` is
  already expressible), runs its own emit, and distributes its lock —
  the lock stays the one interchange, org-side authoring stays TS, and
  fail-closed costs nothing new: a joined lock that fails admissibility
  is already a malformed lock, refused loud.

The kernel sentences amend rather than break: the gate consumes the
**lock family** — the committed lock, the read-time rows of local-locus
layout members whose kinds it declares, and invocation-joined locks —
still offline, still no runtime, payload still internal.

## Rejected

- **The uncommitted lock sidecar** (`lock.local.toml`, the session's own
  prior recommendation): a second lock file and a bespoke join for what
  a locus fact expresses; its drift story was a mechanism's cost, where
  the layout kind has no projection to drift at all.
- **A narrow authored override format**: a second home for clause
  vocabulary; the dial kind names clauses by address instead of
  re-spelling them.
- **Payload as check input**: breaks offline and the payload's
  internality — the two sentences most worth keeping.
- **Keeping the clause label opaque** and inventing a separate
  human-facing clause name: two identities for one row is the
  one-job-one-home violation; widening the label's spelling is smaller.
- **Program-authored local projections** (committed authoring emitting
  to local paths): deferred as the stated widening condition, not
  refused forever.

## Consequences

`representation.md` ("locus"), `pipeline.md` ("Layers", "The lock"), and
`contract.md` ("clause") carry the widenings — same commit, this record.
0030 gains a dated amendment note: mechanism re-spelled, rulings stand.
The `(layer-delivery-format)` fork resolves whole — both halves — and
its record deletes. Plan derives the entries: the locus class, the label
spelling (deterministic grammar is the entry's, collision refusal
already shipped), the dial kind and its TOML format mechanics, the
`--layer` lock join, the announcement line. The claude-code face's
`settings.local.json` is the first candidate local-locus layout kind
beyond the dial itself.

## Amended — 2026-07-16 (0034)

"Layout-only" was the wrong fence: it smuggled a format — markdown's
heading tree, via the definition of layout — into a provenance property,
and the first local-locus kind, the dial this same record ratified as
TOML, could not pass it (`(local-locus-toml-face)`, plan-caught, enforced
code by 40619a0). Decision 0034 re-spells the class from the invariant it
was guarding: a local member is **read-side only**, any declared format —
never an emit input or target, because emit's codomain is the committed
tree. The lock exclusion stands as a corollary (the lock is a committed
artifact); the dial as specified is unchanged. The widening condition
this record named — "a consumer who needs program-authored local
projections" — is retired rather than kept: 0034 rules that write path
out of the tool's job, not merely unbuilt.
