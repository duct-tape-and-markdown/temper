# How does a clause spell "no `:` anywhere, no leading `-`" for an agent `name`?

Driver: the AGENT-NAME-CLAUSES-FOLLOW-THE-CURRENT-DOCS ship note. code.claude.com/docs/en/sub-agents (retrieved 2026-11-25) says a file whose `name` contains `:` or starts with `-` is skipped. Today's default contract cannot refuse either. Its guidance teaches both rules, so check passes an agent Claude Code will not load.

Both rules are decidable, but the clause vocabulary (`specs/model/contract.md`, "clause") cannot spell them. `allowedChars` is a whitelist, and an open-ended name has no whitelist to state. `deny` matches whole values only. `builtins.md` "Default contracts" says to check the strictest documented profile, so the property is owed. What is underdetermined is how to spell it:

- (a) a forbidden-character predicate plus a leading-character one, two primitives;
- (b) one negated-charset form of `allowedChars`, plus a position for the leading rule;
- (c) leave the rules unspelled, keeping guidance only.

(a) and (b) are vocabulary amendments, so they are corpus work. Recommend (b): it widens an existing primitive instead of adding two.
