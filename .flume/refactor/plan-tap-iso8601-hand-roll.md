## Surface
`src/tap.rs`'s `iso8601_utc_timestamp` (src/tap.rs:313-368) and its helper
`is_leap_year` (src/tap.rs:370-372): a hand-rolled Gregorian calendar
computation — epoch-day arithmetic off `SystemTime`/`UNIX_EPOCH`, a hand-rolled
leap-year test, a hand-rolled days-in-month table — to produce an ISO-8601 UTC
timestamp string. This is the exact "Libraries before hand-rolls" mechanic
(specs/process/engineering.md): date/time formatting is a solved mechanic a
crate carries directly, but no crate in the sanctioned set (CLAUDE.md, "Tech
stack") does date/time — confirmed via `rg -n "iso8601|SystemTime::now" src/
tests/`, only this one site — so adopting one is the human sanctioned-set
call the section itself names, not a plan-derivable swap.

## Observed at
cd35a551 (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation
Add a minimal date/time crate (`time`, with its `formatting`/`parsing`
features, is the narrower fit for one RFC 3339 stamp) to the sanctioned set
and replace the hand-rolled block with it; if the human declines the new
dependency, the alternative is declaring this a deliberate pinned-semantics
exception at the site (rust.md's comment taxonomy) rather than leaving it
silent.
