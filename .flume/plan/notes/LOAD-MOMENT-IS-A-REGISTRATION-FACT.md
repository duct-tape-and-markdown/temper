# Load moment: shipped on the prior diff, unblocked by the digest stamp

Re-applied the parked diff (`git apply -3`, clean) on top of c903b390. The
digest stamp answers the park: the self-host check now reads the
repository's `.temper/lock.toml` (bare `+unreleased` stamp) as another
engine's, so its old `memory`/`skill` rows retire with the advisory re-emit
finding instead of a kind collision. `cargo test`, clippy, fmt, doc and the
SDK gate are green.

Owed by a human (outside build's fence): re-emit `.temper/lock.toml` so the
dogfood carries `on-read-beneath` and `paths-listing(paths)` and the
re-emit advisory clears.

Prediction vs landing: no edit was needed in compose.rs, drift.rs or
tests/it/lock_declaration_rows.rs; their channel matches are non-exhaustive
or never name these variants, and the explain tests in read_verbs.rs pin the
new rows end to end through the built-in lock.
