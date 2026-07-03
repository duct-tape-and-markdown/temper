# Plan state

- **Phase:** reconcile. HEAD d4faece.
- **Last shipped:** the DIRECTIVES wave (DIRECTIVES-PRIMITIVE-PARSE →
  DIRECTIVE-TARGET-CLASSING → REACHABILITY-DIRECTIVE-CLOSURE, drained) plus the
  chore d4faece (`claude-code.memory` adopts at-import). The at-import pipeline is
  live end to end — extraction, target classing, reachability closure — with one
  residual gap in *collection* (below).
- **This tick:** drained the inbox — filed **DIRECTIVE-MEMBERS-ALL-KINDS** (open,
  the one pickable entry). Verified on disk: `collect_directive_members`
  (`src/main.rs:1028`) still hardcodes the skill/rule pair + custom_kinds, so a
  discovered memory member's extracted `at-import` directives never reach
  `graph::classify_directives` — an unbacked `@path` in a CLAUDE.md draws no
  finding. `claude-code.memory` does declare the directive extraction
  (`kinds/claude-code/memory/KIND.md:8`), so only collection misses it. The fix
  mirrors CHECK-MEMBERS-ALL-KINDS: range over `builtin_kind::definitions()` via
  `check::surface_units` + `owns_source`. Re-verified all 4 carried entries against
  disk — every one still accurate (`Primitive` lacks `Fenced`, `Field` flat at
  kind.rs:686; no agent kind/package; root package.json still `temper-flume-harness`;
  no CONTRIBUTING/SECURITY). No rewrites.
- **Pickable now:** DIRECTIVE-MEMBERS-ALL-KINDS (open, edits `src/main.rs` +
  `tests/memory_gate.rs` — disjoint from the 4 deferred/parked entries, which build
  won't pick in parallel). The other 4 stay human-gated: deferred on a missing
  consumer (EXTRACTION-VOCAB-GAPS, AGENT-KIND) or parked on human action
  (PACKAGING-CHANNELS creds; COMMUNITY-DOCS a chain.ts fence-widen).
- **Operational note (accepted, not queued):** the session-start 19
  `requirement.dangling` findings remain a **stale installed binary** — a freshly
  built `./target/debug/temper check .temper` is clean of them. `cargo install
  --path .` clears the stale global.

Plan continues: no — inbox drained, one open entry filed and pickable; hand to
build.
