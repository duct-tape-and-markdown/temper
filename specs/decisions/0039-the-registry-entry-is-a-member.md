# 0039 — the registry entry is a member

- **Date:** 2026-07-17 · **Status:** accepted

## Context

Centercode's surface inventory (observed at 7cf9ff0): the one
un-member-typed surface left in a fully-migrated consumer harness is
`extraKnownMarketplaces` — authored residue on the `settings:`
passthrough while the plugins it feeds are typed `installed-plugin`
members whose `plugin@marketplace` enablement keys dangle by convention
against the untyped blob. The shipped `marketplace` kind does not cover
it — that is the publisher-side catalog at
`.claude-plugin/marketplace.json` (verified by the consumer against the
built binary's roster and by the session against `builtins.ts`); this
segment is the consumer-side registry entry, documented as the
name-keyed `extraKnownMarketplaces` object in `.claude/settings.json`,
each entry carrying the same `source` union
(code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-17; the
product stores them per-user in `known_marketplaces.json`, which names
the kind). Coverage-bar shaped, with what 0036 lacked: a real consumer
asking. Session-argued, human-ruled 2026-07-17.

## Decision

**`known-marketplace` ships**: the fourth fields-only registration
member — one consumer-side registry entry under `settings.json`'s
`extraKnownMarketplaces` collection address, name-keyed, its fields the
documented shape (`source` union, `autoUpdate`), its channel the
registry entry itself — the `installed-plugin` pattern at a sibling
address. **The plugin→marketplace dependency becomes a declared edge**:
the marketplace half of an `installed-plugin`'s `plugin@marketplace`
key resolves to the `known-marketplace` member by name (0029
addressing), so an enablement naming a marketplace no registration
declares is a finding, never a convention.

## Rejected

- **Widening the shipped `marketplace` kind to both loci**: two
  documents, two shapes, two owners (publisher catalog vs consumer
  registry) under one name is conflation, not coverage — the consumer's
  own field note drew the line first.
- **Leaving it as settings residue**: was the honest posture while
  nothing modeled it; against the coverage bar and a real consumer's
  dangling dependency, residue is now a gap, not honesty.

## Consequences

`builtins.md`: twelve kinds, four registration members, the
`known-marketplace` bullet with the cite. Plan derives: the kind and
its default contract mirroring the documented shape, the enablement-key
edge, and the `KnownSurface` registry entry for the segment — landing
beside the in-flight coverage-advisory fix so the segment is named
correctly the moment it is modelable. The routed demand note discharges
into this record.
