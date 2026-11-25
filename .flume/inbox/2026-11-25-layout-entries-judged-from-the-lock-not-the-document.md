# check judges a layout collection's entries from the lock's copy, not the document on disk (GH #62)

Reproduced on 0.0.24. A committed layout kind `spec` (`specs/*.md`) with a collection of embedded `invariant` entries (field slots `rule`, `test`) and `clause(required("test"), { severity: "required" })`. Emit, then delete `#### Test` from the document:

- `check` exits 0, and no `root.fresh` fires either;
- both guard edges let the edit through;
- after `emit`, `check` exits 1 (`required field \`test\` is absent`).

The host's own field sections are read off the document at `check`. Its entries are judged from the `[[declaration.nested_member]]` rows the last emit wrote. The `[[declaration.layout_source]]` row carries no content hash, so nothing notices that those rows are stale. One document gets two read paths, and the entries' path is a stored copy of derived state.

Sources: `specs/model/representation.md`, "layout": an entry is "read off the host's layout, source, never projection", and the document "is the authored home, read, never regenerated". `specs/model/pipeline.md`, "Total": a layout document is "the one governed source". `specs/process/engineering.md`: derived state is computed, never stored beside its source. The fix the corpus entails is that `check` and the guard derive a committed layout host's entries from the document at read time, as they already do for its fields and for a `local` member's rows. Re-emitting must not be what makes an edit visible to the gate. If plan finds `pipeline.md`'s "derives its rows in the same pass" reads otherwise, that is a question to raise, not a fingerprint to add. A fingerprint would make every in-place spec edit wait for an emit, which is exactly what adopters who run only `check` don't do.

Acceptance: delete an entry's `#### Test` with no re-emit, and `check` fails on `invariant.required.test` (and on the host-side `required("invariant[*].test")`), with both guard edges agreeing.
