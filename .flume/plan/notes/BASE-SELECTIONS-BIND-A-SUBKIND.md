# Base selections bind subkind members; contract = base's + own

Shipped in `gate`, not `engine`. A subkind member is validated against
`compose::inherited_clauses` (its ancestors' own contracts, joined clauses
included, outermost first) plus its own. `kind_selections` now gives a base's
by-kind selection its subkinds' members (`compose::is_kind_or_subkind`), and
a subkind's selection carries only its own clauses, so an inherited set
clause judges the base population once. `main.rs::kind_contract` moved to
`compose::kind_contract`, and gate's embedded loop uses it too.

Gaps:
- A base's member-grain `unique-name` and similar peer predicates decide over
  the subkind's own member slice, not the base's whole population.
- `schema` (`main.rs`) and `explain`'s contracts (`read.rs` ~l.2390) still
  lift a subkind's own clauses alone. A subkind's JSON Schema and narration
  omit the base's clauses.
- Engine `kind` predicate (`wrong_kind`) compares the exact member tag, so a
  `kind: rule` narrowing would refuse a `decision` member.
- Admissibility of inherited clauses runs at the base's dispatch only.
