# PREDICATE-SEGMENTS-KEBAB-CASE: the wire key moved with the id

- I moved the wire key too, not just the id segment. `Predicate::key()` is both the lock's `predicate` discriminator and the label segment: one concept, one name. The seven predicates were the only snake_case keys in a vocabulary that is otherwise kebab (`closed-keys`, `glob-valid`, ...). So the SDK constructors, the Rust decoder, `key()`, the embedded lock and the example lock all spell `max-len` etc. now.
- Break: a lock emitted before this change has rows `predicate = "max_len"`, which the decoder now rejects as unknown. No alias was added, following the retired-`max_lines` precedent. The remedy is one `temper emit`, which `engine-matches` already points at. The release cut owes this line in the migration note (docs/).
- Prose left over, outside build's writable paths: `.temper/memory/CLAUDE.md:64` (projected to `CLAUDE.md`) still says `allowed_chars`. That is the tech-stack note about the charset mechanics. It needs a human `chore(harness):` edit plus a re-emit.
- Some test assertion messages and two sdk test titles still use the underscore word in prose. They are not ids, so I left them alone.
- The gauntlet snapshot did not move. The acceptance and builtin-contract-matrix snapshots did.
