# Plan state

- **Phase:** both greens hold and the author layer is live — `check` runs
  conformance + admissibility on the *effective* contract (the by-kind floor ⊕ an
  optional `temper.toml`). The harness-contract roster (roles + `verified_by`,
  law 2's interface/trait tier) is the next advance; `(harness-contract-provisioning)`
  is RESOLVED, so it is fork-free.
- **Last shipped:** TEMPER-TOML-LAYER (d7a81e4 / 34a798c) — an optional project-root
  `temper.toml` layered over the by-kind floor (adopt + extend/override + severity
  flip) on `src/compose.rs`, discovered at the invocation root and wired into
  `check` in `main.rs`. Verified on disk.
- **In flight:** nothing; tree clean.
- **Next (filed, `open`, fork-free):** ROLE-ROSTER-PARSE — parse `[role.<name>]`
  tables in `temper.toml` into a typed role model on `AuthorLayer` (parse-only
  foundation). Verified unshipped: `compose.rs::parse` reads only `[kind.<k>]`; no
  role parsing exists (`role`/`verified_by` appear only in doc-comments).
- **Frontier (fork-free, unfiled — follow-on ticks):** role `match`-selection +
  `required` single-filler conformance; `verified_by` referential admissibility;
  the gitignored `temper-local.toml` second layer; `temper schema` (JSON-Schema
  emit); the advisory session-start gate + `claude-session-start` reporter;
  GitHub/SARIF reporters; the plugin tree + `temper bundle`. **Still fork-blocked:**
  `apply`/`install` (`(yaml-writeback)`, `(workspace-scope)`); the model +
  dependency graph + cross-landscape seam (`(model-declaration-format)`); full
  `pattern` (`(regex-crate)`); the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (TEMPER-TOML-LAYER confirmed shipped
on disk and dropped; the role-roster parse foundation filed `open` and verified
unshipped), the inbox is empty, and a pickable entry exists. Build runs and drains
it. The rest of the role / schema / distribution frontier is fork-free but left to
follow-on ticks so each lands as a small disjoint entry once the parse foundation
ships — filing it now is speculative.
