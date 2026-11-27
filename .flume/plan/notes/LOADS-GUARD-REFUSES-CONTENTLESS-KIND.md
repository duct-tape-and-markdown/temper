# LOADS-GUARD-REFUSES-CONTENTLESS-KIND

Shipped. `gate::carries_content` is the one predicate (event, connection,
enablement, registry are content-less); `channel_load_moment` returns None
through it, and `two_greens_dispatch` runs `engine::contentless_loads_guards`
over each at-locus kind's own contract when none of its channels carries
content. Embedded kinds pass `channels: None` — their `loads` guard is already
refused by the locus.

Observed: inherited clauses are not re-checked against a subkind's own
registration (they passed at the base's dispatch). A subkind whose base loads
but whose own registration is content-less would inherit a silent `loads`
guard — corpus does not say whether registration inherits; not acted on.
