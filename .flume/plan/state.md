# Plan state

- **Phase:** reconcile. MATCH-ERADICATE shipped (6889516), so the head of the
  serialized surface-language chain unblocks. Verified on disk: the name-`match`
  selector is gone — `match = {…}` is now an unknown-key reject (compose.rs:2056),
  roster.rs quantifies over satisfier sets — and every downstream rung is still
  unbuilt (`src/document.rs` absent; no PACKAGE.md loader; `template`/`adopt`
  vocabulary in compose.rs; `meta.toml`+body split in skill.rs/import.rs; inline
  `CustomKind`/`governs` in compose.rs; `contracts/*.toml` at main.rs:59/65).
- **Last shipped:** MATCH-ERADICATE — the name-`match` eradication, requirements
  quantified over satisfier sets (948b4d3 / 6889516).
- **In flight:** none.
- **Chain (serialized, shared compose/roster/main/import files ⇒ one at a time):**
  SURFACE-DOCUMENT-FORMAT (**open**) → PACKAGE-DOCUMENT → PACKAGE-BINDING →
  REQUIREMENT-PACKAGE-TYPING → MEMBER-DOCUMENT-IMPORT → KIND-AUTHORED-ARTIFACT →
  EMBED-BUILTIN-PACKAGES (parked: human authors the `.temper/packages/` std-lib).
- **Pickable now (1):** SURFACE-DOCUMENT-FORMAT. Everything else is blockedBy the
  chain, parked (EMBED-BUILTIN-PACKAGES, PACKAGING-CHANNELS), or deferred
  (COVERAGE-CUSTOM-KIND, AGENT-KIND).
- **Inbox:** empty (drained). **Forks:** all previously-open forks are RESOLVED —
  `(kind-artifact-format)` (→ `KIND.md`, un-gating KIND-AUTHORED-ARTIFACT),
  `(reference-id-normalization)`, `(read-verbs)`. No open fork blocks the queue;
  the read-family verbs (`why`/`requirements`) and spec-kind graph work stay
  tracked in open-questions, fileable after the surface-language migration.

Plan continues: no — the queue is reconciled to the corpus, the chain head is
pickable, and the inbox and fork frontier are clear; building drains it from here.
