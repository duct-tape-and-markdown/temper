# Plan state

- **Phase:** reconcile. HEAD 4e1a811.
- **Last shipped:** WEDGE-FACT-FLOOR (build 3cdd3e6 — directive-target classing
  hoisted to the FLOOR tier, verified on disk src/main.rs:694-709: computed once
  and read by both floor and assembly, so a no-temper.toml CLAUDE.md `@import` now
  classes) and AUTHORITY-POSTURE-PARSE (build fae7292 — the assembly's `authority`
  posture parses onto the composed layer; `layer.authority()` at src/compose.rs:761,
  closed `shared`/`surface` vocab, absent ⇒ Shared).
- **This tick:** unblocked the two downstream entries whose upstreams just shipped.
  IMPACT-VERB (was blockedBy WEDGE-FACT-FLOOR) and INSTALL-GUARD-ARTIFACTS (was
  blockedBy AUTHORITY-POSTURE-PARSE) both flip to `open` — verified neither has
  shipped (no `impact` in read.rs/main.rs; no authority/guard in install.rs) and
  their blast radii are disjoint (read.rs/main.rs/read_verbs.rs vs
  install.rs/install.rs test), so they run parallel-safe. Inbox empty (no drain).
  Re-verified the 5 carried parked/deferred entries against disk — all still
  accurate (no `Fenced`/flat `Field` at kind.rs:616/686; BUILTIN_KINDS still
  `["skill","rule"]`; package.json still `temper-flume-harness`/private; no
  CONTRIBUTING/SECURITY; no `ignore` crate).
- **Pickable now:** IMPACT-VERB (src/read.rs + src/main.rs + tests/read_verbs.rs)
  and INSTALL-GUARD-ARTIFACTS (src/install.rs + tests/install.rs) — disjoint,
  parallel-safe. WALK-IGNORE-DISCIPLINE + the 4 deferred/parked entries stay
  human-gated.
- **Operational note (accepted, not queued):** the session-start 19
  `requirement.dangling` findings remain a **stale installed binary** — a freshly
  built `./target/debug/temper check .temper` is clean. `cargo install --path .`
  clears the stale global.

Plan continues: no — inbox drained, two disjoint open entries pickable; hand to build.
