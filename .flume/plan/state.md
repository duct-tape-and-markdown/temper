# Plan state

- **Phase:** reconcile after SPEC-KIND-IMPORT + SPEC-KIND-WORKSPACE shipped. Verified
  on disk that two adjacent frontiers are *also* fully shipped: the `type` primitive
  (`Predicate::Type` in `contract.rs` + `engine.rs`) and the whole
  `temper.toml`/roster/`verified_by` layer (`compose.rs` layering + `roster.rs`
  selection/conformance/admissibility, wired into `check` in `main.rs`).
- **Last shipped:** SPEC-KIND-IMPORT + SPEC-KIND-WORKSPACE (2f08210). On disk:
  `import.rs` scans `<harness>/specs/*.md` into `<into>/specs/<name>/`, and
  `check.rs`'s `Workspace` carries `pub specs: Vec<Spec>`.
- **In flight:** nothing; tree clean apart from untracked human artifacts
  (`specs/{05-model,15-kinds}.md`, `contracts/spec.toml`).
- **The gap reconciled:** `check` validates skills + rules + the roster but **never
  the specs** — `ws.specs` is loaded yet never extracted or checked. That gap is the
  spec-kind gate, now split into SPEC-FEATURES (the pure extractor, `open`/pickable)
  and SPEC-KIND-GATE (the `check` dispatch + `contracts/spec.toml` embed, still
  `parked` — a human must commit the untracked spec.toml the gate embeds via
  `include_str!`). Closed the resolved forks whose dependents shipped:
  `(field-type-lattice)` and `(harness-contract-provisioning)` are now build-done
  (notes updated in open-questions). Inbox empty; no new fork.

Plan continues: no — the queue is reconciled and SPEC-FEATURES is `open` and
immediately pickable (fork-free, extract.rs only). Hand to build; the parked GATE and
the references-resolve / decisions-name-alternatives clauses all wait on the human
committing `contracts/spec.toml`, not on more planning.
