# Plan state

- **Phase:** decompose — the tick the resolved `(package-surface-sequencing)`
  fork called for. The PACKAGE-MODEL-RECONCILE monolith is replaced by a
  serialized chain (shared compose/roster/main files ⇒ `blockedBy`, one at a
  time): MATCH-ERADICATE (open, inbox-routed, atomic) → SURFACE-DOCUMENT-FORMAT →
  PACKAGE-DOCUMENT → PACKAGE-BINDING → REQUIREMENT-PACKAGE-TYPING →
  MEMBER-DOCUMENT-IMPORT → KIND-AUTHORED-ARTIFACT (also on the new
  `(kind-artifact-format)` fork) → EMBED-BUILTIN-PACKAGES (parked: human authors
  the `.temper/packages/` std-lib — dogfood after machinery). Verified on disk:
  `match` selectors live across compose/roster/graph/coverage, `template`/`adopt`
  vocabulary throughout, `contracts/*.toml` embedded at main.rs:59/65, import
  writes the meta.toml+body split, custom kinds fully inline in `[kind.*]`.
- **Last shipped:** `lock.toml` rollup rename (RENAME-ROLLUP-LOCK, on disk).
- **In flight:** none.
- **Inbox:** drained. MATCH-ERADICATE filed as the sole `open` entry per its
  routing. The representation hold is lifted — the human authored the
  surface-language model into `20-surface.md` (9aed8cc) after routing the hold,
  and the resolved sequencing fork directs plan to reconcile against that corpus;
  the representation work is filed as SURFACE-DOCUMENT-FORMAT +
  MEMBER-DOCUMENT-IMPORT.
- **Pickable now (1):** MATCH-ERADICATE. Everything else is blockedBy the chain,
  parked (EMBED-BUILTIN-PACKAGES, PACKAGING-CHANNELS), or deferred
  (COVERAGE-CUSTOM-KIND, AGENT-KIND — both rewritten to the surface-language
  shapes).
- **Blocked frontier (forks):** `(kind-artifact-format)` (new — the custom kind
  definition's concrete file/dialect), `(reference-id-normalization)`,
  `(read-verbs)` — all await a human decision.

Plan continues: no — the queue is reconciled to the surface-language corpus, the
inbox is drained, and MATCH-ERADICATE is pickable; building is how the queue
drains from here.
