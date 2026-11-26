# Nested guard is refused: pinned at the lock seam

The property holds. A `when` row whose body carries a second `when` row reaches
`engine::when_restrictions` (src/engine.rs ~297), which refuses it by name with
"`when` guard body cannot nest another `when` clause"; `check` fails loud. The
pin drives that end to end through an emitted lock (tests/it/lock_declaration_rows.rs,
`a_nested_guard_is_refused`), beside the existing self-guarding-row refusal at lift.
