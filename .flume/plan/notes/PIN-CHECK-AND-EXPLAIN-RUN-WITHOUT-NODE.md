# PIN-CHECK-AND-EXPLAIN-RUN-WITHOUT-NODE: the property holds

Pinned in tests/it/read_verbs.rs: a real SDK program is emitted through the node
seam, then `check` (github reporter) and `explain` (a skill and a qualified rule
target) are spawned with PATH set to an empty directory. Both succeed and read
byte-identical output to the full-PATH run.

The only `node`/`npm`/typecheck spawns in src/ are emit-time (drift.rs program run
and typecheck) and install-time (install.rs) — none on the read verbs' path.

Shared runner change: tests/it/common grew `check_in_on_path` /
`explain_in_on_path` (optional PATH override); `check_in` / `explain_in` delegate,
so the one-home rule for spawning those verbs still holds.
