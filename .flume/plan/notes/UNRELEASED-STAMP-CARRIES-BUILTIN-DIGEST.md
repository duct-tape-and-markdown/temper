# Unreleased stamp carries the built-in digest

Shipped: an unreleased stamp now reads `<release>+unreleased.<12 hex>`, the
leading digits of the SHA-256 of `src/builtin_lock.toml` (the bytes
`builtin_lock.rs` embeds). `engine_stamp()` takes the built-in bytes as a
third argument; `sha2` is now also a build-dependency (Cargo.lock unchanged).
build.rs declares `src/builtin_lock.toml` as a rerun input.

Human half still owed: re-emit `.temper/lock.toml` (outside build's fence) —
its stamp is still bare `0.0.25+unreleased`, which now reads as another
engine (advisory `root.engine-matches` until re-emitted). The example lock
was re-emitted here; only its first line moved.
