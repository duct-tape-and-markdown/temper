# Plan state

- Spec derived through: 0aa9e62
- Audited through: 8ccd01d
- Residue swept through: 8ccd01d
- This tick: INBOX DRAIN. Routed the one note (builtins update ritual, re-filed
  under its own spec per at cda03a6) into two dispositions. (1) BUILTINS-CITE-REFRESH
  entry (open, pickable) — the decidable, no-new-predicate remainder: re-check every
  builtins.ts cite dated 07-07..07-10 against a re-fetch, re-verify
  DOCUMENTED_HOOK_EVENTS/MCP_TRANSPORTS, and the skill forbiddenKeys review; per
  builtins.md "The clauses live in code". Verified on disk: clause `cite`/`guidance`
  are ClauseRow columns (34 rows in src/builtin_lock.toml), so the entry moves the
  frozen lock; doc-comment retrieval dates are TS-only. (2) open fork
  (builtins-coverage-predicates) — the two candidates that reach the closed predicate
  vocabulary (rule glob-validity, agent tools-must-resolve); verified src/contract.rs's
  Predicate enum carries neither. Inbox drained. Grown-frontmatter modeling (audit
  #2/#6) left untouched — out of the note's per; deriving it from the audit doc would
  read a horizon doc as intent. Cursors unmoved: no spec delta past 0aa9e62; post-ship
  window 8ccd01d..HEAD = cda03a6 (inbox) + 1d251f6 (docs), neither touches src/tests/sdk.
- Queue: BUILTINS-CITE-REFRESH (open, pickable) → next; PACKAGING-CHANNELS-REMAINDER
  (parked — John's Apple notarizing + v0.1 tag).

Plan continues: no — inbox drained, no spec delta, post-ship window carries no
src/tests/sdk change; BUILTINS-CITE-REFRESH is pickable so build takes over.
