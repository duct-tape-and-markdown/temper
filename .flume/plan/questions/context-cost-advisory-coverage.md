# Do the context-cost advisories reach subdirectory memory and commands?

`specs/builtins.md`, "Default contracts": the advisories judge "a harness's skill listing against that budget" and "each rule against the size limit". Two runtime facts sit outside that text, and both leave an overrun silent:

- **Subdirectory memory is truncated too.** The memory docs ("Claude ignores the end of my subdirectory CLAUDE.md", re-verified 2026-11-26 by build) cut a subdirectory `CLAUDE.md`/`CLAUDE.local.md`, its `@`-imports, and a subdirectory's `.claude/rules/` at the same per-file limit. No memory-kind clause judges them, and the `rule` locus is root `.claude/rules/**` only. Does the size advisory extend to these, and is a subdirectory rule a member at all?
- **Commands share the skill listing.** One budget spans both kinds, but a whole-grain clause judges only its own kind's selection, so a command clause would budget commands apart and miss the joint overrun. The spec names a skill listing only. Does the budget span both kinds, and if so through what shape (a selection over two kinds, a root-bound clause)?

Raised by build notes RULE-SIZE-ADVISORY-AT-THE-ON-DEMAND-LIMIT and SKILL-LISTING-BUDGET-ADVISORY.
