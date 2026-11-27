# Snake-case predicate keys normalize at decode; clause ids stay as committed

`contract::predicate_from_row` now matches on `row.predicate` with `_` mapped
to `-`, so a v0.0.25-era lock row spelling `min_len` lifts as `min-len`;
`max_lines` maps to `max-lines`, which this engine never named, and still
refuses (pin green). The `when` guard key rides the same decode by recursion.

Not normalized: the `ClauseRow.predicate` string itself (drift's TOML read
keeps it verbatim) and the clause **id**. Labels are opaque to the engine, so
an old lock's finding prints `skill.min_len.description` until the next emit
rewrites both canonically. Consequence worth weighing: a dial addressing the
kebab label (`skill.min-len.description`) will not reach that clause until the
re-emit. Any other consumer matching `row.predicate` as a string (today only
`drift::stamp_clause_label`, which runs at emit on payload rows, not on lock
reads) would see the snake spelling; if a future read-side site keys on it,
normalize at `ClauseRow::from_table` instead.
