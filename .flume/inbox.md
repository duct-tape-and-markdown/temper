<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- MAJOR RECONCILE (spec `0784d00`): `role` and `requirement` are now ONE concept in the
  specs — a single **requirement** (a named obligation with optional `means`, optional
  typing `kind`/`contract`, a fill via opt-in `satisfies` or a `match` selector, optional
  multiplicity, optional `verified_by`). `filled_by` is RETIRED. Coverage is the one
  referential check. Reconcile the CODE to the spec the way `spec.rs` was retired earlier.

- KILL `FILLED-BY-ROLE` outright — do NOT build it. The spec removed `filled_by` entirely
  (there are no longer two concepts to bridge). Drop the pending entry.

- CONSOLIDATION (file this; decompose as needed, but SERIALIZE — the steps share
  `src/compose.rs`/`roster.rs`/`coverage.rs`/`graph.rs`/`main.rs`, all currently GREEN, so
  each step must keep `cargo test` + clippy green, updating `insta` snapshots deliberately):
  - `src/compose.rs`: parse `[requirement.*]` as the unified table; fold the old
    `[role.*]` fields (`artifact`→`kind`, `contract`, `match`, `count`/`membership`/
    `unique`, `degree`, `verified_by`) AND the requirement fields (`means`, `satisfies`
    target, `required`) onto ONE `Requirement` struct. Retire the `Role` struct + `[role.*]`
    key (accept `[role.*]` as a deprecated alias for one release ONLY if cheap; otherwise
    hard-cut — the surface isn't published yet, so a clean cut is fine and preferred).
  - unify `src/roster.rs` (match/count/membership/unique/typed-ref/conforms/verified_by)
    and `src/coverage.rs` (satisfies→requirement resolution, unfilled/dangling) so BOTH are
    checks over the one `Requirement`. Coverage stays the referential presence check;
    roster predicates become the multiplicity facet over the same requirement's filler set.
  - `src/graph.rs` (`degree` uses `Role`) and `src/main.rs` (`roster::{admissibility,
    check,conformance}` over `layer.roles()`) update to the unified type/name.
  - naming rule (`specs/90-spec-system.md`): `requirement` is the load-bearing term; no
    `role`, no `filled_by`, in code or specs.

- FOLD the two already-filed hardening entries into (or serialize immediately after) the
  consolidation — their intent SURVIVES on the unified requirement, but they touch the same
  files so must not run parallel to it:
  - `COVERAGE-HARDEN` (dedup `satisfies`, match-precision, doc cross-ref) → apply to the
    unified coverage.
  - `UNKNOWN-KEY-REJECT` → the spec now scopes stray-key rejection to `[requirement.*]` +
    `[kind.*]` + predicate clauses (NO `[role.*]` — that table is gone). Adjust the entry.

- PARALLEL-SAFE (disjoint from the consolidation, leave as-is): `REPRESENTATION-PRESERVE`
  (`src/import.rs`/`skill.rs`/`rule.rs` — the data-loss must-fix) and `BUNDLE-PLUGIN` (new
  `bundle` verb). These can run alongside the consolidation chain.

- DEFERRED still deferred: `COVERAGE-CUSTOM-KIND`, `PACKAGING-CHANNELS`, `AGENT-KIND`.
