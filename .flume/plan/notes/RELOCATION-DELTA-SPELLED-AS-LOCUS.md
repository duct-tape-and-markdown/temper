# Relocation delta spelled as the locus: names chosen, halves owed

Spellings landed (spec names none; derived from 0114's "locus stays the one
noun" and its rejection of `governs` as naming only the `at` form):
SDK delta `relocate/extend(base, { locus: { root, glob } })`; lock columns
`locus_root`/`locus_glob`; finding id `kind.locus-collision`.

Read tolerance: `KindFactRow::from_table` (src/drift.rs) falls back to
`governs_root`/`governs_glob` when the locus columns are absent, the same
prior-version skew doctrine as the bare-string glob read. Needed because the
repo's own `.temper/lock.toml` is outside build's writable paths and the
self-host test reads it; emit writes only the locus columns. Retire the
fallback once no committed lock carries the prior names, if plan wants it gone.

Human halves owed: re-emit `.temper/` (lock columns move; `.temper/kinds.ts`
has no `governs:` delta today); migration-note lines for the delta key, the
two lock columns and the finding id.

Not renamed (internal, never author-facing): Rust `CustomKind::governs` and
`kind::Governs`, and the many internal docs that cite them (import, install,
gate, compose, extract, glob). Several `explain`/remedy strings that spelled
`governs` as a noun now say locus. A later tidy could rename the Rust field
for one-name consistency; not done here.
