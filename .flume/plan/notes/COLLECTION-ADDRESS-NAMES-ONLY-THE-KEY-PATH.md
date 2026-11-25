# COLLECTION-ADDRESS-NAMES-ONLY-THE-KEY-PATH: Rust struct field survives on a fence miss

Shipped: the `collection_address` row (lock + SDK `CollectionAddressRow`) and the SDK
`CollectionAddress` drop `manifest`; a registration kind's manifest is its starless `at`
locus glob (`manifestOf` in sdk/src/kind.ts, refused at `kind()` for a starred or non-`at`
locus; `CustomKind::manifest()` in src/kind.rs, a starred/missing `governs_glob` is a
`LockRowError` at lift). Every engine consumer (admissibility collision, main's guard
expected keys, drift `manifest_path_for`, read's explain) reads it off the locus.

Not shipped: the Rust in-memory `kind::CollectionAddress::manifest` field. It stays,
filled by the lift from `governs_glob`, because `src/builtin_kind.rs` (outside this
tick's fence) spells it in four live built-in definitions, and `builtin_lock.rs` compares
those against the lifted rows. No consumer reads the field any more. Next entry: retire
the field; fence `src/builtin_kind.rs`, `src/kind.rs`, and the tests constructing
`CollectionAddress { manifest, .. }` (tests/it/{hook_kind,mcp_server_kind,manifest_adapter,
check_cost,installed_plugin_kind,emit}.rs).

Behavior shift: expected-keys and collision now follow a *relocated* registration kind's
glob, where they used to follow the spelled-in address that relocation never moved.

Dogfood half (chore(harness)): re-emit `.temper/lock.toml` so its collection_address
rows lose `manifest`. The reader ignores the stale key, so nothing breaks before then.
