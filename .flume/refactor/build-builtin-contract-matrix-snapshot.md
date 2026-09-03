# build: Insta snapshot file scope

## Problem

Entry `BUILTIN-CONTRACT-MATRIX-SNAPSHOT` requires creating a test at `tests/builtin_contract_matrix.rs` that uses insta snapshot testing (per entry.acceptance: "snapshot-tested off the embedded src/builtin_lock.toml").

The insta crate generates and manages a snapshot file at `tests/snapshots/builtin_contract_matrix__builtin_contract_matrix.snap` which must be committed alongside the test source.

Entry.files declares only `tests/builtin_contract_matrix.rs`, but the snapshot file is not optional — it is reviewed, maintained, and committed as part of the test suite (same as the 16 other .snap files in tests/snapshots/ across the codebase).

The previous attempt (reverted at 414903d2) included the snapshot file in the commit, triggering the writable-paths gate failure: the snapshot path is under the phase's writablePaths but outside the entry's declared files.

## Solution

Widen entry.files to include the generated snapshot:
- `tests/builtin_contract_matrix.rs` (test source)
- `tests/snapshots/builtin_contract_matrix__builtin_contract_matrix.snap` (generated snapshot, reviewed and committed)

This mirrors the pattern in every insta snapshot test in the codebase.
