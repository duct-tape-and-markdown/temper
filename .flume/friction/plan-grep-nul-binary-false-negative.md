# grep silently false-negatives on sdk/src/prose.ts (NUL sentinel → binary detection)

## Symptom
`grep -n "MENTION_SLOT" sdk/src/prose.ts` and `grep -n "mention" sdk/src/prose.ts`
both return nothing, though the file carries both dozens of times. The
`MENTION_SLOT` constant (prose.ts:31) is a literal NUL byte, so grep
binary-detects the file and suppresses every match — no warning line surfaced
in the pipeline used. Any "search before claiming not implemented" probe
(collaboration rule) over this file false-negatives silently; a plan tick
could file a false missing-surface claim, or a ship audit could wrongly
report an anchor dead.

## Cost this tick
One extra verification round-trip during the ship audit — the PROSE-INCLUDE
anchor re-check read the file manually after the grep came back empty. Small
this time; the miss mode (a silently false "not implemented") is the
expensive recurrence.

## Suggested fix
A line in CLAUDE.md "Tech stack"/conventions or the plan/build prompts:
`sdk/src/prose.ts` contains a literal NUL (the mention sentinel) — grep
binary-detects it; use `grep -a` or `rg` (rg handles it) when probing that
file. Alternatively an inbox item to respell the sentinel as the escape
sequence backslash-u0000 in source, so the file itself stays NUL-free text —
same string value at runtime, no binary detection.
