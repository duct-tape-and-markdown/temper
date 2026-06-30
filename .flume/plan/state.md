# Plan state

- **Phase:** reconcile after SPEC-KIND-IR shipped. The corpus also gained a new
  human spec, `specs/05-model.md` ("The domain model") — a consolidation/orientation
  file that owns the *relationships* between temper's entities (every entity delegates
  detail to its owning spec). It authorizes no new buildable mechanism; it only
  touches `(model-declaration-format)` (its reflexive model is prose, no format yet).
- **Last shipped:** SPEC-KIND-IR (7befdf9 / a1b67c3). Verified on disk: `src/spec.rs`
  is the full prose-only `Spec` IR (`from_source_file`, `from_surface_dir`,
  `to_meta_document`, provenance + SHA-256), and `pub mod spec;` is wired in `lib.rs`.
- **In flight:** nothing; build drained SPEC-KIND-IR. Tree clean apart from untracked
  human artifacts — `specs/{05-model,15-kinds}.md` and `contracts/spec.toml`.
- **Next (filed):** SPEC-KIND-IMPORT (`open`) — unblocked, IR shipped; then
  SPEC-KIND-WORKSPACE (`open`, disjoint, check.rs only); then SPEC-KIND-GATE
  (`parked` — a human must first commit the untracked, curated `contracts/spec.toml`
  the gate embeds via `include_str!`).
- **Reconciled this tick:** confirmed on disk that `import.rs`/`check.rs`/`extract.rs`/
  `main.rs` carry no spec handling yet — IMPORT/WORKSPACE/GATE all unshipped. Flipped
  IMPORT + WORKSPACE from `blockedBy SPEC-KIND-IR` to `open` (IR now on disk). GATE
  stays `parked`. Inbox empty; `05-model.md` reconciled (no new entry/fork). Forks
  unchanged.

Plan continues: no — the queue is reconciled (IR confirmed shipped on disk; the
remaining spec-kind build-out unshipped, with IMPORT now immediately pickable and
fork-free), `05-model.md` reconciled, inbox empty. Build runs and drains SPEC-KIND-IMPORT.
