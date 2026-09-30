## Surface

`HOOK-AUTHORS-ITS-HANDLERS` moved `hook()`'s authoring onto the group's `hooks`
array, so the one thing still feeding the write face a **flat** handler triple is
the corpus's own stale spelling — which `EVERY-AUTHORED-HOOK-SPELLS-ITS-HANDLERS`
is scoped to retire. Two surfaces outlive that: the tolerance branch and the doc
naming this entry as future work, both outside this entry's fence.

- **`src/json_manifest.rs:729`** — `hook_matcher_group`'s doc: *"The second is what
  the SDK's `hook()` authors today — the surface HOOK-AUTHORS-ITS-HANDLERS moves
  onto the array — and until it moves, both spellings reach this one face."* False
  as of this commit: `hook()` authors the array, and `Hook` no longer types a
  handler key at all.
- **`src/json_manifest.rs:1166`** — the unit test's comment, same claim: *"the shape
  `hook()` authors until HOOK-AUTHORS-ITS-HANDLERS moves it."*
- The **branch itself** (`src/json_manifest.rs:749-751`, the `residue`→one-handler
  nest) still has live callers: `tests/it/gauntlet.rs:132,141`,
  `tests/it/settings_kind.rs:39,61` and the dogfood's `.temper/hooks.ts` all still
  spell handler keys flat, and `node` type-strips them, so they stay green while the
  tolerance holds.
- Beside it, the array key itself has **no home**: `"hooks"` is a bare literal at
  `src/builtin_kind.rs:300`, `src/builtin_kind.rs:1076`, `src/kind.rs:932` and — added
  this tick, matching its neighbours — `src/install.rs:2368`'s `gate_hook_fields`. The
  SDK grew the constant (`HOOK_HANDLER_KEY`, `sdk/src/builtins.ts`); the engine has
  four transcriptions. Out of this entry's fence (`src/builtin_kind.rs`, `src/kind.rs`),
  so it could not be unified here.

## Observed at

34f95f54 (HEAD when observed) — the entry's own commit is the child of it.

## Suggested consolidation

Ride `EVERY-AUTHORED-HOOK-SPELLS-ITS-HANDLERS`: once no program spells a handler
key flat, the residue branch has no caller, so retire it with its two doc comments
in that entry rather than leaving `hook_matcher_group` with two arms and a
forward reference to work that already landed. Add `src/json_manifest.rs` to that
entry's fence.
