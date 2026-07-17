# 0030 — review is the price of softening

- **Date:** 2026-07-16 · **Status:** accepted

## Context

The `(local-overrides)` fork, plus its unrecorded dual: a consumer war game
(8 simulated personas, adversarially verified, 2026-07-16) rated a personal
uncommitted layer the most-demanded missing capability (4 of 8, max
severity adoption-blocker) and separately surfaced an org compliance
overlay — a hardening layer applied over a team's harness from outside it.
The two are mirror images: one softens below the committed harness, one
hardens above it. Session-argued, human-ruled 2026-07-16. Prior art
verified live: Claude Code's own settings stack (Managed > CLI args >
Local > Project > User, code.claude.com/docs/en/settings, retrieved
2026-07-16) and the policy-as-code CI pattern (invocation-carried policy,
org-owned pipeline).

## Decision

The harness value composes from an **ordered stack of layers** under the
one merge algebra the SDK already speaks — later layers win by array
surgery; no layer is a special species. Four rulings:

- **Precedence is the provider's fact, never temper's choice.** Which
  slots exist and who beats whom is a claim about how the consumer's
  runtime resolves configuration; temper carries it as cited data on the
  provider face, exactly as default-contract clauses carry their cites.
  The claude-code face emulates the product's documented stack: user <
  project < local < invocation (settings docs, retrieved 2026-07-16); the
  user slot (`~/.claude`, the harness that follows the person) is named
  and reserved, ungoverned today. Temper adopts the ordering, not the
  override semantics — the bounds below are temper's own gate rules,
  which no provider documents.
- **Two rules attach to any *uncommitted* layer** — one review never saw
  — not to a layer's name. (1) *Check-side only*: committed bytes are
  layer-invariant — emit with and without an uncommitted layer is
  byte-identical over committed artifacts; uncommitted members project
  only to natively-local targets (`settings.local.json` is the first).
  The lock captures the committed harness alone; uncommitted layers
  compose over the locked declarations at check time. (2) *Review is the
  price of softening*: the committed harness may soften anything — that
  is a reviewed diff. An uncommitted layer hardens without bound, in
  every mode; it softens only visibly — a dialed clause still reports,
  never deletes (0023) — and its softening is inert in block mode. A
  block-mode pass on any machine therefore implies the shared gate's
  pass; a machine may only be stricter than CI, never laxer.
- **Policy arrives with the invocation.** `check` composes additional
  layers named by its invocation; whoever owns the invocation owns the
  top of the stack, and org authority is the org's CI definition — temper
  builds no trust model. A layer that fails to parse fails the check —
  fail-closed, a deliberate divergence from the product's tolerant parse
  of managed settings, because a stripped policy entry in a gate is
  silent fail-open. Runtime enforcement (MCP allowlists, permission
  ceilings at session time) stays the product's managed layer's job;
  temper gates authored artifacts and does not duplicate machine-level
  delivery.
- **`check` announces every active uncommitted layer and every softened
  clause** — a developer can never mistake their personalized view for
  the shared one.

## Rejected

- **Two bespoke features** (a personal-override file and a separate
  policy-injection mechanism): two merge rules and two precedence stories
  where one algebra serves, and the third layer demand (per-client
  parameterization) arrives homeless.
- **Unbounded local surgery**: full array surgery locally is fine as
  mechanics and is retained — what is refused is the two *outcomes*:
  altering committed bytes, and silent removal (mute). The envelope is an
  admissibility condition over the layer's effect, not a stunted verb
  set. The reopening condition: a real case where visible-advisory is
  demonstrably not enough and a full local mute is needed.
- **Block mode ignoring the local layer wholesale**: forbids a machine
  from holding itself stricter; only unreviewed *softening* is
  gate-inert.
- **Temper-invented precedence in the kernel**: bakes one provider's
  fact into the provider-neutral model and lies where a runtime resolves
  differently.
- **Machine-level policy delivery** (managed-file/MDM analog): serves the
  interactive-runtime threat model, which the product's own managed
  settings already own.
- **A signing/authority model**: a product temper is not building; the
  invocation composes with whatever attestation the pipeline already has.
- **Tolerant parse of a policy layer**: right for sign-in, fail-open in a
  gate.
- **Deferred admission** (design fixed now, built when a real consumer
  needs a layer — the session's position, argued from the simulated
  provenance of the demand and the launch wedge): overruled — ruled an
  important capability, admitted now.

## Consequences

`pipeline.md` gains the "Layers" section — same commit, this record. The
`(local-overrides)` fork resolves and its record deletes; the compliance
overlay is answered by the same stack and opens no fork. Plan derives the
entries: the stack and envelope in engine and SDK, the claude-code face's
cited precedence declaration, `--layer` on check, the announcement line.
The claude-code precedence cite re-fetches raw at encode time per
`builtins.md`.
