<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- **"The SDK is the product" is RATIFIED** (`specs:` 71d0d30; pre-state =
  `manifest-era` tag; record D1–D6:
  claude.ai/code/artifact/5ef1905d-a4f1-4fd0-b553-3b3a1a9a7b1f). Expected
  reconcile, agreed with John: (1) **No demolition entries** — every
  engine-side retirement (document carriage, the committed temper.toml,
  the manifest gate path, the copy tree, the Rust-side emitters) is
  evidence-gated behind the dogfood migrating onto the SDK, each a later
  per-piece ceremony (D6). The committed manifest/lock machinery keeps
  working as-is until then. (2) **What CAN file now, in-fence:**
  (a) SDK-EMIT-REFUSALS — emit refuses on declare-side failures (dangling
  join, unresolved mention, unfilled required requirement; 20-surface,
  "Emit refuses before it writes"); sdk/** + its tests; (b) CONTRACT-DIR —
  promote the SDK byte-parity fixtures + Rust emitter/extraction goldens
  into a shared contract/ directory both test suites consume, with schema
  generation Rust-first (schemars → JSON Schema 2020-12; ts-rs types;
  50-distribution, the contract fixtures) — note contract/** needs a fence
  entry first (the interactive session will widen chain.ts on request);
  (c) SDK-ASSEMBLY-ARTIFACTS — emit writes the locusless assembly facts
  (bindings, roster) as small committed temper-owned artifacts
  (20-surface, the assembly bullet; the engine's read side stays on the
  existing manifest path until its sunset — additive, not a swap).
  (3) The genre projection carrier is a NAMED OPEN (20-surface, the fence
  Decision) — no entries against it; the adoption pilot decides.
  (4) The dogfood migration itself (temper.config.ts for this repo) is a
  human/session ceremony, never an entry.

- **toml_edit output style is version-unstable — treat bumps as contract
  events** (interactive session, 07-03, web-verified: toml_edit CHANGELOG
  documents a breaking default-output change at 0.22.25, 2025-04-25 —
  "Reduced escaping in strings"; three earlier style-churn precedents). The
  SDK's byte-parity emitter (272b4f4) mirrors 0.22.27 behavior, so any
  future toml_edit bump is a latent silent parity break. Standing rule: a
  toml_edit version bump entry must re-run the SDK byte-parity fixtures and
  reconcile both sides in the same entry — never bump-and-ship. (The
  structural retirement of this tax — single-writer-per-format — rides the
  TS-primary ceremony, human-gated.)
