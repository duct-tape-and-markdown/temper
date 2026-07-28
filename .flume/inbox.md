<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## Requirement-grain field strand is dead for its one live consumer — observed at 18cf4d96

REQUIREMENT-GRAIN-TELEMETRY-FIELD-STRAND (6246cb22) shipped the wiring,
but two compounding defects make it narrate nothing for exactly the case
that drove it — an unfilled telemetry requirement (`context-arrives` in
this repo's own harness):

1. `requirement_detail` early-returns on the unfilled branch
   (src/read.rs:1507-1510, "No member satisfies it.") **before** the
   strand append at :1551 — so a telemetry requirement with no
   satisfiers never narrates evidence or 0046's stated absence. Verified
   live: `temper explain context-arrives` over an absent log with the
   verifier row present in lock.toml (line 107) prints no strand.
2. `requirement_field` (src/telemetry.rs:126) counts only records whose
   identity names a **satisfier** — for a satisfier-less requirement the
   denominator set is empty, so even past defect 1 every record filters
   out. The routed note (1986987e) asked for counts over the requirement's
   *declared events* with member/session denominators and a zero-hit
   member list; the satisfier join is right for satisfier-backed
   requirements but structurally zero for the bare-verifier shape, which
   is the shape the dogfood's one consumer has.

The shipped tests pass because every fixture gives the requirement
satisfiers — the untested cell is the live one. Fix shape: move the
strand append above the early return (an unfilled requirement still owns
its evidence and its absence line), and when the satisfier set is empty,
join over the whole lock-declared member corpus the way member-grain
`field` does, naming declared members with zero records.

Field validation worth carrying into the entry: member-grain 0046
narration did its designed job today — "declares tap registrations, but
the tap log carries no records" was true on this machine and pointed at a
real discard (a stale pre-tap `~/.cargo/bin/temper` 0.1.0 swallowing
every hook invocation since wiring; refreshed to 0.0.14, tap verified
recording). The requirement-grain surface is where that signal should
have been findable without knowing which member to ask about.

