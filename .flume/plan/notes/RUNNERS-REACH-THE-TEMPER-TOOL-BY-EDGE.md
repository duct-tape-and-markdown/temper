# Gate hooks and taps reach a tool; taps and the typecheck still leave no lock edge

Shipped: `gateHook({ tool })` and `telemetry(events, { tool })` replace `binary`
(a `Remedy` slot names the move at `tsc`). The seam keys became
`temper:gate-tool` / `temper:tap-tool`, carrying the composed tool's address
or `null`; `src/drift.rs` `RunnerTools` renders `{package,bin}` as
`./<workspace>/node_modules/.bin/<bin>`, `{file}` as `./<file>`, `{system}`
bare. `null` is the temper package bin at that anchored path. The
unrepresented install path still writes `temper` on the PATH (pinned).

Debt plan should weigh:

- **Degree undercount.** A gate hook's handler carries a `runs` edge to its
  tool, so the lock's `runs` family counts it. A tap does not: a telemetry
  declaration is a requirement, not a member, so no `RunsRow` has a runner
  address to key on. The typecheck has the same gap. `toolDefaultContract`'s
  advisory `degree` clause therefore reports a tool reached only by a tap or
  the typecheck as one nothing runs. The fix needs a decision on what the
  runner address of a tap/typecheck edge is.
- **Two homes for the gate's tool address.** The seam key and the `runs` row
  both carry it. The key is seam-inbound only (render needs the address
  without computing handler addresses), and the row is what the lock keeps.
- **Default moved for represented harnesses.** Emitted gate commands now run
  `./.temper/node_modules/.bin/temper`, not PATH `temper`;
  `examples/base-harness` was re-emitted. The rendered word has no Windows
  `.cmd` shim, unlike `typecheck_command`, since hooks run under a POSIX shell.
- **Out of reach this tick:** `docs/MIGRATING-0.0.24.md` still shows
  `binary:`, and the dogfood `.temper/` must move to `tool` (both human
  territory).
