# Plan state

- **Phase:** reconcile. HEAD fab4f79.
- **Last shipped:** DECLARED-FRONTMATTER-ADAPTER (fab4f79 / build 2a11cb0) — the
  per-kind IRs collapsed into one generic frontmatter adapter; verified on disk
  `src/skill.rs` + `src/rule.rs` are gone, `document.rs`/`frontmatter.rs` carry
  the adapter, `BUILTIN_KINDS = ["skill","rule"]`.
- **This tick:** filed MEMORY-KIND (parked) from the drained open-questions
  CLAUDE.md-bootstrap recipe — the CLAUDE.md-onto-surface flip proof, blocked on
  human-authored curated `kinds/memory` + `packages/memory.anthropic` (outside
  build's fence). Corrected the recipe's overclaim: memory's `@path` is its own
  edge primitive, NOT EXTRACTION-VOCAB-GAPS's fenced/key-path — that entry stays
  deferred, not revived. Re-anchored stale line refs (`Primitive` kind.rs:361-382,
  `Field` flat-get kind.rs:404) and dropped the shipped-DFA cross-refs in
  EXTRACTION-VOCAB-GAPS + AGENT-KIND. Inbox already empty.
- **In flight / pickable:** none open. Every entry is parked (MEMORY-KIND,
  PACKAGING-CHANNELS, COMMUNITY-DOCS) or deferred (EXTRACTION-VOCAB-GAPS,
  AGENT-KIND) — all remaining work is human-gated.
- **Next (human action, not build):** author `kinds/memory/KIND.md` +
  `packages/memory.anthropic/PACKAGE.md` to un-park MEMORY-KIND; or widen the
  fence for COMMUNITY-DOCS; or set release creds for PACKAGING-CHANNELS.

Plan continues: no — queue reconciled; no `open` entry remains and all pending
work is human-gated (curated kind data, fence-widen, release creds). Nothing for
build to pick and nothing more for plan to do this turn.
