# PATH-ROOT-NEWTYPE-LOCK-SOURCE-PATH — blocked on coverage_note.rs scope

## Blocker

Changing `drift::to_lock_path()` to return `HarnessRelativePath` breaks `src/coverage_note.rs:199`, which calls this function and collects the result into `Vec<(String, bool)>`. The coverage_note.rs file is outside the entry's declared write allowance (src/path.rs, src/drift.rs, tests/lock_declaration_rows.rs, tests/emit.rs).

## Path

- src/coverage_note.rs:199 — `drift::to_lock_path(rel)` used in a tuple collected to `Vec<(String, bool)>`

## Scope issue

The entry must either:
1. Expand write allowance to include src/coverage_note.rs, or
2. Change approach to avoid breaking existing callers of `to_lock_path`, perhaps by creating a separate function that returns HarnessRelativePath while keeping the original returning String for backwards compatibility, or
3. Re-scope to exclude the producer-function changes and retype only the lock row structures

Recommend: clarify whether coverage_note.rs should be updated to use HarnessRelativePath, or whether a dual-signature approach (keeping String return for external callers, creating _typed variant for internal use) is preferred.
