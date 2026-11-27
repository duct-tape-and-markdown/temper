# The rule's 200-line budget fires on a path-scoped rule

Found when flume-80 ran `temper check` (0.0.26) over flume's harness. The rule default contract's `extent("lines", 200)` clause (`sdk/src/builtins.ts`, `ruleDefaultContract`) sits outside any guard. Its guidance says it judges unconditional rules ("always-on context, paid every session"), but it fired on flume's `.claude/rules/platform-facts.md`, which carries `paths:` and renders 528 lines.

The very next clause already runs under `when(loads("on-demand"), …)` (0120). By the clause's own stated intent, the line budget belongs under `when(loads("launch"), …)`. `specs/builtins.md` states no line budget for rules, so this is the contract's guidance disagreeing with the clause's binding, not a spec gap.

Acceptance: a `paths:`-scoped rule, and a rule under a subdirectory's `.claude/rules/`, past 200 lines raise no `extent` lines finding. A root rule without `paths` past 200 lines still does.
