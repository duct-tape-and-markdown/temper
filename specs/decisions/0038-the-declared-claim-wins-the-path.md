# 0038 — the declared claim wins the path, and the star names the member

- **Date:** 2026-07-17 · **Status:** accepted

## Context

Centercode's standards-as-layout-source prototype (field-reported,
observed at 4cc3081) surfaced two gaps in the layout-source story at
scale. First: a consumer kind's exact-path locus and the built-in
skill's `supporting-doc` template both materialize a member from
`.claude/skills/jobs/conventions.md` — the layout kind works end to end
while a phantom `supporting-doc` twin rides beside it, counted by
coverage, narrated by `explain`, and judged by the supporting-doc
default contract (a bogus `degree` finding at 0 incoming, since the
real edge targets the layout member). Second: the natural scaled locus
— root `.claude/skills`, glob `*/conventions.md`, one standards doc per
skill directory — is refused by the SDK's name splice, and identity
would collide anyway (every member named by the same file stem). The
consumer capped the kind at one member per declaration to proceed.
Session-argued, human-ruled 2026-07-17.

## Decision

**A path a declared kind's locus claims leaves a template's discovery —
exactly that path, nothing else.** This is the third instance of the
declaration-beats-presumption family (0034's errata: a local `governs`
beats gitignore and the workspace skip; the admit-join fix: admission
overrides exactly the grain it names): a corpus-declared locus is
reviewed authorship, a built-in's file template is a shipped default,
and specific authorship carves out of the default's reach. Two
*declared* kinds claiming one path stay a refusal — the existing
governs-collision invariant is untouched. The phantom twin and its
findings retire; one path, one member, decidably.

**Single-`*` directory globs are admitted, and the star names the
member.** A kind may govern `<root>`, glob `*/<file>`, with member
identity derived from the starred directory segment — the identity rule
the built-in `skill` already exercises (`UnitShape::Directory`, its
directory's name), generalized to the constructor every author uses:
ownership, not privilege. Identity source becomes the kind's **declared
fact** — file stem or starred segment, spelled on the kind, never
inferred — and identity uniqueness stays gated by the existing
machinery.

## Rejected

- **Refusing the declared-vs-template double-claim** (symmetry with
  governs collisions): makes the consumer's pattern impossible without
  replacing the whole template layer via admission — a sledgehammer for
  a one-path carve-out — and a template is a default, not a peer claim.
- **Template wins / first-discovered wins**: a shipped default
  overriding reviewed authorship inverts the family's principle.
- **Deriving star-segment identity implicitly** (no declared fact, stem
  when no star, segment when starred): identity is the kind's fact and
  stays spelled; inference is where the next phantom collision breeds.
- **Full multi-`*` or `**` name splices**: no consumer, and identity
  from multiple wildcards has no decidable single spelling; the
  single-`*` form is the demanded, well-defined subset.

## Consequences

Plan derives the entries: template-discovery arbitration (the carve-out
at discovery time, phantom twins retired, a gauntlet cell where a
declared locus meets a template glob), and the star-segment locus
(splice widened, identity-source fact on the kind, both SDK and engine
faces). The consumer's exact-path workaround un-caps against the star
form. Both field notes route to the inbox carrying this ruling.
