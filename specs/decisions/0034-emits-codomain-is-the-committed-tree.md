# 0034 — emit's codomain is the committed tree

- **Date:** 2026-07-16 · **Status:** accepted

## Context

The `(local-locus-toml-face)` fork, plan-caught routing 0032: two ratified
sentences of one commit collide — `representation.md`'s local locus "is
layout-only" (and a layout is a template over markdown's heading tree)
against `contract.md`'s dial, a local-locus **TOML** document — and by
40619a0 the collision was enforced code (`local_locus_fault`), a refusal
DIAL-KIND would hit on contact. The session unwound it in two steps.
First, the fence's premise: 0030's ratified invariant reads byte-identical
"**over committed artifacts**", and "layout-only" is the fence you build
after dropping the qualifier. Then the human went past the repair: nobody
uses temper to project *into* a project — source control owns carrying
bytes to machines; temper is an **authoring tool**, and a projection
exists to iteratively update a source-controlled artifact. That is the
whole write story. Session-argued, human-ruled 2026-07-16.

## Decision

**Emit's codomain is the committed tree.** Temper writes no uncommitted
path: every projection is an iterative update to a source-controlled
artifact, and distribution is source control's job. (`bundle`'s output
directory is an export artifact handed *to* distribution, not a
projection into a harness — out of scope.)

Corollaries, replacing fences that stood beside the invariant as if they
were extra rules:

- **The local locus is read-side only, format-free.** A local member is
  something temper *reads* — an input to `check` (the dial), or a
  natively-local file the runtime owns, gated in place — never an emit
  input or target. "Layout-only" is retired: the property carrying the
  trust story is read-never-written, and markdown was smuggled in by the
  word. `local_locus_fault` re-cuts from "layout content only" to
  "read-in-place, any declared format".
- **Local rows never enter the lock** — now derived, not ruled: the lock
  is a committed artifact.
- **A `toml-document` read face joins the format inventory** — the
  deliberate-addition path the format-faces record blesses, `toml_edit`
  already the keystone crate — and it is honestly read-only: no emit
  round-trip exists for a local member to need.
- **0030's local-projection allowance is retracted** ("uncommitted
  members project only to natively-local targets"): there is no local
  write path; an uncommitted member is a check-side input, full stop.
  Dated errata on 0030 and 0032, never silent rewrite.

## Rejected

- **Layout-only, literal** (the dial is not local-locus, or the dial is
  markdown): format smuggled into a provenance rule. `settings.local.json`
  is JSON because the *product* made it JSON — a markdown-only local locus
  could never govern the product's own local file — and dial-as-markdown
  contradicts the ratified `.temper/dial.toml`.
- **A commitment-class partition on emit** (committed targets from
  committed sources, local targets from local sources — the session's own
  intermediate position): a two-class codomain for a one-class tool;
  local→local projection is a write path with no consumer, carrying an
  overwrite hazard and a second drift story.
- **Widening "layout-only" to "source-read-only" alone** (the session's
  first repair): fixes the word, leaves the write path unruled — the fork
  reopens at the first local kind that wants emit.

## Consequences

Same commit: `pipeline.md`'s emit section gains the codomain sentence and
its Layers bullet re-spells check-side structurally; `representation.md`'s
local bullet re-cuts to read-side-only; 0030 and 0032 gain dated errata.
`(local-locus-toml-face)` resolves and deletes; `(settings-local-kind)`
reduces to ship-or-not — one posture left, gated in place. Plan derives:
the `local_locus_fault` re-cut (40619a0 shipped the fence as written and
pre-declared exactly this), the `toml-document` read face, and behind
them DIAL-KIND and CHECK-ANNOUNCES-THE-LOCK-FAMILY unblock.

## Amended — 2026-07-17 — discovery's presumption yields to the declaration

The ruling's own sweep found the two remaining fences built on the
pre-local world, both in discovery (`src/import.rs`): the
gitignore-honoring walk has no local-class awareness — and a local
member's document is, in real use, always gitignored
(`settings.local.json` is ignored by the product's own convention) —
and the walk categorically skips the workspace dir, where `contract.md`
ratifies the dial's path (`.temper/dial.toml`). So the shipped class
finds its members only in fixtures where the file happens to be
tracked, and the dial is doubly unreachable: rows silently fail to
derive — the fail-open silence invariant 6 forbids.

One corollary closes both, human-ruled 2026-07-17: **a committed
local-locus kind's `governs` declaration overrides discovery's
presumptions — gitignore and the workspace skip — for exactly its own
scope.** The reviewed declaration is the authorship claim over its
unreviewed documents. "An ignored file is by declaration not authored
here" survives untouched for committed kinds, and install's adoption
walk keeps the presumption whole — adoption never converts a local
document into a committed member. Plan derives the walk carve-out with
the coupled landmine named: `local_document_rows`' silent none for
non-layout local kinds (`src/main.rs:1394`) re-cuts in the same entry
as `local_locus_fault`, or a JSON local kind passes admissibility and
silently yields zero rows.
