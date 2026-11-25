# Relocation keeps reached exclusions; SDK now duplicates the engine's reach test

Shipped. `relocatedLocus` (sdk/src/kind.ts) now filters the base exclusions: it keeps each one the new template still matches or reaches beneath, and drops the rest.

Duplicate surface: the SDK had no placement-template matcher, so `templateMatches` / `templateReachesBeneath` in sdk/src/kind.ts re-spell src/kind.rs's `template_reaches_beneath` and the globset match, limited to the placement-template subset. These are two implementations of one rule on either side of the Rust/TS seam. Nothing pins them to agree. The engine's `excluded_from_row` is still the authority at load. A cross-seam pin would be worth adding (a Rust test loading an SDK-emitted relocation row). Today the SDK tests only assert the emitted `exclude` column.

Still open: the spec's "an author may add exclusions to a built-in through its relocation". `KindRelocation.governs` still carries only root+glob. No entry named it this tick.
