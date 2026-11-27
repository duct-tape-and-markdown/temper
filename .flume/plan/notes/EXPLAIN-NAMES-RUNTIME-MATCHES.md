# EXPLAIN-NAMES-RUNTIME-MATCHES: runtime match strand shipped

`explain` on a member now adds a "what the runtime matches its strings
against" strand (`src/read.rs`, `runtime_match_strand`), joined in
`explain_target` beside the load strand.

- Settings / settings-local: each `Read(...)`/`Edit(...)` rule in
  `permissions.allow|ask|deny` states its anchor (`//` root, `~/` home,
  `/` settings source = primary working directory, else cwd). Cited:
  code.claude.com/docs/en/permissions, retrieved 2026-11-26.
- Agent: a `name` in the documented built-in set (Explore, Plan,
  general-purpose, claude, statusline-setup, claude-code-guide,
  web-fetch). Cited: code.claude.com/docs/en/sub-agents, retrieved
  2026-11-26. The docs state the override **only for Explore**, so the
  other names read "shares the name ... (override documented for
  Explore alone)". If the corpus wants a uniform override claim, it
  needs a probe first.

Observations:
- The kind dispatch is by built-in kind name (`settings`,
  `settings-local`, `agent`) in read.rs; a custom subkind of those
  kinds does not get the strand. There is no kind-level fact today that
  carries "this field is a permission-rule list".
- Only the anchor is narrated. The docs' depth rules (a single-segment
  relative pattern like `src/**` matches at any depth in deny/ask but
  only at cwd in allow; bare filenames match at any depth) are not
  narrated yet. They are a natural widening of the same strand.
- `Write(...)`/`Glob(...)`/`Grep(...)` path rules never match per the
  docs; the strand says nothing about them, which a stranger may
  misread as "nothing to see".
