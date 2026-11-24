# Collection entry judged as its kind: only the host path was broken

- **Member-grain half already held on the base** (now a pin): a `required`
  clause on the entry kind (`invariant.required.test`) fires on a declared
  layout host's entries, committed or `local`, matching GH #62's SDK shape.
  Either #62 was fixed before this tick or the reporter's 0.0.23 npm build
  differed; worth closing the issue against that.
- **The real gap was the host path.** `invariant[*].test` resolved against the
  host's `fields`, which never carried the collection, so the clause held
  vacuously. Fix: `compose::fold_collection_entries` (in
  `kind_units_and_features`, the gate/explain read) seats each layout
  collection's entries under its `member_kind` as an array of leaf objects,
  one layer of sub-collections deep. An empty collection seats nothing,
  matching an unfilled slot; an authored field of the same name wins.
- **Possible adopter break:** `closed-keys` on a layout host now sees the
  collection key. No example or dogfood harness uses it.
- **Debt:** `coverage.checked` reports `invariant (0)` for a `local` layout
  host even when its entries are judged (the findings fire): the embedded
  count misses read-time derived rows.
- Templated (non-layout) hosts' nested members are deliberately not folded;
  the spec states host addressing only for layout entries.
