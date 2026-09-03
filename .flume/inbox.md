<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at d3848196 (interactive review of 4d7e8127, GH #33 fix) — the
  new `is_scope_contained_in_gate` (`src/graph.rs`) decides subset by
  generating witness paths from the scope glob and matching them against
  the compiled gate set. Matching rides `globset`; the witness generator is
  a hand-rolled segment rewrite that concretizes `*`→`file`, `?`→`x`, and
  strips brackets, and never expands brace alternation, so a scope like
  `**/*.{sql,md}` yields the literal witness `file.{sql,md}` and reports
  uncontained against a gate `**/*.sql` (false positive survives for brace
  globs) while a char-class scope `[!a]*.sql` concretizes to `!a*.sql`
  (unsound witness). Low severity, posture-sweep grade: either widen the
  witness generator to the glob vocabulary `globset` accepts (braces, `!`
  classes) with a table-driven test, or document the supported subset in
  the finding text. Not a release blocker.

- observed at e77766c5 (defect-class review, human-directed 09-03) — CLASS 2,
  emit's codomain vs check's domain. GH #30 (workspace-rooted locus emits
  and locks, never discovered), #27, PR #23's relative-root loss, and the
  untracked-file half of #38 all surfaced as a quiet zero where the two
  verbs disagree on the member set. `tests/acceptance.rs`'s
  `acceptance_check_then_reemit_is_a_no_diff` proves one direction; its
  twin is missing. File: for every kind the lock declares (built-in and
  custom, file and embedded loci, relative and absolute root spellings),
  `emit` then discovery returns exactly the locked member set — a
  parameterized round-trip over a fixture carrying every locus shape. One
  test, catches the class at the gate. Highest priority of these notes.

- observed at e77766c5 (defect-class review, human-directed 09-03) — CLASS 4,
  a capability added to one of a pair, never its twin. GH #36 (rule has
  `mentionReachable`, skill lacks it), #37 (install places the banner,
  emit does not), #38 (guard matches Write|Edit, not Bash; projections,
  not the spliced manifest). The built-in contracts are a kind × clause
  table nobody can see: `sdk/src/builtins.ts` declares 14
  `*DefaultContract` arrays and `src/builtin_lock.toml` carries them as
  rows. File: an `insta` snapshot rendering the matrix (kind rows, clause
  keys as columns, cell = severity or blank) from the embedded built-in
  lock, so an asymmetry is a reviewed diff. Pure test; no engine change.

- observed at e77766c5 (defect-class review, human-directed 09-03) — CLASS 3,
  paths and globs compared as strings with "relative to what" unstated.
  GH #33 (glob-string equality for containment, fixed 4d7e8127), the guard
  suffix bug (f1401cc4), PR #23's `CurDir` mismatch, 2fa5efd7's tap
  identity, #31 (host-relative path read from repo root). Two parts:
  (a) newtypes per root — repo-rooted, harness-relative, host-relative —
  so a path fact's root is its type, not its doc comment; #31's fix added
  a second string field where a type would have forced the question.
  Scope the first cut to the facts crossing the SDK/engine seam
  (`EdgeTargetFacts`, lock `source_path`, tap identity). (b) a
  posture-sweep rule: `ends_with`/`starts_with`/`==` on a path-carrying
  string outside `src/path.rs` is a finding (19 candidate sites today,
  most legitimate — the rule names the sanctioned home, the sweep judges).
  (a) is an entry chain; (b) is a harness rule (human commit), surfaced
  here so plan does not file it.

- observed at e77766c5 (defect-class review, human-directed 09-03) — CLASS 1,
  silent collapse on a non-unique key. GH #26 (fixed 21f61463), #32 (parked
  on (hook-member-identity)), PR #23's one-row round-trip. Each is a map
  built by insert on a key never proven unique. File: one refusing index
  constructor per side — engine (a `BTreeMap` wrapper that returns a
  named collision error) and SDK (`emit.ts`'s `Map<string, ...>` builders
  at ~442/482 and the `kind:name` composition map) — and every identity
  map built through it. Consolidation entry per engineering.md ("a shared
  concept is one type"); name the unification in the commit body. Should
  wait for (hook-member-identity) only where the hook map is concerned;
  the constructor itself does not.

- observed at e77766c5 (defect-class review, human-directed 09-03) — cross-
  cutting: every defect in the batch came from a foreign harness, none
  from the self-mirror. `tests/gauntlet.rs` snapshots emit+check over an
  in-repo corpus. File: extend the gauntlet with two or three real
  external harness fixtures (vendored, license-checked, trimmed to their
  `.claude/` trees) so the July external-yield probe becomes a standing
  gate. Candidates are in `docs/ledger.md`'s probe note; the fixture
  choice is human (licensing), the test wiring is build's.
