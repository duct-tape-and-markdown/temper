# 0036 — the machine's own file gets a kind

- **Date:** 2026-07-17 · **Status:** accepted

## Context

The `(settings-local-kind)` fork, registered from 0032's own
Consequences: `settings.local.json` was the first candidate local-locus
kind beyond the dial. The "can it" half was built out from under the
question — 0034 made a local member read-side only under any declared
format, the discovery override finds always-gitignored documents, the
dial proves the pattern end to end, and `check` announces every local
member it reads — so ship-or-not was all that remained, at zero
upstream cost. The argument that ships it is the coverage bar, not
demand: `builtins.md` owes a kind to every artifact the surface
documents as real, and `.claude/settings.local.json` is a documented
Claude Code artifact present on practically every working machine (the
product writes permission grants into it; code.claude.com/docs/en/settings,
retrieved 2026-07-16). Session-argued, human-ruled 2026-07-17.

## Decision

**The claude-code face ships `settings-local`**: the machine's own
settings overlay at `.claude/settings.local.json`, a JSON document at
the **local** commitment class — per-machine, uncommitted, read in
place and gated, never emitted, its rows derived at read time and its
presence named in every verdict, all inherited from the shipped local
pattern with no new surface.

Its posture is `settings.json`'s partial governance, one step more
honest about its boundary:

- **Fields, not members.** The kind governs the file's documented shape
  — the documented keys typed, the genuinely unschematized residue
  opaque and named as such. It is *not* a registration manifest: a hook
  or plugin enablement written locally is an uncommitted registration
  whose member-hood and reach stay unmodeled and named as such — the
  installed-plugin cache precedent. Modeling uncommitted members' reach
  is a widening a real consumer must argue.
- **Singleton identity from its fixed documented path.**

## Rejected

- **A local registration manifest** (local hooks and enablements as
  first-class uncommitted members): models reach review never sees, for
  which no consumer has asked; the fields-only kind gates the file's
  shape today and leaves that widening argued, not defaulted.
- **Leaving it ungoverned**: was honest while the local class made a
  JSON local kind impossible; after 0034 the absence is a coverage gap
  with no excuse left.

## Consequences

`builtins.md`: the roster grows to eleven (eight file members), the
`settings-local` bullet carries the cite, and the kind sits outside the
domain partition beside the manifest kinds — machine configuration,
never authored session content. Plan derives the entry (kind, default
contract with the documented-profile clauses the settings docs settle,
fixture under the discovery override). The `(settings-local-kind)`
record deletes.
