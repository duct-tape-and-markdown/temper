<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- Resolve `(rollup-index-rename)` → rename the generated roll-up `author.toml` →
  `.temper/lock.toml`. It is the contents' **state-of-record** (provenance +
  drift/apply fingerprints), not an index — a lock (Cargo.lock analogy). The spec
  already names `lock.toml` (`20-surface.md`), so this is a mechanical CODE rename:
  `src/import.rs` (write path + the `author.toml` literal/comments), `src/drift.rs`
  (baseline read), `src/main.rs` and any other reference, plus tests/snapshots that
  assert `author.toml`. Green the gates as usual; keep entries disjoint or serialized.
