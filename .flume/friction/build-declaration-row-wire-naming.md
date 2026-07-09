## Symptom

Added a seventh declaration-row family (`nested_members: Vec<NestedMemberRow>` on
Rust's `Declarations`) and its SDK-side field. Named the TS `Declarations`
interface field `nestedMembers` (idiomatic camelCase) — every existing
multi-word row field in `sdk/src/declarations.ts` (`governs_root`,
`unit_shape`, `verified_by`, `source_path`, …) is actually spelled snake_case
because it's the literal wire key `#[derive(Deserialize)]` reads on the Rust
side, no `#[serde(rename)]` anywhere. This convention is legible once you
notice it but isn't stated anywhere; `sdk.md`'s "The engine seam" section says
the row builders/Rust reader/embedded lock "move together" but not that field
*spelling* must match byte-for-byte.

## Cost this tick

One full red loop: `cargo test` passed locally (Rust side alone has no
opinion on TS spelling), but `tests/builtin_lock_frozen.rs` — which shells out
to a real `node` build of the SDK — failed with `missing field
'nested_members' at line 383` since the JSON payload carried `nestedMembers`.
Caught only by that one cross-language test; a `cargo build`/`cargo clippy`
pass alone would have shipped it broken. ~10 minutes to trace from the
opaque payload-parse error back to the naming mismatch.

## Suggested fix

Add one line to `.claude/rules/sdk.md`'s "The engine seam" bullet: new/changed
declaration-row fields are spelled snake_case in the TS interface too — they
are the literal wire key, not a TS-idiomatic name — and the check that would
have caught this is `pnpm --dir sdk build && cargo test --test
builtin_lock_frozen` (or any `tests/emit.rs`-style golden-lock fixture), not
`cargo test` alone.
