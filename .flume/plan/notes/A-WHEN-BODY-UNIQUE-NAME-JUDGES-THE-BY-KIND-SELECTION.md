# when-nested unique-name judges the by-kind selection

Shipped as a contract split, not a change to `decide`'s `when` arm: new
`engine::split_peer_clauses` partitions a kind's clauses into (slice, peers),
splitting a `when` so each side keeps the guard over its share of the body.
`gate::two_greens_dispatch` keeps the slice half; `judge_members` judges the
peer half over the by-kind selection. The `when` arm itself is untouched, so
standalone `engine::validate` still judges a nested unique-name over the slice
it is handed, as the entry notes asked.

Observed: emit restamps body-row labels from the guard (`rule.when.globs=*.rs.
unique-name`), so a test's hand-set body label never reaches output; the test
matches on the `.unique-name::<member>:` suffix.
