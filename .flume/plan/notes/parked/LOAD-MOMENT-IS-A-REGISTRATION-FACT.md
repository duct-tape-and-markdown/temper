# Load moment: built, green but for the dogfood lock build cannot write

The work was built, and it passes every gate but one. The SDK gate and both new
explain tests pass. `cargo test` fails three tests: two example-harness lock
drifts, which build can re-emit, and
`cli::self_host_check_is_clean_over_tempers_own_surface`, which build cannot fix.

Why: `.temper/lock.toml` snapshots the built-in rows `memory`
(`["always"]`) and `skill` (no `paths-listing`). `row_relocates_builtin`
(src/compose.rs) admits a built-in-named row only if its registration equals
the embedded one. So `check` raises `kind.admissibility` "collides with an
embedded built-in". `.temper/**` is outside build's fence, and emit needs the
new SDK, so the re-emit cannot come first. Every afterMerge run reverts the
entry.

Decision needed (one of):
1. Ship as one commit with the dogfood re-emit: widen this entry's fence to
   `.temper/lock.toml`, or hand it to a human.
2. Product: a built-in-named row at the built-in's own locus is a stale
   snapshot. It reads as the built-in, or draws a "stale lock: re-emit"
   finding, never a collision. Adopters hit this on every upgrade that moves
   a built-in fact, and the collision text misleads.

Shape built (redo, ~340 lines):
- kind.ts/kind.rs: `on-read-beneath` (no field, never dead), and
  `paths-listing(field)` (dead on the same terms as paths-match).
- `kind::member_home` comes out of `absence_gate`, and
  `gate::read_beneath_homes` feeds the explain `load_strand`.
- memory registers `[on-read-beneath]`. skill registers user-invoked,
  description-trigger and `paths-listing(paths)`.
- The description-trigger narration says "listed once its `paths` open".
- Tests: the two lines in read_verbs.rs, plus the updated `always` assertion,
  builtins.test.ts and builtin_kind.rs.
- Comments in graph.rs, contract.rs and builtins.ts no longer say "paths
  adds no registration".

Diff saved at /tmp/load-moment-is-a-registration-fact.patch, if it
is still there.
