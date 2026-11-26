# Does a member string the runtime matches against a target gate when it matches nothing it was meant to?

Driver: field report (2026-11-26, a second harness's survey). One class, three instances, each a string Claude Code matches against something else, where a miss is silent:

1. A settings `permissions` `ask`/`deny` path rule that misses a relative or absolute spelling of the same file, or a shell write that bypasses the Edit tool.
2. An agent meant to override a built-in (`Explore`, `Plan`, `general-purpose`) whose name misses, so it becomes a second agent.
3. A `permissions.allow` rule that no longer matches the command a hook or script actually invokes.

Checked: no spec section states any of the three. `sdk/src/builtins.ts` (settings contract header) declares "whether a permission rule reads correctly" undecidable and absent, but 1 and 3 are decidable from the documented rule syntax (code.claude.com/docs/en/permissions — UNVERIFIED this tick). 0101 covers reading a member as the runtime reads it, not the author's intended target; 2 needs a near-miss judgment against built-in names, which is taste unless only an exact case-fold collision counts. 3 correlates two members (cf. `cross-source-edge-correlation`).

Asked: is any of these a default clause, an opt-in clause, or out of scope? A ruling is a `specs/builtins.md` "Default contracts" edit.
