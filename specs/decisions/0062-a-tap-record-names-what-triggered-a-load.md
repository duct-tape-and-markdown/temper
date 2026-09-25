# 0062 — a tap record names what triggered a load

- **Date:** 2026-09-24 · **Status:** accepted

## Context

An adopter's harness audit (item 1). A `path_glob_match` record says which
rule loaded but not what caused the load, so the tap cannot answer "which
surface reached this rule". Claude Code's `InstructionsLoaded` payload
carries `file_path`, `load_reason`, `memory_type`, `globs`,
`trigger_file_path` (lazy loads) and `parent_file_path` (`include` loads)
(code.claude.com/docs/en/hooks, "InstructionsLoaded input", retrieved
2026-09-24). The tap keeps the first two. `pipeline.md` bounded the record
to the member or path, the load reason and the session id. Ruled by John
2026-09-24.

## Decision

**A lazy or included load's record carries the path that triggered it, or
the file that included it.** Both are paths, relativized like the
identity, so they are discriminants and not captured prose. A record an
older tap wrote reads without them.

## Rejected

- **Keep `globs` as well** (the adopter's ask): the docs define it as the
  loaded file's whole `paths:` list, not the pattern that matched. The lock
  already holds that list, and the pattern that matched follows from the
  trigger path at read time.
- **Keep `memory_type`**: it follows from the path.

## Consequences

`pipeline.md`'s tap paragraph names the two paths. Plan derives the entry:
`TapRecord` gains both as optional fields, the classifier reads them, and a
test pins an old-version record reading clean.
