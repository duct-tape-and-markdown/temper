# How does a size advisory judge a file the runtime cuts that no kind claims?

`specs/builtins.md`, "Default contracts": the size advisory judges "every file the runtime cuts at the per-file limit, a subdirectory's `CLAUDE.md`, `CLAUDE.local.md`, their imports and its own `.claude/rules/` among them". Subdirectory `CLAUDE.md` and nested rules are members (SIZE-ADVISORY-AT-SUBDIRECTORY-MEMORY-AND-NESTED-RULES). Two are not:

- `CLAUDE.local.md`: no built-in locus claims it (the memory kind claims `**/CLAUDE.md` and `**/AGENTS.md`; it appears only in AGENTS.md's absence gate).
- An `@path` import target: an edge's target file, a member only when some kind claims its path.

A clause binds to a member selection (`model/contract.md`, "selection"), so neither is reachable as written. Two readings fit: memory claims `CLAUDE.local.md` (a local-locus template, 0090's grammar) and the import target is sized through the reference graph at the root, which `import-backed` already walks; or the sentence is read as members only. Recommendation: the first, as a `builtins.md` memory-kind sentence claiming `CLAUDE.local.md` plus a root size clause over import targets.
