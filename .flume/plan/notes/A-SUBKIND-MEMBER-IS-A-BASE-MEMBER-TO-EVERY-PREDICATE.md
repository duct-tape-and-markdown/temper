# A subkind member is a base member to `kind` and `unique-name`

Shipped: `engine::judge` takes an `is_kind(kind, narrowed)` test (gate passes
`compose::is_kind_or_subkind`), so a `kind` narrowing admits subkind members.
Peer clauses (`Predicate::decides_over_peers`, today `unique-name`) are withheld
from each dispatcher's `validate` and judged by `engine::judge_members` over the
kind's by-kind selection, which already binds subkind members.

Residual gap: a `unique-name` nested in a `when` body is not a top-level peer
clause, so it is still judged inside `validate` over the dispatcher's own slice
(`engine::decide`'s `when` arm passes `all` through). Lifting it needs
`judge_members` to route `when` bodies too; no entry names it.

`engine::validate` used standalone (tests) still judges `unique-name` over the
slice it is handed — consistent, since that slice is its whole population.
