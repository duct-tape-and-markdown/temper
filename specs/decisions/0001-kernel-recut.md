# 0001 — the kernel re-cut

- **Date:** 2026-07-06 · **Status:** accepted · **Pre-state tag:** `metaphor-era`

## Context

The prior corpus stifled the build: metaphor served as technical vocabulary
and model motion billed the engine a migration per tick (form: 0002). Every
term was traced to its code mechanism before re-founding.

## Decision

Eight nouns, two layers. Representation: member · kind · locus · nesting.
Contract: edge · clause · selection · well-formedness. The harness is the
root member. Locus is binary (file | embedded), orthogonal to nesting. A
requirement is a shipped kind: its intent is prose, its verifier is an edge,
its "must be filled" an overridable cardinality clause at error severity.
Reachability is a default-contract clause, not an engine check.

## Supersedes (in-source rulings, overturned on evidence)

1. `coverage.rs:44-50` — satisfies-as-edge rejected as "a fake artifact
   kind": its premise (member = file on disk) is retired by the embedded locus.
2. `compose.rs:78` — "required is never cardinality": its irreducibles were a
   severity, a vacuity check, and a live bug (coverage's satisfier set is
   kind-blind; roster's is kind-typed).
3. Engine-owned unconditional reachability: it carries a dialed severity, and
   a dialable check is a clause (the spine rule).

## Retires (vocabulary, one-way)

law → invariant/spine rule · floor → default contract · posture →
locus+nesting · genre → nested kind template · landscape → governed corpus ·
the world → root node · the definition → predicate vocabulary · front door →
install · content-faithful → verbatim+deterministic · blast radius → impact ·
seam → emit payload · prose "embed" → include · expect/require/satisfies/
mention → bindings and edges over selections · means → requirement prose ·
required → default cardinality clause.

## Rejected

Keeping the coined vocabulary with better docs (the names carried the tax);
acyclicity as a clause (a cyclic graph makes evaluation ill-defined); a
frozen release-line corpus (intent stays evergreen; the kernel is the center).

## Consequences — work the model demands of code

Embedded locus in the engine; one edge enumeration (declared + directive);
requirement as a member kind (fixes its prose never persisting); the
satisfier-set bug; kinds for hooks, permissions, MCP servers; the scalar
`Kind` enum renamed off the noun.
