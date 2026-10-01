## Surface

`COLLECTION-ENTRY-SHAPE-IS-A-DECLARED-COLUMN`'s fence is one file short.
Making `CollectionAddressRow::entry_shape` a required `String` breaks a
construction site the entry's `files[]` never named:

- `tests/it/hook_kind.rs:848` — `collides_at_hooks_event(name, entry_shape:
  Option<&str>)` builds the row with `entry_shape: entry_shape.map(str::to_string)`,
  and `:886` passes `None` for the colliding `custom_hook` kind
  (`two_kinds_at_the_same_collection_address_trip_collision_loud`). Both the
  helper's parameter type and that one call site must change; the collision
  the test judges is the *address pair*, so any declared shape serves —
  `"group-array(hooks;matcher)"` on both sides keeps the two rows diverging
  only on name, which is what the helper's doc comment already claims.

That is the complete out-of-fence list, taken from `cargo test --no-run
--no-fail-fast` with the column already made total. Every other breakage
(`tests/it/common/mod.rs:1227`, `tests/it/check_cost.rs:1046,1419,1427`,
`tests/it/read_verbs.rs:1526`, `tests/it/lock_declaration_rows.rs:103,118,
1690,1720,1746,1782,1866`) lands inside the fence as scoped. The `src/` half
(`src/kind.rs`'s fabrication deleted, `src/drift.rs`'s column made required
on the row, the table writer and `req_str` reader) compiles clean and the
three cited provider constants keep their consumers in `src/json_manifest.rs`,
`src/install.rs` and `src/builtin_kind.rs`.

## Observed at

ad372b90 (HEAD when observed).

## Suggested consolidation

Re-scope the entry with `tests/it/hook_kind.rs` added to `files[].edit`:
"`collides_at_hooks_event` (848) takes `&str` instead of `Option<&str>`;
the `custom_hook` call site (886) declares `group-array(hooks;matcher)`
like its colliding twin — the collision under test is the address pair,
not the shape." Nothing else about the entry moves.
