# Representation — member · kind · locus · nesting

Layer 1 of the model: a faithful, typed representation of the harness as a
forest of nested members, rooted at the harness itself. Representation is
ground level — the value is the contract over it (`contract.md`) — but
everything the contract can reach, this layer must model.

## member

One authored unit: an instance of a kind. A member carries

- an **identity**,
- typed **fields**, projected to frontmatter or structured config,
- **prose** — its authored words, copied verbatim, byte-for-byte,
- **edges** — its declared references (`contract.md`),
- and, when its kind nests, **nested members**.

Members are what `emit` projects into artifacts (`pipeline.md`).

## kind

The type of a member. A kind declares

- the **field schema**,
- the **prose shape**,
- the **edge fields** — which fields are references, and to what,
- a **format** — a template literal rendering the member's values into its
  artifact: constant text around interpolated fields, prose, and child
  layers. The kind's format is the default; a member may dictate its own.
- and, when it nests, a **template** per inner layer: the child kind, plus
  the path pattern (relative to the parent's unit) when children are files.

A kind is data, never code; its extractor is composed from that data. Kind
identity travels by import, never by string. Built-in and user-declared kinds
are the same construct — ownership, not privilege (`../builtins.md`).

## locus

Where a member serializes. Binary:

- **file** — the member owns a file at a path its kind governs, or
- **embedded** — the member lives inside its parent's body, addressed per
  the parent's format.

## nesting

A kind may template inner layers of members, to arbitrary depth; a nested
member is a full member with its own kind — one member type in the model;
nested content never lives in a parallel value shape, whatever machinery
folds it from the parent's body. Nesting is **model containment**
and locus is **serialization**, and the two are orthogonal: a skill's bundled
reference documents are nested in the model yet own their files; a hook is
nested and embedded in `settings.json`; a spec decision is nested and
embedded in its document. A prose **leaf** — one addressable authored
string — is a nested member at the finest grain.

## The root member

The harness itself is a member — the root of the forest. Its kind's template
names the top layers (`.claude/`, the memory files, settings), and the
contract attaches to it the way a contract attaches to any member. Harness-
wide declarations are its fields — the enforcement mode among them —
overridable per member. There is no container above the forest; it is
members all the way down.

## Reach

- Structured config trees (`settings.json`, `.mcp.json`) are not an untyped
  residue: hooks, permissions, and MCP servers are embedded members with
  kinds and default contracts. A small residue of genuinely unschematized
  keys remains as opaque fields, named as such.
- The engine is corpus-generic — any corpus of authored artifacts can be
  modeled as members and gated — but exactly one governed corpus ships: the
  harness. A second corpus is a feature, never a founding assumption.
