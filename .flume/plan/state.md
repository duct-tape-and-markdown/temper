# Plan state

- **Phase:** both greens hold — `check` runs conformance (skill + rule, dispatched
  by kind) and admissibility, and the `type` primitive over the closed kind lattice
  is now live. The harness-contract layer (law 2's author-declared half,
  `40-composition`) is the next advance; `(harness-contract-provisioning)` is
  RESOLVED, so it is fork-free.
- **Last shipped:** TYPE-PRIMITIVE (fac4c3b) — `Predicate::Type` parsed in
  `contract.rs` over the closed lattice, decided in `engine.rs` against each field's
  preserved source `Kind`, with the name⇄`Kind` table homed in `extract.rs::Kind`.
- **In flight:** nothing; tree clean.
- **Next (filed, `open`, fork-free):** TEMPER-TOML-LAYER — an optional project-root
  `temper.toml` layered over the by-kind built-in floor (adopt + extend/override +
  severity flips) in `check`, on a new `src/compose.rs`. Verified unshipped on disk:
  no `temper.toml` loading exists; `check` validates against the embedded floor only
  (`main.rs`).
- **Frontier (fork-free, unfiled — decomposed by follow-on plan ticks):** the role +
  `verified_by` roster in `temper.toml` (the interface/trait tier; blocks on
  TEMPER-TOML-LAYER); the gitignored `temper-local.toml` second layer; `temper schema`
  (JSON-Schema emit); the advisory session-start one-shot gate + `claude-session-start`
  reporter; GitHub/SARIF reporters; the plugin tree + `temper bundle`. **Still
  fork-blocked:** `apply`/`install` (`(yaml-writeback)`, `(workspace-scope)`); the
  model + dependency graph + cross-landscape seam (`(model-declaration-format)`);
  full `pattern` (`(regex-crate)`); the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (TYPE-PRIMITIVE confirmed shipped on
disk and dropped; the harness-contract foundation filed `open` and verified
unshipped), the inbox is empty, and a pickable entry exists. Build runs and drains
it. The rest of the harness-contract / schema / distribution frontier is fork-free
but left to follow-on ticks so each lands as a small disjoint entry once the
`temper.toml` foundation ships — filing it now is speculative.
