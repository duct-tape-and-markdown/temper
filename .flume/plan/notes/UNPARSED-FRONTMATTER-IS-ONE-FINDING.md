# Unparsed frontmatter is one finding

Shipped. A forgiven member (a `malformed-yaml` leniency kind whose block does not parse)
now draws only `member.non-portable-yaml`, which names the top-level keys the block spells
as read unset. Its clauses that read frontmatter are skipped: per-field, closed-keys,
forbidden-keys, and `when` (`Predicate::reads_frontmatter`).

Observations for plan:
- The unset names are scanned line by line from the block (`frontmatter::authored_keys`),
  because the block does not parse: a top-level `key:` line names a key. A key the author
  never wrote is not named. If a required field is missing as well as the YAML being
  broken, that field is silent until the YAML is fixed.
- The layout arm (`compose::layout_unit`) marks a forgiven document the same way, so a
  forgiven layout document's slot-field clauses are skipped too. No built-in layout kind
  declares `malformed-yaml` today. A custom layout kind that does would lose its slot
  checks along with its frontmatter checks.
- `Unit` gained a `forgiven: bool`, and `engine::validate_as_read` takes the forgiven ids.
  The embedded dispatcher passes an empty set.
- The pin `an agent whose frontmatter yaml does not parse keeps the load fault` was
  already present (`tests/it/agent_kind.rs`). The kept parsed-no-description case gained
  an explicit test in `tests/it/gate_fail_loud.rs`.
