# Plan state

- **Phase:** the harness-contract **role tier** is complete in `check` — selection
  (each `required` single-filler role filled by exactly one artifact) and `conforms-to`
  (each filler run through the role's resolved contract) both ship. The next advance is
  **roster admissibility**: the roster itself checked against the definition. Fork-free.
- **Last shipped:** ROLE-CONFORMS-TO (1c1c8f7 / b676a02). Verified on disk: `roster.rs`
  `conformance()` resolves each role's contract and runs its selected filler(s) through
  `engine::validate`, retagged under `role.conforms-to`; `main.rs` wires it into the
  Check arm. (Also confirmed shipped & drained: the `type` primitive — `engine.rs:185`
  decides `Predicate::Type` over the preserved source scalar kind — closing
  `(field-type-lattice)`'s TYPED-EXTRACTION + TYPE-PRIMITIVE.)
- **In flight:** nothing; pending was empty, tree clean apart from one untracked human
  artifact, `contracts/spec.toml` (a spec-landscape contract — not plan's to commit;
  routed to the new `(spec-landscape-kind)` open question).
- **Next (filed, `open`, fork-free):** ROSTER-ADMISSIBILITY — `roster::admissibility`
  checks each role's selector resolves, a `required` single-filler role is satisfiable
  (known artifact kind), its contract is admissible (template resolves + `engine::admissibility`),
  and its `verified_by` path resolves. The follow-on `b676a02`/`roster.rs:20` deferred.
- **Reconciled this tick:** queue was empty (build drained ROLE-CONFORMS-TO); no entry to
  rewrite. Inbox empty. Verified roster admissibility genuinely unshipped (`roster.rs` has
  no `admissibility` fn — only `engine.rs` clause-level). Filed the spec-as-kind direction
  as `(spec-landscape-kind)` rather than a pending entry — the corpus doesn't yet authorize
  a `spec` kind, the `section_contains` primitive, or the spec reference syntax.
- **Frontier (fork-free, unfiled — follow-on ticks, one small disjoint entry each):**
  the gitignored `temper-local.toml` second layer; `temper schema` (JSON-Schema emit); the
  advisory session-start gate + `claude-session-start` reporter; GitHub/SARIF reporters; the
  `apply`/`re-add`/`install` drift engine; the plugin tree + `temper bundle`.
  **Still fork-blocked:** spec-as-artifact-kind + its two primitives (`(spec-landscape-kind)`);
  the declared model + dependency graph + cross-landscape seam (`(model-declaration-format)`);
  full `pattern` (held, `10-contracts.md`); the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (ROLE-CONFORMS-TO confirmed on disk and
dropped; `type` confirmed shipped; roster admissibility confirmed unshipped and filed
`open`, fork-free), the inbox is empty, the untracked spec contract is routed to an open
question, and a pickable entry exists. Build runs and drains it. The deeper frontier is
fork-free but left to follow-on ticks so each lands as a small disjoint entry.
