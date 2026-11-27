# EMIT-PINS-ONE-OPEN-PER-FILE

Shipped `emit_opens_each_file_at_most_once_per_run` in tests/it/check_cost.rs:
a re-emit over represented_projection_harness's payload, counted at
`temper::bytes::paths_opened`, asserts SKILL.md, rust.md and lock.toml were
each read and none twice. Green on the base as a pin should be; no src/ change.

The harness's payload is now `represented_projection_payload()`, so the
fixture and the pin emit the same thing. INSTALL-RUN-HOLDS-ONE-OWNER-PER-INPUT
can reuse it from the same file.
