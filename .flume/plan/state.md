# Plan state

- **Phase:** reconcile. SURFACE-DOCUMENT-FORMAT shipped (6f57255), so the next
  rung of the serialized surface-language chain unblocks: PACKAGE-DOCUMENT flips
  from `blockedBy` to **open**. Verified on disk: `src/document.rs` is the fenced
  `+++` header/body primitive (wired in `lib.rs`); every downstream rung is still
  unbuilt — no PACKAGE.md loader in `contract.rs`, `adopt`/`template`/
  `RequirementContract::Template` alive in `compose.rs`, `contracts/*.toml`
  embedded at `main.rs:59/65`, `meta.toml`+body split in `skill.rs`.
- **Last shipped:** SURFACE-DOCUMENT-FORMAT — the surface-language fenced-document
  primitive (6567d33 / 6f57255).
- **In flight:** none.
- **Chain (serialized, shared compose/contract/main/import files ⇒ one at a time):**
  PACKAGE-DOCUMENT (**open**) → PACKAGE-BINDING → REQUIREMENT-PACKAGE-TYPING →
  MEMBER-DOCUMENT-IMPORT → KIND-AUTHORED-ARTIFACT → EMBED-BUILTIN-PACKAGES
  (parked: human authors the `.temper/packages/` std-lib).
- **Pickable now (1):** PACKAGE-DOCUMENT. Everything else is blockedBy the chain,
  parked (EMBED-BUILTIN-PACKAGES, PACKAGING-CHANNELS), or deferred
  (COVERAGE-CUSTOM-KIND, AGENT-KIND).
- **Inbox:** empty. **Forks:** all resolved — no open fork blocks the queue; the
  read-family verbs and spec-kind graph work stay tracked in open-questions,
  fileable after the surface-language migration.

Plan continues: no — the queue is reconciled to the corpus, the chain head is
pickable, and the inbox and fork frontier are clear; building drains it from here.
