<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Field report, two findings (observed at f9cc899), both verified against
  disk before filing:

  **A fresh regression, introduced by today's own KIND-NAME-COLLISION-
  ADMISSIBILITY fix (85fdffd): `row_relocates_builtin` (`main.rs:1089-1102`)
  hard-fails any legitimate templates-extension of a built-in host.**
  Confirmed: the check
  `declared.templates.is_empty() || declared.templates == builtin.templates`
  reduces to "declared templates must be empty" for every built-in, because
  a compiled-in built-in's own `templates` is always `Vec::new()` (the same
  fact the T18 finding traced — built-ins are constructed from
  `builtin_kind::definitions()`, never `from_kind_fact_row`, so nothing
  ever populates it). Any lock row that legitimately extends a built-in
  host with a declared child template — the exact T18 authoring pattern
  (e.g. `rule` gaining a `directive` template via `withinHosts: ["rule"]`)
  — now fails `kind.admissibility` outright, where before this fix it was
  merely silently inert (T18's original framing). Repro: declare a
  `directive`-style template against a built-in host in `harness.ts`,
  `check --harness .` goes from clean to `kind.admissibility` exit 1.
  This blocks T18's own eventual fix — extending `effective_governs` (or a
  sibling) to lift `templates` for built-ins would immediately trip this
  admissibility check the moment a row declares one, so the two need
  resolving together or in the right order, not independently.

  **Requirement satisfaction is real now (`fe2b22c` verified live via
  mutation testing — a bound temporarily set to an impossible `min: 99`
  produced genuine `graph.degree` violations naming the real measured
  degree, then reverted clean), but `memory` members are structurally
  invisible to the roster/graph/coverage tier regardless.** Confirmed:
  `by_kind`/`all_features` (`main.rs:678-688`) are assembled by chaining
  only `skill_features`, `rule_features`, and `custom_kinds` — nothing
  else. Tracing back to `gate()`'s built-in dispatch loop, every built-in's
  features get computed and validated against its own contract, but
  `match kind.name.as_str() { "skill" => ..., "rule" => ..., _ => {} }`
  silently discards the result for every other built-in, `memory`
  included. So a memory member's `declarations.satisfies` row — real,
  correct, present in the lock — is permanently invisible to
  `roster::check`/`graph::degree`/`coverage::check`: not a case these
  passes get wrong, a case they structurally cannot see, because the
  member never enters the corpus they range over. `fe2b22c` fixed which
  source populates a member already in that corpus; this is a member
  that's never in it. The gate's own comment ("no memory member publishes
  a requirement today; folding more built-ins into the requirement corpus
  is a separate scope question") reads as a deliberate boundary but is
  circular — restating what the hardcoded dispatch already forces, not an
  independent observation about the world. `explain <requirement>` can
  report "No member satisfies it" while the lock's own `satisfies` row
  says otherwise — a false verdict, not an incomplete one.
  Scoping note (traced further, same-day): this isn't a missing match arm
  fixable in isolation — `assemble_by_kind` (`main.rs:956-961`) itself
  takes exactly two named parameters (`skill_features`, `rule_features`),
  not a generic collection; the fix widens that signature (and its two
  call sites), not just the dispatch loop above it. `roster::check`/
  `graph::degree`/`coverage::check` themselves are already kind-generic —
  they take a plain `BTreeMap<&str, &[Features]>` and their own tests
  construct one with a non-real `"manifest"` kind name to prove it — so
  this is one isolated seam in `main.rs`, not a rewrite of the
  requirement-satisfaction engine. Traced the origin: this two-kind
  assumption dates to `skill_rule_corpus` (2026-07-04, `c9a576c`), from
  before `agent`/`command` shipped as built-ins (decision 0014) — pre-
  existing debt from when skill/rule genuinely were the only two built-ins
  with member-level requirement relevance, never revisited as the roster
  grew, not something introduced recently. A sibling with the identical
  shape, not yet verified further: `BUILTIN_DEFAULT_CONTRACT_KINDS`
  (`main.rs:63`, still `&["skill", "rule"]`) — worth checking when this
  entry is scoped, in case default-contract lookup has the same gap.
  One more scoping fact, confirmed by reading both sites in full:
  `gate()` and `explain()` are two independent implementations of the same
  corpus-assembly shape, not one function with one caller —
  `explain()` (`main.rs:~414-444`) separately fetches `skill_kind`/
  `rule_kind`, computes its own `skill_features`/`rule_features`, and
  calls the identical `assemble_by_kind(&skill_features, &rule_features,
  &custom_kinds)` `gate()` does. Widening `assemble_by_kind`'s signature
  and fixing only one call site leaves the other on the old two-kind
  assumption — `explain` and `check` would start disagreeing about which
  members satisfy what, which is worse than both being wrong the same
  way. Scope this entry to consolidate the two into one shared
  corpus-assembly helper, not patch each copy — the `specs/process/
  engineering.md` standard this repo just ratified names this exact
  shape ("one job, one home").

  Not filed, already correctly resolved by plan's own re-verify-before-
  scoping discipline: T14 (kind-rename-deletes-files) does not reproduce —
  `drift.rs`'s `owned_paths` cross-check (built from every path the
  *current* run's projections claim, checked before any prior-lock row
  reaches `reap_or_report_orphan`) already guards the exact same-locus
  rename scenario. Plan caught this independently (`5955a07`, "T14
  refuted") before any pending entry was filed for it — noted here only
  so the record is complete, not because it needs routing.

