# Plan state

- **Phase:** reconcile. HEAD 7405f21.
- **Last shipped:** RECURSIVE-GOVERNS-PLACEMENT-ID (build 007178e, chore 26e296e) —
  the final slice of the memory engine wave; all five slices have landed and are
  re-verified on disk (`collect_glob` recurses `**`, `wholefile_id` folds placement,
  `resolve_bare` carries qualified-identity/collision resolution).
- **This tick:** drained the inbox — filed **SCAN-QUALIFIED-IDENTITY** (`open`), the
  engine bug the human found by placing the four curated memory files locally: the
  generic import/drift scans iterate the qualified `definitions()` set but then
  RE-RESOLVE each kind by its bare name (`import.rs:178,222`; `drift.rs:763` — the
  inbox's `:1030` is a test helper), so co-embedding two `memory` providers throws
  `AmbiguousKind` on scans no caller ever pointed at bare. All three sites verified
  on disk. It also re-pins the hardcoded enumeration test (`builtin_kind.rs:278`) to
  derive from the `kinds/` tree. Refreshed MEMORY-KIND's gate/notes: its human file
  commit is now gated on SCAN shipping first. No other entry moved; all cites still
  resolve.
- **Operational note (accepted, not queued):** the 17 `requirement.dangling`
  session-start findings are a **stale installed binary** — the freshly-built
  binary's `temper check .temper` is clean; `cargo install --path .` clears them.
- **Pickable now:** SCAN-QUALIFIED-IDENTITY (sole `open`, disjoint from every other
  entry — they touch kind.rs/extract.rs/builtin.rs/docs, never import/drift/
  builtin_kind). Parked (human action): MEMORY-KIND, PACKAGING-CHANNELS,
  COMMUNITY-DOCS. Deferred (no consumer): EXTRACTION-VOCAB-GAPS, AGENT-KIND.

Plan continues: no — inbox drained, one pickable `open` entry filed and disjoint
from all; hand to build.
