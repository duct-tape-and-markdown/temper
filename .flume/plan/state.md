# Plan state

- **Phase:** the corpus gained a new human-authored spec, `specs/15-kinds.md` (the
  kind system + the closed **extraction** algebra — predicates:contracts :: extraction:kinds).
  It authorizes the `spec` **custom kind** over `specs/*.md` and resolves most of the
  long-open `(spec-landscape-kind)` fork. This tick files the `spec`-kind build-out
  through the pipeline (IR → import → workspace → gate).
- **Last shipped:** ROSTER-ADMISSIBILITY (f742cca / be567fb). Verified on disk:
  `roster.rs` `admissibility()` checks each role's `match` resolves, a `required` role's
  kind is satisfiable, its contract resolves + passes `engine::admissibility`, and any
  `verified_by` resolves; `main.rs:149` wires it into the Check arm before selection.
- **In flight:** nothing; pending was empty (build drained ROSTER-ADMISSIBILITY). Tree
  clean apart from two untracked human artifacts — `specs/15-kinds.md` (now in the corpus,
  reconciled below) and `contracts/spec.toml` (the spec contract — still human territory).
- **Next (filed):** SPEC-KIND-IR (`open`, fork-free) — the `Spec` IR module; then
  SPEC-KIND-IMPORT + SPEC-KIND-WORKSPACE (`open`, `blockedBy` IR); then SPEC-KIND-GATE
  (`parked` — a human must first commit the untracked, curated `contracts/spec.toml` the
  gate embeds via `include_str!`).
- **Reconciled this tick:** queue empty, nothing to rewrite/drop. Inbox empty. Confirmed
  no `spec` kind on disk (`src/spec.rs` absent; `import.rs`/`check.rs` scan only skills +
  rules). Updated `(spec-landscape-kind)` → RESOLVED for fronts (1) + (3): `15-kinds.md`
  makes `spec` a checked kind and **declares** the backtick-filename reference syntax.
  Carved the remaining narrow gap — the `section_contains`/decisions-name-alternatives
  **predicate** is still not enumerated in `10-contracts.md`'s closed predicate algebra —
  as the new `(decision-marker-predicate)` fork.
- **Frontier (fork-free, unfiled — follow-on ticks):** the `references-resolve` referential
  predicate (over `15-kinds.md`'s now-declared backtick-filename syntax) + its reference
  extraction; then the older frontier — `temper-local.toml` second layer; `temper schema`
  (JSON-Schema emit); the advisory session-start gate + `claude-session-start`/GitHub/SARIF
  reporters; the `apply`/`re-add`/`install` drift engine; the plugin tree + `temper bundle`.
  **Still fork-blocked:** decisions-name-alternatives (`(decision-marker-predicate)`); the
  declared model + dependency graph + cross-landscape seam (`(model-declaration-format)`);
  full `pattern` (`(regex-crate)`, held); the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (ROSTER-ADMISSIBILITY confirmed shipped on
disk; the spec kind confirmed unshipped and decomposed into four disjoint entries),
`15-kinds.md` reconciled into the open questions, the inbox is empty, and SPEC-KIND-IR is
immediately pickable and fork-free. Build runs and drains it.
