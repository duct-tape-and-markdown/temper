# Plan state

- **Phase:** reconcile. REQUIREMENT-PACKAGE-TYPING shipped (afe6f48 / 134a27f),
  so the next rung of the serialized surface-language chain unblocks:
  **MEMBER-DOCUMENT-IMPORT** flips from `blockedBy` to **open**. Verified on disk:
  the require-side typing floor is live (`compose.rs` — `package` typing facet;
  `contract =` / `[[requirement.*.clause]]` now rejected as unknown keys), while
  the member read/write surface is still the retired pair — `Skill::to_meta_document`
  writes `meta.toml` + body (`skill.rs`/`import.rs`), `satisfies: Vec::new()` at
  `kind.rs`, `include_str!("../contracts/*.toml")` at `main.rs:59/65`. The
  `+++`-fenced document primitive (`src/document.rs`) foundation exists to build on.
- **Last shipped:** REQUIREMENT-PACKAGE-TYPING — requirements typed by package
  name, the `contract` facet retired (afe6f48 / 134a27f).
- **In flight:** none.
- **Chain (serialized — shared import/skill/rule/kind/check files ⇒ one at a
  time):** MEMBER-DOCUMENT-IMPORT (**open**) → KIND-AUTHORED-ARTIFACT →
  EMBED-BUILTIN-PACKAGES (parked: human authors the `.temper/packages/` std-lib).
- **Pickable now (1):** MEMBER-DOCUMENT-IMPORT. Everything else is blockedBy the
  chain, parked (EMBED-BUILTIN-PACKAGES, PACKAGING-CHANNELS), or deferred
  (COVERAGE-CUSTOM-KIND, AGENT-KIND).
- **Inbox:** empty. **Forks:** all resolved except the human-only OPEN strategics
  (`kind-harness-axis`, `multi-harness-projection`) and provisional `project-name`;
  none gate the chain head.

Plan continues: no — the queue is reconciled to the corpus, the chain head is
pickable (MEMBER-DOCUMENT-IMPORT open), and the inbox and fork frontier are clear;
building drains it from here.
</content>
