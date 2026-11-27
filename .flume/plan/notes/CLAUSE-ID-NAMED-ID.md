# CLAUSE-ID-NAMED-ID shipped: clause rows and dial entries key the id `id`

- The lock column was never SDK-written: `drift::ClauseRow` carries it, so the field
  itself is renamed `label` -> `id` (the TS seam binding `ClauseRow.ts` re-blessed with
  it). `contract::Clause.label` (the runtime clause) keeps its name; it is not a
  serialized key.
- `ClauseRow::from_table` still reads a `label`-keyed row as the same `id` (decision 0117's
  class: an older engine's lock is stale, never malformed); the next emit rewrites it.
  Not named in the entry; flagged here so plan can drop or keep it.
- No finding serializes a clause id as a key today: the tap log carries none, SARIF keeps
  `ruleId`. Nothing to rename to `clause`.
- Guidance prose still says "dial this label" in several shipped clauses (builtins.ts,
  assembly.ts); prose, not a key, left alone.
- Human halves still owed: the dogfood `.temper/dial.toml` (`label` -> `id`) and its lock
  re-emit, and a migration-note line (an adopter's dial must re-spell `label` as `id`; the
  old key is refused by closed-keys).
