# 0015 — a manifest represents its container's segment

- **Date:** 2026-07-07 · **Status:** accepted

## Context

`(json-projection-format)` asked what a JSON kind's format is — off by one
noun: an MCP entry in `.mcp.json` authors nothing; the member is the
*registration*, the entry its presentation. The corpus also carried a
tension: permissions as embedded members (`model/representation.md`)
versus a derived union (`model/pipeline.md`).

## Decision

A structured config file is a **manifest**: a projection representing a
controlled segment of a **container member** — the container's own fields,
its members' **registration facts**, and **derived aggregates** (the
permission union — never a member). Claude Code does registration this way
at every level: settings and `.mcp.json` are the harness's manifests,
`plugin.json`/`hooks.json` a plugin's, `marketplace.json` a marketplace's
(code.claude.com/docs/en/plugins-reference, retrieved 2026-07-07).

- A registration member (hook, mcp-server, installed plugin) is a
  **fields-only kind** — no prose, no artifact of its own, no lock rows
  (0012). Where its registrations surface is a **collection address**
  (`mcpServers.*`, `hooks.<Event>`) — a kind fact, the manifest's fence.
- The format is document delegation (0013): a real parser owns the
  grammar; the adapter — `frontmatter.rs`'s peer — walks declared key
  paths into the generic extraction; undeclared keys are opaque fields.
  Reading an unrepresented manifest infers its registered members; a
  represented one regenerates whole, canonically (declared order then
  residue, LF); the unrepresented write stays 0008's splice.
- A plugin is both faces: consumer-side a registration member
  (`enabledPlugins`, any settings scope); producer-side a nesting
  container whose manifests are its projections — `temper bundle` is the
  bespoke instance the general write subsumes.
- Levels are **peer forests**: user, project, local, enterprise carry one
  corpus shape, merged at runtime by the surface per documented, cited
  per-kind precedence. temper governs the project forest; an ignored local
  file is by declaration not authored here; another level is another
  target path, never a model change.

## Rejected

Manifests as document hosts with embedded prose members (no words to
guard; makes the permission union a fake member). Modeling the runtime
merge (an effective-harness narrator is read-verb work). LSP-server and
monitor kinds now (documented components; 0011's bar when reached).

## Consequences

Reach recuts; `builtins.md` drops permissions from the member list; the
JSON adapter and canonical-manifest write file as derivable work;
hook/mcp-server/plugin kinds draft next with their own doc fetch (0014's
pattern); bundle's manifests become general-write instances.
