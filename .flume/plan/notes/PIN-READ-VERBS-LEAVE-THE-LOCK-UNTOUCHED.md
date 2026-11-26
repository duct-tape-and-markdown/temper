# PIN-READ-VERBS-LEAVE-THE-LOCK-UNTOUCHED: the property holds

Pinned in `tests/it/read_verbs.rs`. The test spawns the binary: `check` (bare, and with `--harness`), `guard` (Write and Edit payloads, and it asserts the block-mode exits so we know the lock was actually read), and `explain` (member, qualified, and absent targets). They run against a lock with one clean member and one drifted member. The lock bytes stay identical after every verb. Nothing was parked.
