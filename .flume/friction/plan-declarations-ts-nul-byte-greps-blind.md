## Symptom

`sdk/src/declarations.ts` contains a literal NUL byte — `placementKey` builds
its key as `` `${host}\x00${kind}\x00${key}` `` (line ~540). `grep` and `rg`
therefore classify the whole 715-line file as **binary** and print nothing for
a normal query: `rg -n "ClauseRow" sdk/src/declarations.ts` emits only
"binary file matches", and piping it through `head` (the reflexive shape)
swallows even that, so the search reads as a clean **zero hits**.

This is a silent-wrong-answer failure, not a slow one. Three greps in a row
returned nothing while auditing this file, and the natural inference — "then
declarations.ts doesn't build clause rows" — is exactly backwards: it builds
one for every `expect` binding (line 684). That inference would have inverted
the tick's central finding. It was caught only because the file's own imports
visibly named `ClauseRow`, which contradicted the grep.

The delimiter choice itself is sound (NUL is the one byte a kind/key cannot
contain, so the key cannot collide). The cost is that it is invisible in
review — nothing at the call site says "this file is now un-greppable".

## Cost this tick

~10 minutes and 6 wasted tool calls; one near-miss false audit conclusion that
would have been committed as reconciliation evidence. Recurrence is high and
open-ended: `declarations.ts` is the seam every kind/clause/row question
routes through, so it is among the most-grepped files in the repo, and every
future tick and session pays the same trap at full price.

## Suggested fix

Preferred — remove the trap at its source: swap the NUL for U+001F (unit
separator) in `placementKey`. It keeps the exact non-collision property (no
kind/key contains it) while leaving the file plain text to every text tool.
One-line change, no semantics moved; the key is internal and never
round-tripped through the lock, so nothing external pins the byte. This is
product work — route via `.flume/inbox.md` if a human agrees.

Fallback if the byte must stay: state it where searches start — a
`CLAUDE.md` line noting that `sdk/src/declarations.ts` reads as binary to
`grep`/`rg` and needs `-a`. Weaker: it warns rather than fixes, and only
helps whoever read that line first.
