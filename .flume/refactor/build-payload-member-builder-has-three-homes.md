## Surface

A bare `PayloadMember` builder — kind, name, host, body, no fields — is now
spelled in three places:

- `tests/it/common/mod.rs:1124` `payload_member(kind, name, host, body)` —
  added this tick, the one home `specs/process/engineering.md` ("One job, one
  home") names for shared test scaffolding.
- `tests/it/emit.rs:589` `plain_member(kind, name)` — the same literal with
  `host: None` and the body hardcoded to `"# Body\n"`; 8 call sites.
- `tests/it/json_document_format.rs:557` `titled_member(kind, name, host)` —
  the same literal with an empty body and one `title` field derived from
  `name`; 3 call sites.

Both per-file copies were already there when the common builder was added; the
entry's fence (`src/drift.rs`, `tests/it/prose_include.rs`,
`tests/it/common/mod.rs`) could not reach either file, so the fold was not
takeable this tick.

## Observed at

e82ea08b (HEAD when observed)

## Suggested consolidation

Keep `common::payload_member`. `plain_member(kind, name)` becomes
`common::payload_member(kind, name, None, "# Body\n")` at its 8 sites;
`titled_member` keeps its one-field shape but builds off the common builder
(spread the returned member and push the `title` field) rather than
re-spelling the struct literal.
