# 0109 — a locus takes templates, and placement is a path fact

- **Date:** 2026-11-26 · **Status:** accepted

## Context

Three plan questions, triaged under John's direction to rule as before:

- `memory-locus-holds-two-templates`: memory claims `**/CLAUDE.md` and,
  since 7b99c08c, `**/AGENTS.md`, but a file locus held one template. The
  `AGENTS.md` load condition (no `CLAUDE.md` at or above) had no home in
  the registration grammar.
- `colon-in-a-name-derived-identity`: emit splits an identity at `:` to
  place it, so an output style named `"Mode: Teach"` emits as nested.
- `published-schema-home`: which npm package ships the schema the
  unrepresented modeline points at (0108).

## Decision

- **A file locus governs through one or more templates**, each with its own
  exclusions (`representation.md`, "locus"). Memory holds its two. A
  subkind cannot carry `AGENTS.md`, since it governs within its base's
  locus, and a second kind would restate memory's facts (0078).
- **The `AGENTS.md` condition is a gate by a document's absence**
  (`builtins.md`, registration): its channels open only where no
  `CLAUDE.md` sits at or above. It is carried with the template and judged
  by reach, like a path-scope gate, and is never a channel entry.
- **A field-derived name is opaque, and placement is the member's own path
  fact** (`representation.md`, "member"). `:` is reserved only where
  identity derives from a path, so no `:` is ever parsed back into a
  placement.
- **The SDK package ships the schemas** (`distribution.md`, "Keystroke").
  It is the one npm package and pins the engine at its version.

## Rejected

- **Refusing a name that holds `:`.** Claude Code loads it, and 0101 forbids
  calling a loaded document a fault.
- **Escaping `:` in an identity.** That is the encoding 0090 rejected for
  `/`.
- **The platform engine packages for the schemas.** There are several, and
  only the release workflow assembles them.

## Consequences

The lock's `governs_glob` becomes a list. The CDN's URL grammar is cited
when the modeline entry is built.
