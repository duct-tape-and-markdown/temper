<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- From a cross-perspective review of the shipped requirements/satisfies/representation
  chain, plus three now-RESOLVED design decisions (spec `e72c23c`). File the build
  entries below.

- SERIALIZATION NOTE (read first): the MUST-FIX touches `src/import.rs`/`skill.rs`/
  `rule.rs` and is disjoint from everything else → runs in parallel. The other three
  (HARDEN-COVERAGE, FILLED-BY-ROLE, STRAY-KEY-REJECT) all touch `src/compose.rs` and/or
  `src/coverage.rs`, so they SHARE files → serialize them into one chain (`blockedBy`),
  do NOT fan them out in parallel or the wave conflicts and reverts.

- MUST-FIX (high, data-loss): `re-add`/`import` clobber authored representation.
  `drift::re_add` (`src/drift.rs:961-969`) re-projects drifted/added skills+rules through
  `import::import_skill`/`import_rule`, which rebuild `meta.toml` from source
  (`skill.rs:188-189` → `satisfies` empty, `rationale` None) and only emit
  `[representation]` when non-empty (`skill.rs:306`). Representation is surface-only
  authored state with no source side, so a body edit on disk + `re-add` silently WIPES
  authored `satisfies`/`rationale` → coverage flips a covered requirement to false
  UNFILLED. Violates `specs/20-surface.md` "merge rather than clobber" (three-state law).
  FIX: before writing, if the target surface `meta.toml` holds `[representation]`, read it
  (`from_surface_dir` already parses it, `skill.rs:238-242`) and carry `satisfies`/
  `rationale` forward — a small merge helper shared by `import_skill`/`import_rule` (and
  the rule equivalent). Add a round-trip test: author satisfies → drift the body on disk →
  re-add → assert satisfies+rationale survive. Touches `src/import.rs` + `src/skill.rs` +
  `src/rule.rs` + tests. DISJOINT → parallel-safe.

- HARDEN-COVERAGE (low): (a) pin EXACT-STRING match for `satisfies`↔`requirement.<name>`
  (both literal human-authored TOML keys; no case/whitespace fold) + a mismatch fixture
  proving a typo yields the paired UNFILLED+DANGLING (true positives); (b) dedup
  `satisfies` before the dangling loop so `["x","x"]` emits ONE diagnostic; (c) doc
  cross-ref in `src/coverage.rs`: DANGLING mirrors `graph::check` route-resolution,
  UNFILLED mirrors `graph::degree` min-in-degree over a NON-artifact target set, and WHY
  unifying into `graph.rs` is rejected (avoids a fake `requirement` kind in `by_kind`).
  Touches `src/coverage.rs` + `src/compose.rs` + tests. SERIALIZE with the two below.

- FILLED-BY-ROLE (resolved decision; spec `10-contracts.md` "Two fill paths"): parse
  `filled_by = { role = "<name>" }` on `[requirement.*]` in `src/compose.rs`. Coverage
  semantics: a requirement carrying `filled_by` is covered iff the named role's required
  filler is present (delegate to `roster` match — referential, decidable), NOT by
  `satisfies` opt-in. ONE fill path per requirement: `filled_by` and bare-`satisfies`
  coverage are mutually exclusive for a given requirement. Admissibility: `filled_by` must
  name a DECLARED role (referential resolve, same posture as `verified_by`); dangling role
  ref → admissibility error. Deliberately NO `filled_by = { kind }` (duplicates
  `role.artifact`). Touches `src/compose.rs` + `src/coverage.rs` + tests. SERIALIZE.

- STRAY-KEY-REJECT (resolved decision; spec `10-contracts.md` "Decision: unknown keys are
  rejected"): unknown keys in `temper.toml` contract tables — `[requirement.*]`, `[role.*]`,
  `[kind.*]`, predicate clauses — become a HARD parse/admissibility error, not silently
  dropped (`requird = true` must fail, not degrade to `required=false`). Project-wide across
  the contract-surface parsers in `src/compose.rs`. DO NOT touch the artifact `extra`
  unknown-frontmatter catch-all (law 5 byte-preserves that — it is content, not a contract
  key). Touches `src/compose.rs` + tests. SERIALIZE.

- kind-blind ratification: SPEC-ONLY, already shipped in `e72c23c` (coverage is already
  kind-blind in code). No build entry — noting so plan doesn't file a no-op.

- DEFER (track as real entries): (1) custom-kind (`spec`) gains a `[representation]` read +
  `Features.satisfies` population so temper's own spec corpus participates in coverage —
  `kind::Unit` hardcodes `satisfies=Vec::new()` (`src/kind.rs:452-455`). (2) `temper why
  <artifact>` + `temper requirements` READ verbs (forward `satisfies`/`filled_by → means`+
  rationale; reverse `requirement → satisfiers` = blast radius) — queue AFTER the dogfood.

- DOGFOOD (after MUST-FIX lands): author temper's own `[requirement.*]` in the root
  `temper.toml` + a `satisfies`/`filled_by` — human territory (root `temper.toml`), NOT a
  build entry; sequence after REPRESENTATION-PRESERVE. The friction (harness carries rules,
  not skills) IS the demo.
