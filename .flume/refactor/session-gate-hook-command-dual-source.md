## Surface

The gate hook command (the fail-loud PATH-resolvability preamble + the
`temper check`/`temper guard` invocation) lives in two homes that must stay
byte-identical but have no compiler tie across the Rust/TS boundary:

- `src/install.rs:96` `SESSION_START_COMMAND` and `src/install.rs:148`
  `GUARD_COMMAND` — what `install` places and `gate_installed`
  (`src/install.rs:530`) checks the live `settings.json` against.
- `.temper/hooks.ts:9` `failLoud` + `hook_sessionStart`/`hook_guard` — what
  `emit` projects into the dogfood's `.claude/settings.json`.

Cost paid this session: finding 3 (e46ac67) changed the Rust constant as a
`build:` entry; build's fence forbids `.temper/`, so hooks.ts kept the bare
form. `gate_installed` then compared the bare emitted hook against the guarded
constant and reported drift **every session** — a red gate `temper install`
could not clear (emit re-asserts the harness's form). It read as a version
skew, not a divergence, and burned a full investigation before the two-home
mismatch surfaced. bdf6be3 mirrored the string by hand; the next edit to the
wrapper re-opens the same silent trap.

## Observed at

bdf6be3 (HEAD when observed).

## Suggested consolidation

Root cause is double-ownership: for a **represented** project the harness
declares its own gate hooks (emit owns them), yet `gate_installed`'s
`represented` branch (`src/install.rs:534`) still checks against the hardcoded
`install.rs` constant — the *unrepresented* on-ramp default. Preferred home:
for a represented project, `gate_installed` defers to the harness's declared
hooks (the emit projection / seam), and `install` does not re-place hooks the
harness already owns; the constants stay the default only for the unrepresented
`place_settings_only` path. That removes the second source, so this drift class
is structurally impossible. Likely a `specs/distribution.md` touch (install's
ownership boundary for represented vs unrepresented) — may want an open-question
if the ownership rule is unsettled. Minimal interim if the structural change is
deferred: a test asserting the dogfood's emitted gate-hook commands equal
`SESSION_START_COMMAND`/`GUARD_COMMAND`, so a future divergence fails the
afterMerge gate loudly instead of going silent until a session opens.
