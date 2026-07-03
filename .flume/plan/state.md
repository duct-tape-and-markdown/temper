# Plan state

- **Phase:** reconcile. HEAD ec479ee.
- **Last shipped:** DIRECTIVE-MEMBERS-ALL-KINDS (build 28df927 — `collect_directive_members`
  now ranges over `builtin_kind::definitions()`, verified on disk src/main.rs:787),
  plus the spec/dogfood commits ratifying the authority posture + impact verb (ec479ee)
  and joining temper's own CLAUDE.md/AGENTS.md to the managed surface (83bf769).
- **This tick:** dropped DIRECTIVE-MEMBERS-ALL-KINDS (shipped). Drained the 5-line inbox
  into four new entries: WEDGE-FACT-FLOOR (open, hoist directive classing to the floor
  tier — verified the classing still sits inside `if let Some(layer)` at src/main.rs:694,
  so a no-temper.toml CLAUDE.md `@import` is silent), AUTHORITY-POSTURE-PARSE (open, parse
  `authority` on the assembly, mirroring the `[reachability]` parse in src/compose.rs),
  IMPACT-VERB (blockedBy WEDGE-FACT-FLOOR — shares src/main.rs), INSTALL-GUARD-ARTIFACTS
  (blockedBy AUTHORITY-POSTURE-PARSE). Parked WALK-IGNORE-DISCIPLINE (no written
  ignore-discipline Decision yet + needs the `ignore` crate sanctioned). Re-verified the 4
  carried deferred/parked entries against disk — all still accurate (no `Fenced`/flat
  `Field`; no agent in BUILTIN_KINDS; package.json still `temper-flume-harness`; no
  CONTRIBUTING/SECURITY).
- **Pickable now:** WEDGE-FACT-FLOOR (src/main.rs + tests/memory_gate.rs) and
  AUTHORITY-POSTURE-PARSE (src/compose.rs + tests/temper_toml.rs) — disjoint files,
  parallel-safe. IMPACT-VERB + INSTALL-GUARD-ARTIFACTS unblock when their upstreams ship.
  WALK-IGNORE + the 4 carried entries stay human-gated.
- **Operational note (accepted, not queued):** the session-start 19 `requirement.dangling`
  findings remain a **stale installed binary** — a freshly built `./target/debug/temper
  check .temper` is clean. `cargo install --path .` clears the stale global.

Plan continues: no — inbox drained, two disjoint open entries pickable; hand to build.
