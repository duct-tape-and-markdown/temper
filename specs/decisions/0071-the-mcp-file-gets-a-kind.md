# 0071 — the committed `.mcp.json` gets a kind

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(unmodeled-surface-registry)` fork. `coverage.unmodeled-surface`
could no longer fire: its one call site hands it the full built-in set,
and both registry rows read as governed whole. A harness carrying
`.mcp.json = {mcpServers, somethingElse}` reported nothing, and the
unknown key reached no row, no projection input, and no finding —
invariant 6. `.mcp.json` documents one top-level key, `mcpServers`
(code.claude.com/docs/en/mcp, retrieved 2026-09-29). 0050 set the
precedent for a committed config file: a container kind, documented keys
typed, residue named. Ruled by John 2026-09-29 on the session's
recommendation.

## Decision

**`.mcp.json` gets a container kind, `mcp`**, the 0050 shape: singleton,
committed, `mcpServers` as the **mcp-server** collection address, every
other key opaque residue, named, and the byte fingerprint over the whole
file. With both config files governed whole, the unmodeled-surface
advisory has no subject and **retires**.

## Rejected

- **Keeping the advisory with `.mcp.json` re-segmented.** It names the
  gap every run instead of closing it; 0011's bar says a documented
  surface earns a kind.
- **Growing the registry with another surface.** No cited `.claude/`
  surface lacks a kind, and inventing one past the docs is 0011's
  forbidden move.

## Consequences

The rule, its `segments` column, `Segment`, `manifest_top_level_keys`,
and both verdict branches retire; `KNOWN_SURFACES` shrinks to the
unclaimed-entry exclusion list, whose `with_locked_kinds` survives. The
fixtures that withheld a built-in to observe the advisory retire with it.
