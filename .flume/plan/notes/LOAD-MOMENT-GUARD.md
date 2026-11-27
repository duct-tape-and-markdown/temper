# LOAD-MOMENT-GUARD

Shipped `loads(moment)` as a guard-only predicate (`Predicate::Loads`, lock key
`loads`, moment on the `values` column). A `when` over it binds its body at the
member. Label: `<kind>.when.loads=<moment>` (the guard key stands where a field
guard's field would; `contract::when_label_field`).

Derivation (`gate::load_moments`): on demand = an `on-read-beneath` member homed
below the root, or a `paths-match` member declaring globs; launch = a kind
registering `always`/`on-read-beneath`/`paths-match` otherwise. A member whose
kind registers none of the three (skill, agent, hook, mcp...) has NO moment and
the guard is silent over it. The spec says "at launch or on demand" without
deciding whether an invoked/listed member has a moment; silence mirrors an
absent guarded field. If the corpus means otherwise, that is a ruling owed.

Not counted: an `always` channel behind an on-demand absence gate, which
`explain` narrates as on demand. No built-in uses it since memory moved to
`on-read-beneath`, but a custom kind could, and the two readings would diverge.

Refused: a bare `loads` clause (judgeless), and a `loads` guard bound to an
embedded kind (it loads with its host).

Debt: `read::member_globs` and `graph::declared_globs` (now `pub(crate)`) are
near-duplicates differing on a map-valued field; and the load-moment read in
`gate` and the `explain` narration in `read` derive the same fact twice. Folding
`narrate_channel`'s moment onto `gate::load_moments` would give one home.

SIZE-ADVISORY-AT-SUBDIRECTORY-MEMORY-AND-NESTED-RULES still needs the rule kind
to register `on-read-beneath` (its own entry) before nested unscoped rules read
on demand.
