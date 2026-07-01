# Plan state

- **Phase:** reconcile. PACKAGE-BINDING shipped (1a3d8cd), so the next rung of the
  serialized surface-language chain unblocks: **REQUIREMENT-PACKAGE-TYPING** flips
  from `blockedBy` to **open**. Verified on disk: the kind-side bind-by-name floor
  is live (`compose.rs` — `[kind.<name>] package = "<name>"`, path-adopt retired),
  while every downstream rung is still unbuilt — `RequirementContract::Template`/
  `Inline` + the `contract =` requirement key alive (`compose.rs:419-449/270`),
  `meta.toml`+body split in `skill.rs`, `satisfies: Vec::new()` at `kind.rs:454`,
  `include_str!("../contracts/*.toml")` at `main.rs:59/65`.
- **Last shipped:** PACKAGE-BINDING — packages bound by name over the built-in
  floor ∪ `.temper/packages/` (36cc4ed / 1a3d8cd).
- **In flight:** none.
- **Chain (serialized — shared compose/roster/skill/kind/main files ⇒ one at a
  time):** REQUIREMENT-PACKAGE-TYPING (**open**) → MEMBER-DOCUMENT-IMPORT →
  KIND-AUTHORED-ARTIFACT → EMBED-BUILTIN-PACKAGES (parked: human authors the
  `.temper/packages/` std-lib).
- **Pickable now (1):** REQUIREMENT-PACKAGE-TYPING. Everything else is blockedBy
  the chain, parked (EMBED-BUILTIN-PACKAGES, PACKAGING-CHANNELS), or deferred
  (COVERAGE-CUSTOM-KIND, AGENT-KIND).
- **Inbox:** empty. **Forks:** all resolved except `(project-name)` (human-only,
  gates only PACKAGING-CHANNELS launch work); read-family verbs and spec-kind
  graph work stay tracked in open-questions, fileable after the migration.

Plan continues: no — the queue is reconciled to the corpus, the chain head is
pickable (REQUIREMENT-PACKAGE-TYPING open), and the inbox and fork frontier are
clear; building drains it from here.
