# Plan state

- **Phase:** reconcile after SPEC-FEATURES shipped. Verified on disk:
  `extract::spec_features` (src/extract.rs) projects a `Spec` into `Features`
  (id + body_lines + headings + source_dir), with full tests — the spec
  contract's read-side. SPEC-KIND-GATE is still genuinely unbuilt: `main.rs`'s
  `Check` arm never projects `ws.specs` nor runs a spec contract.
- **Last shipped:** SPEC-FEATURES (32d9ebd / bee0de8).
- **In flight:** nothing; tree clean apart from untracked human artifacts
  (`contracts/spec.toml`, still `??`).
- **The gap reconciled:** dropped SPEC-FEATURES (shipped). SPEC-KIND-GATE stays
  `parked` — a human must commit the untracked `contracts/spec.toml` the gate
  embeds via `include_str!`. With the queue otherwise human-blocked, filed the
  largest fork-free unbuilt gap: DRIFT-DIFF, a read-only first slice of the drift
  engine (`20-surface.md`) — `temper diff` re-scans the harness and compares each
  artifact's source hash to the import baseline. Its three forks
  (`surface-authority`/`yaml-writeback`/`workspace-scope`) are all RESOLVED.
  Inbox empty; no new fork.

Plan continues: no — DRIFT-DIFF is `open` and immediately pickable (fork-free,
new `drift.rs` + thin `main.rs`/`lib.rs` wiring). SPEC-KIND-GATE waits on a human
committing `contracts/spec.toml`, not on more planning. Hand to build.
