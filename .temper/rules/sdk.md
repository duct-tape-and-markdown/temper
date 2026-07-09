# SDK conventions — author

Path-scoped to `sdk/`. The bar is `pnpm --dir sdk test` clean — one script,
strict `tsc` then `node --test` over the compiled `dist/test/`. It is a flume
afterMerge gate, so a violation reverts the entry after the cargo gates have
already passed; when a change touches `sdk/**`, run it in the green loop
alongside the cargo trio.

## The engine seam

- The SDK is the authoring face of the same model the Rust engine reads; emit
  is a byte-faithful projection of it. `tests/builtin_lock_frozen.rs`
  re-derives the built-in lock by building and running the real SDK
  (`npm run build`, `node`) and byte-compares it against the engine's embedded
  copy — a green `cargo build` proves nothing about this seam; only
  `cargo test` exercises it.
- A declaration-row change is two-sided by construction: the row builders in
  `sdk/src/declarations.ts`, the Rust reader (`src/read.rs`), and the embedded
  lock (`src/builtin_lock.toml`) move together, in one commit. A row field's TS
  name is the literal wire key the Rust side `#[derive(Deserialize)]`s — always
  snake_case to match, never the idiomatic TS camelCase, since there is no
  `#[serde(rename)]`. `cargo test` alone won't catch a mismatch (Rust has no
  opinion on TS spelling); `pnpm --dir sdk build && cargo test --test
  builtin_lock_frozen` exercises the real cross-language seam.
- Rust↔TS ripple is the normal shape of work here, not scope creep. When a
  contract change surfaces in emit output, `rg` both trees for the seam before
  concluding the fix is one-sided — an entry that names only `src/**` may
  still owe its matching `sdk/src/` edit.

## Style & tests

- The comment taxonomy and churn rules of the rust rule apply unchanged
  (`.claude/rules/rust.md`, "Style & structure"): comment only what the code
  can't say, no spec-path citations, headers shrink on touch.
- Tests are framework-free `node --test` files in `sdk/test/*.test.ts`
  (`node:assert`), compiled by the same `tsc` pass as the source.
