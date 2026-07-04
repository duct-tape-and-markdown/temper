# temper-sdk — the authoring face (scaffold)

The typed module library the ratified corpus names as temper's altitude
authoring medium (`specs/intent/00-intent.md`, "the authoring face is a typed
library; the gate reads inert data"; `specs/architecture/20-surface.md`,
"Member carriage"). Authors compose members, genre values, and the assembly
as typed values; `emit` compiles them into the inert manifest — the gate,
CI, and every read verb consume only declared data, offline, no Node.

## What is real in this scaffold

- **The typed vocabulary**: `rule()` / `skill()` / `memory()` /
  `customMember()`, `defineHarness()`, the `md` dedenting prose template,
  `fromFile`, and the genre constructors `decision()` / `law()` / `bound()` /
  `genre()` — TSDoc on every export (tsc is the keystroke channel; hover is
  the guidance channel, `specs/architecture/50-distribution.md`).
- **The manifest schema types**, mirroring the Rust `[[member]]` /
  `[[member.section]]` / `[[member.genre]]` tables the gate reads.
- **`emit`**: the full compile in one deterministic pass — the manifest
  (members → TOML), the `.claude/**` **projection**, and the **lock** whose
  `source_hash`/`emit_hash` fingerprints the drift engine reads; all three
  **double-emit verified in-process** (law 5's discipline). `writeEmit` lands
  them on disk. `emitManifestMembers` remains the manifest-only seam.
- **Body resolution**: `fromFile` assets read in and mentions
  resolution-checked against the harness's declared values, at emit.
- Tests: `pnpm test` (tsc + `node --test`), including byte-parity of the
  manifest, the projection, and the lock fingerprints against real Rust output.

## Stated bounds — each a named follow-on slice, never silently faked

- **Projection covers the built-in projected kinds** (`rule`, `skill`); a
  memory (`CLAUDE.md`) or custom-kind member lands in the manifest but projects
  nowhere and stamps no lock row — the two kinds the Rust projector emits.
- **Publish name/scope** pending the PACKAGING-CHANNELS ruling (needs
  registry credentials); `private: true` until then.
