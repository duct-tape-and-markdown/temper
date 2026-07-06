# Plan state

- **Phase:** residue sweep, quiescent. Spec-delta empty (no `specs/` commit
  since e227a27); inbox empty; `open-questions.md` already drained to OPEN
  forks only (07-06 anti-accumulation rule). The sweep ran regardless and
  found no un-derived residue.
- **Last shipped:** PKG-NOUN-EXPLAIN-DIAGNOSTIC (build 54a4e9a / chore
  48b875c) — re-cut `explain`'s "Governing-package" line + doc comment to
  floor/kind/clause vocabulary and dropped the stale read.rs/builtin.rs spec
  cites. That was the package noun's **last** diagnostic residue.
- **This tick — the package-noun retirement is COMPLETE on disk.** Verified:
  no `conforms_to` and no conformance pass (every `conformance` hit is an inert
  comment); no `package` facet on `Requirement` (compose.rs:78-98 carries
  count/unique/membership/degree/verified_by only); no package resolver keyed
  off a requirement; no package-vocabulary diagnostic in read.rs/builtin.rs; no
  `KIND.md` header-grammar parser. The only surviving "package" uses are
  legitimate — the sanctioned std-lib `PACKAGE.md` embed (bundle.rs), the
  `floor_package`/floor-binding helpers, and the npm/SDK manifest domain. The
  stale "`kind::BUILTIN_KINDS` const survives / KIND-BUILTIN-CONST-RETIRE
  fileable" claim is **false**: no such const exists — `rg BUILTIN_KINDS
  src/kind.rs` is empty; the only one is the live, used `builtin_kind::
  BUILTIN_KINDS` (builtin_kind.rs:27, referenced :323). No entry filed.
- **In flight:** 2 entries, **both parked / human-gated — nothing pickable.**
  (1) REQUIREMENT-CLAUSES-RECUT — re-verified live this tick that all four
  facets still stand cross-language: compose.rs `CountBound`:105 /
  `DegreeBound`:118 / `Membership`:158 + the `Requirement` fields;
  drift.rs `RequirementRow` count/unique/membership/degree columns (:978);
  roster.rs set-scope checks; sdk/contract.ts `count?`/`unique?`/`membership?`/
  `degree?` :83-86 with **no** `clauses?` field; SEAM_VERSION=1 both sides.
  A cross-language serialized reshape (facets → `clauses` array + SEAM bump)
  needing a decomposition ceremony (John + session), like GATE-READ-LOCK got.
  (2) PACKAGING-CHANNELS — release infra (release.yml + optionalDependencies),
  parked on human release creds + the engine-binary workflow + USPTO screen.
- **What's next (all human/foundation-gated):** the REQUIREMENT-CLAUSES-RECUT
  decomposition ceremony (folds in reachability-as-edge-scope-clause and the
  held general-degree-band predicate — a design fork the autonomous phase must
  not decide); PACKAGING-CHANNELS release setup; the `(genre-fence-format)`
  workshop (cascade pilot); the OPEN forks in open-questions.md
  (`(authority-home)`, `(edge-representation-unify)`,
  `(builtin-workspace-qualified-key)`, `(json-projection-format)`, …), each
  needing John or the SDK-primary foundation.

Plan continues: no — pending fully reconciled (both entries re-verified,
correctly parked), the package-noun sweep came back clean, inbox drained,
spec-delta empty, no fork newly resolved. No open pickable entry exists: the
queue is genuinely blocked on two human ceremonies. Re-planning the same
parked queue would be spin, not diligence.
