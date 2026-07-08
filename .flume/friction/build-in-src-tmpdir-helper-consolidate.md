**Symptom:** entry `IN-SRC-TMPDIR-HELPER-CONSOLIDATE` specified, for `src/main.rs`,
"add `use crate::test_support::tmpdir;`" — identical to the other 7 sites. That
fails to compile: `main.rs` is the crate root of the separate `temper` *binary*
crate (it references the library via `temper::...`, e.g. `use temper::builtin;`),
not a module of the library crate that owns `src/test_support.rs`. Neither
`crate::test_support` nor `temper::test_support` resolves — the former because
`main.rs`'s own crate tree has no such module, the latter because
`test_support` is `#[cfg(test)] pub(crate)`, invisible outside the library
crate and not even compiled into it when the binary depends on it normally.

**Cost this tick:** one full clippy cycle burned diagnosing an
`unresolved import` before recognizing the bin/lib crate split as the actual
cause, rather than a typo.

**Fix applied:** `main.rs` now carries its own `#[cfg(test)] mod test_support;`
pointing at the same `src/test_support.rs` file (default sibling-module path
resolution, no `#[path]` needed) — one authored source, compiled twice (once
per crate), each `pub(crate)` scoped to its own crate. Source-level "one home"
without promoting `tempfile` out of `dev-dependencies` or widening
`test_support`'s visibility.

**Suggested fix:** when a future entry's `files.edit` list spans both
`src/main.rs` and library modules for a `#[cfg(test)]`-only helper, name the
bin/lib crate split explicitly and prescribe the `mod test_support;`
re-declaration in `main.rs` rather than a bare `use crate::...`.
