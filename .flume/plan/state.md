# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox drained (routed to
  SESSION-START-CHECK-SURFACE); no new fork.
- **Last shipped (trunk):** EMBED-BUILTIN-PACKAGES (697a18c/1635db5) — the built-in
  std-lib is embedded from `packages/{skill.anthropic,rule.anthropic}/PACKAGE.md` via
  build.rs; the `contracts/` TOML loader is retired. HEAD 1635db5; tree clean.
- **Verified on disk this tick:** `packages/{skill.anthropic,rule.anthropic}/PACKAGE.md`
  embedded (`builtin.rs` `include!` of `$OUT_DIR/builtin_packages.rs`, main.rs schema
  floors at 304/305); `contracts/*.toml` no longer loaded by src (only comment/fixture
  refs). `Cargo.toml` still `license = "MIT"`; `.github/CONTRIBUTING.md` present; no
  LICENSE-*/AGENTS.md. temper's own surface live: `temper.toml` + `.temper/` (specs kind,
  spec package, rules satisfying both requirements).
- **This tick:** dropped EMBED (shipped); drained the inbox FOLLOW-ON into a new `open`
  entry SESSION-START-CHECK-SURFACE — session-start (main.rs:377) always fresh-imports,
  discarding `satisfies` recognition, so temper's own two filled requirements report as
  blocking on clean input (the law-3 false positive the SessionStart hook shows now);
  spec ac4de6a pins the surface-present dispatch. Corrected AGENT-KIND's stale main.rs
  serialization ref (EMBED → SESSION-START-CHECK-SURFACE).
- **Pickable now (3 disjoint / parallel-safe):** SESSION-START-CHECK-SURFACE (main.rs +
  tests/session_start.rs), OFFERING-LICENSE (Cargo.toml + LICENSE-*), AGENTS-MD
  (AGENTS.md). Parked: PACKAGING-CHANNELS (human release creds). Deferred: AGENT-KIND
  (priority). Forks: read-verbs + KIND-* remain RESOLVED-but-unfiled decision records;
  read-verbs fileable now that the surface-language migration (through EMBED) has landed.

Plan continues: no — the inbox is drained, EMBED dropped as shipped, and three disjoint
`open` entries are pickable (session-start fix top, being a live false positive). Building
drains it.
