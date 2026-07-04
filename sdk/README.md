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
- **`emitManifestMembers`**: members → manifest TOML, deterministically
  ordered, **double-emit verified in-process** (law 5's discipline).
- Tests: `pnpm test` (tsc + `node --test`).

## Stated bounds — each a named follow-on slice, never silently faked

- **Byte-parity with the Rust emitter** (`toml_edit` output) is the altitude
  slice's acceptance bar; this serializer emits valid TOML of the same schema.
- **Projection writing** (members → `.claude/**`), **lock stamping**,
  **`fromFile` resolution**, and **mention resolution-checking** are absent
  and fail loud if reached.
- **Publish name/scope** pending the PACKAGING-CHANNELS ruling (needs
  registry credentials); `private: true` until then.
