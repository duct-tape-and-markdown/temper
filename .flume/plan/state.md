# Plan state

- **Phase:** both greens hold — `check` runs conformance (skill + rule) and
  contract admissibility. The type-pair precondition is now on disk, so the `type`
  field primitive is unblocked and pickable.
- **Last shipped:** TYPED-EXTRACTION (604bf63) — the extractor preserves each
  field's parsed source `Kind` in `FeatureValue` (`extract.rs`: scalar carries
  kind+text, `FeatureValue::kind`); the `extract.rs` stringify shortcut is gone.
- **In flight:** nothing; tree clean.
- **Next (filed, `open`, fork-free):** TYPE-PRIMITIVE — add `Predicate::Type` over
  the closed lattice, parsed in `contract.rs`, decided in `engine.rs` against the
  preserved kind, with the name⇄`Kind` mapping homed in `extract.rs`. Verified
  unshipped on disk: no `Predicate::Type` arm exists.
- **Frontier (fork-free, unfiled — deferred to a follow-on plan tick, decomposed
  once the type primitive lands):** the harness-contract layer (`temper.toml` +
  roles + `verified_by`, 40-composition); `temper schema`; the advisory
  session-start gate + `claude-session-start` reporter; the plugin/`bundle` tree;
  GitHub/SARIF reporters. **Still fork-blocked:** `apply`/`install`
  (`(yaml-writeback)`, `(workspace-scope)`); the model + dependency graph +
  cross-landscape seam (`(model-declaration-format)`); full `pattern`
  (`(regex-crate)`); the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (TYPED-EXTRACTION dropped as shipped,
TYPE-PRIMITIVE unblocked to `open` and verified unshipped), the inbox is empty, and
a pickable entry exists. Build runs and drains it. The harness-contract / schema /
distribution frontier is fork-free but deferred to a follow-on tick so each lands as
small disjoint entries once the type primitive ships — filing it now is speculative.
