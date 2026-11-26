# EXPLAIN-STATES-A-MEMBERS-LOAD-AND-COST

Shipped: `explain <member>` gains a load strand (`read::load_strand`, joined in
`explain_target` beside the prose/version strands, so `read::explain`'s
signature and its ~12 callers are untouched). Channels come from
`gate::registrations_by_kind`; live/dead from `graph::dead_registration` (now
`pub(crate)`); gate state from the new `gate::absence_gate_states`, whose
shut half (`gate::shut_gates`) now backs `absence_gated` — one walk, open and
shut. Listing cost reads the kind contract's own `listing` clause through
`engine::listing_entry`, extracted from `over_listing`.

Gaps a later tick should weigh:
- Moment for memory comes off `always` plus the absence-gate scope. A
  subdirectory `CLAUDE.md` (no gate) narrates "at launch, unconditionally",
  but the runtime loads it on demand. The registration vocabulary cannot
  express that; the fix is in the kind/registration model, not the read.
- A skill's `paths` withholds its listing until a match (skill guidance).
  The strand does not narrate it: no registration channel carries that, and
  the reach judge does not read it either.
- Cost is `Features::rendered_chars` (body after frontmatter, characters).
  A skill's supporting docs and a rule's per-file truncation limit go
  unnarrated; the rule-size advisory already judges the limit.
