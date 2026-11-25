# LAYOUT-SATISFIES-READ-OFF-THE-DOCUMENT-AT-CHECK: shipped, scoped to layouts with a satisfies section

compose.rs replace_committed_layout_entries now also replaces each discovered
host's lock `satisfies` rows with the ones its document lowers, but only when
the kind's layout declares a `satisfies` field region.

Why scoped: the lock's `satisfies` family merges two sources under the same
host address. One is the program's own member.satisfies
(sdk/src/declarations.ts satisfiesRows, which a layout member may carry). The
other is emit's layout_satisfies (drift.rs emit, `declarations.satisfies.extend`).
A row carries no origin. For a layout with no satisfies section, every stored
fill is program-authored and stays.

Residual gap: a layout host that has a satisfies section AND module-authored
`satisfies` loses the module fills at check. That surfaces as an unfilled
`required` requirement, which is loud and not a fabricated pass, but it is a
doubled authoring home the corpus does not rule on. Closing it means either
refusing module `satisfies` on a layout host whose layout carries the slot at
emit, or tagging the satisfies row's origin (a lock-shape change). Both are
plan's or a human's call.

Bare-id lock rows (resolve_kind_units also matches `row.member == unit.id`)
are left alone. Emit writes addresses, and dropping a bare id could take a
same-named member of another kind's fill with it.
