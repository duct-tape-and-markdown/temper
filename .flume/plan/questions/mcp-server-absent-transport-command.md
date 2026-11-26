# Does the mcpServer default contract require `command` when `type` is absent?

Site: `sdk/src/builtins.ts` `mcpServerDefaultContract` (~2339). The stdio
body is `when(enumOf("type", ["stdio"]), [required("command")])`, and its
guidance says a `command` "is also required" when `type` is absent. It is
not: `contract.md`, "clause", fixes a guard as silent where its field is
absent. So `{"mcpServers":{"x":{"url":"…"}}}` checks green, though the
`McpServer` doc (~784) calls it an entry Claude Code reads as stdio and
skips (code.claude.com/docs/en/mcp, retrieved 2026-07-15 per the site).

The corpus underdetermines the fix:

1. **Narrow the guidance.** Accept the gap as one the guard language
   cannot express. Reword the guidance to claim only what the clause does.
2. **A guard that holds on absence.** Amend "clause" so a guard can bind
   where its field is absent or one of a set. That is a language change.
3. **An unguarded clause.** Require `command` or `url`, or deny `url`
   without `type`, if the predicate set has a form that judges the pair.

Recommendation: 1 now, and record 2 as the rejected alternative unless a
second shipped contract needs an absent-defaulted guard. The posture sweep
found no other such site.
