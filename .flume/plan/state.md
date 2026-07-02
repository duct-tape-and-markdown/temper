# Plan state

- **Phase:** reconcile + inbox drain. HEAD 2331a4d.
- **Last shipped (trunk):** the queue was drained to deferred/parked last tick; the
  intervening commits are human chores (curated `kinds/skill|rule/KIND.md`
  hand-authored — b06f3a0; the `kinds/**` fence note + `files.retire`-is-a-path rule
  — 2331a4d/40df). No `build:` tick since READ-CUSTOM-SATISFIERS.
- **This tick:** drained the 4-line inbox into the **source-not-mechanism extraction
  wave** — built-in/custom is SOURCE, not MECHANISM (human ruling; `15-kinds.md`
  end-state: extraction composed from a closed algebra). Filed a serialized 4-entry
  chain, code-only (`src/**`, `build.rs`, `tests/**` — `kinds/**` is human territory,
  build EMBEDS but never writes): HEADER-FIELD-EXTRACTION (open) →
  EXTRACT-EQUIVALENCE-PIN → EMBED-BUILTIN-KINDS → BUILTIN-EXTRACT-GENERIC, each
  `blockedBy` the prior so no two open entries share a file. The prior fence revert
  was a schema misuse (symbol retirement is an `edit`, not a `retire` path) —
  corrected: every retirement is an edit naming the symbol, `retire: []`. Recorded
  `(extraction-source-not-mechanism)` RESOLVED; fixed a stale open-questions line
  (`contracts/` is deleted on disk, not "pending deletion"). AGENT-KIND (deferred),
  PACKAGING-CHANNELS / COMMUNITY-DOCS (parked) carried, re-verified.
- **Pickable now:** HEADER-FIELD-EXTRACTION (open). The rest of the wave serializes
  behind it; deferred/parked entries unchanged. Sole live OPEN fork:
  (edge-representation-unify).

Plan continues: no — HEADER-FIELD-EXTRACTION is pickable; hand to build. The wave
serializes one entry at a time, so re-planning before build ships the head would
only re-emit the same queue.
