# Plan state

- **Phase:** the **rule** artifact kind — second instance of the engine, toward
  self-hosting (`specs/20-surface.md`, "Artifact kinds & contract selection").
- **Last shipped:** RULE-IR + RULE-CONTRACT-TEST (55e9f75). Verified on disk:
  `src/rule.rs` is the typed rule IR (`from_source_file`/`to_meta_document`/
  `from_surface_dir`, body filename `RULE.md`), `extract::rule_features` projects
  it, `contracts/rule.toml` + `tests/rule_contract.rs` pin the clause vector.
- **In flight:** nothing committed; working tree clean. `import` still scans only
  `<harness>/skills/` (import.rs:97), `Workspace` carries only `skills`
  (check.rs:34), `main.rs` validates only the skill contract (main.rs:77-81) —
  the rule kind is modelled but not yet wired through import/check.
- **Next:** RULE-IMPORT is now pickable (its RULE-IR gate is satisfied → `open`):
  teach `import`/`Workspace` the rule surface. Then RULE-CHECK (blockedBy
  RULE-IMPORT) dispatches `check` by kind and lands the self-host proof on
  temper's own `.claude/rules/`.

Plan continues: no — shipped entries dropped, the only reconciliation was flipping
RULE-IMPORT to `open` (RULE-IR landed), inbox is empty, and a pickable `open`
entry leads the chain. Build drains from here.
