# Plan state

- **Phase:** roster **selection** now runs in `check` — each `required`
  single-filler role is gated on being filled by exactly one artifact of its kind.
  The next advance is **conformance**: validate each filled role's artifact against
  the role's own contract (`conforms-to`). Fork-free.
- **Last shipped:** ROLE-SELECTION-CONFORM (e021e7a / af8f004). Verified on disk:
  `roster.rs` evaluates each `Role`'s `MatchSelector` over the workspace and reports
  a zero-or-many filler as a conformance error; `main.rs` wires `roster::check` into
  the Check arm behind the optional `temper.toml` layer.
- **In flight:** nothing; pending was empty, tree clean.
- **Next (filed, `open`, fork-free):** ROLE-CONFORMS-TO — run each role's selected
  filler(s) through the role's resolved contract (inline clauses or adopted
  template) via `engine::validate`.
- **Reconciled this tick:** `(workspace-scope)` + `(yaml-writeback)` marked RESOLVED
  in open-questions — the specs settled both (per-project; patch-only write-back) but
  the fork file still listed them open. With `(surface-authority)` they were the last
  forks gating `apply`/`install`, which is now **fork-free**.
- **Frontier (fork-free, unfiled — follow-on ticks):** roster admissibility
  (match/`verified_by`/template resolve, `required` satisfiable); the gitignored
  `temper-local.toml` second layer; `temper schema` (JSON-Schema emit); the advisory
  session-start gate + `claude-session-start` reporter; GitHub/SARIF reporters; the
  `apply`/`install` drift engine (newly unblocked); the plugin tree + `temper bundle`.
  **Still fork-blocked:** the model + dependency graph + cross-landscape seam
  (`(model-declaration-format)`); full `pattern` (held, `10-contracts.md`); the skill
  referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (ROLE-SELECTION-CONFORM confirmed
shipped on disk and dropped; the two spec-resolved forks marked RESOLVED; the
conforms-to gap filed `open` and verified unshipped, since neither `roster.rs` nor
`main.rs` runs any per-role conformance), the inbox is empty, and a pickable entry
exists. Build runs and drains it. The deeper roster-admissibility / schema /
distribution / `apply` frontier is fork-free but left to follow-on ticks so each
lands as a small disjoint entry — filing it now is speculative.
