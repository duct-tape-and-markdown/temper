# Plan state

- **Phase:** the harness-contract roster now *parses* onto `AuthorLayer`
  (`compose.rs`: `Role`/`RoleContract`/`MatchSelector`, `roles()`), but nothing
  consults it — the next advance is **conformance**: select each role's filler and
  gate `required` single-filler roles. Fork-free (`(harness-contract-provisioning)`
  RESOLVED).
- **Last shipped:** ROLE-ROSTER-PARSE (eb58d30 / 2b59df5) — `[role.<name>]` tables
  parse into a typed roster on `AuthorLayer`. Verified on disk: `compose.rs` carries
  `Role`, `parse_role`, the match/contract parsers, and `roles()`; `main.rs` never
  reads them yet.
- **In flight:** nothing; tree clean (pending was empty).
- **Next (filed, `open`, fork-free):** ROLE-SELECTION-CONFORM — evaluate each
  `Role`'s `MatchSelector` over the workspace and report a `required` single-filler
  role filled by zero or many artifacts as a conformance error, wired into `check`.
- **Frontier (fork-free, unfiled — follow-on ticks):** role `conforms-to` its
  contract; `verified_by` referential admissibility; the gitignored
  `temper-local.toml` second layer; `temper schema` (JSON-Schema emit); the advisory
  session-start gate + `claude-session-start` reporter; GitHub/SARIF reporters; the
  plugin tree + `temper bundle`. **Still fork-blocked:** `apply`/`install`
  (`(yaml-writeback)`, `(workspace-scope)`); the model + dependency graph +
  cross-landscape seam (`(model-declaration-format)`); full `pattern`
  (`(regex-crate)`); the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (ROLE-ROSTER-PARSE confirmed shipped on
disk and dropped; the selection/conformance gap filed `open` and verified unshipped,
since `main.rs` never reads `layer.roles()`), the inbox is empty, and a pickable entry
exists. Build runs and drains it. The deeper role / schema / distribution frontier is
fork-free but left to follow-on ticks so each lands as a small disjoint entry once
selection ships — filing it now is speculative.
