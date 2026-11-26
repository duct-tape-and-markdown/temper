# AGENT-NAME-REFUSES-A-COLON-AND-A-LEADING-DASH

Shipped as entered. Observations for plan:

- Label collision the entry did not name: two `allowed_chars` clauses on one
  field compiled one address (`agent.allowed_chars.name`), a malformed lock.
  Resolved in the `when_label_field` manner: `contract::allowed_chars_label_field`
  qualifies the field segment with the non-default form after `=`
  (`name=negated`, `name=leading+negated`); a plain whitelist keeps the bare
  field, so every existing label is byte-unchanged. Two default-form charsets on
  one field still collide; that is a redundancy the author merges.
- Schema: a second `pattern` on one property now composes through `allOf`
  (the shape precedent) instead of overwriting. Leading form projects as
  `^(?:[class]|$)`, negated as `[^class]`.
- `src/read.rs` needed no edit: no read path renders a charset. But the
  bundled reference / `explain` render both agent clauses identically as
  "`allowed_chars` on `name`": the guidance tells them apart, the line does
  not. A candidate: render the charset form (or the label) on the clause line.
- `src/bundled_reference/kind-agent.md` and `sdk/test/builtins.test.ts` moved
  too (not in `files.edit`).
