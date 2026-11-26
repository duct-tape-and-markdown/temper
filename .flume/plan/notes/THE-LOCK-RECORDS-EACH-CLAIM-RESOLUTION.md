# The lock records each claim resolution

Landed: `KindFactRow.ceded` (`[{ to, pattern }]`). `compose::resolved_kind_rows` fills it
once from `effective_kinds`, and emit records it in the lock. `member_projection_path`
reads `facts.ceded`. A cede to a row whose `base` is the placing kind is refused as
`SubkindProjection`; any other cede is refused as the new `HostUnitProjection`. Drift
learns no built-in names. Explain narrates the recorded cedes. `CustomKind.ceded` now
carries the claimant too (`Vec<CedeRow>`).

Decisions worth a look:
- check still derives the cedes through the same compose function. It does not read
  the column: `from_kind_fact_row` ignores it, so a relocated built-in never inherits
  stale cedes from the embedded lock. The input and function are the same, so the
  answer is the same. If 0102's "every verb reads it there" means a literal read, that
  is a separate entry, and it would need a policy for locks that lack the column.
- If the kind set does not lift, emit records no resolution and does not refuse. Load
  refuses that lock, and about ten "refuses at load" tests emit such locks on purpose.
- Exact overlap (the previous note's question): recorded cedes made the noise
  visible (the gauntlet's rule ceded `docs/guides/*/**`). `reaches_into_at_any_depth`
  now refuses a root whose literal segment differs from the subtree's at the same
  depth; a segment carrying `*` still counts as agreeing. The four built-in cedes
  (agent, command, memory and rule to skill) are real overlaps.
- The `sdk/src/declarations.ts` file needed no change, since the engine writes the
  column. Only the generated bindings moved.

Human half: the dogfood `.temper/lock.toml` sits outside build's paths. It needs a
re-emit to gain `ceded` columns. Until then the guard's placement checks against it see
no cedes.
