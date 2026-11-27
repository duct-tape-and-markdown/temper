# What load moment does `loads(…)` read for a member that reaches context in two steps?

`contract.md` "clause" gives every guarded member "the member's load moment (at launch or on demand)". `builtins.md` "Default contracts" names a skill's `paths` as registering *its listing* on a read of a match — a load-moment fact — while its body loads on invocation. Shipped (LOAD-MOMENT-GUARD, `gate::load_moments`): a kind registering none of `always`/`on-read-beneath`/`paths-match` has no moment, so a `loads` guard is silent over every skill, agent, hook and MCP member.

Two readings, each consistent with the corpus: (a) a moment is a body-into-context fact, and listed/invoked members have none (today's silence); (b) the listing is a load, so an unscoped skill reads `launch` and a `paths`-scoped one `on-demand`. Nothing in intent ranks them. A ruling is a `contract.md` "clause" sentence. No built-in contract depends on it; an author's `loads` guard over a skill is vacuous until it lands.
