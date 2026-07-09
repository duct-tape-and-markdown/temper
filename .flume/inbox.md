<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- T18 — a built-in host's lock-declared `templates` are never applied
  (observed at 3ff0dd1). Confirmed on disk, unaffected by either later
  fix (`TEMPLATES-RELOCATION-COLLISION-REGRESSION` only stopped a false
  collision on a templates-bearing row; `MEMORY-ENTERS-REQUIREMENT-CORPUS`
  generalized the features/corpus-assembly side, not kind construction).
  Now unblocked and ready to fix. Mechanism: `from_kind_fact_row`
  (`kind.rs:225`) lifts a lock row's `templates` correctly — confirmed
  by its own passing test — but `gate()`/`explain()` construct every
  built-in kind purely from `builtin_kind::definitions()` (the compiled
  struct), never `from_kind_fact_row`; the only overlay applied to a
  built-in from its lock row is `effective_governs` (`main.rs:769`),
  which — per its own doc comment, still true today — carries `governs`
  alone. So a lock row declaring `templates` against a built-in host
  (e.g. `rule` gaining a `directive` child kind via `withinHosts:
  ["rule"]`) is silently inert: `CustomKind::extract`/`fold_members`
  (`kind.rs:257-300`) would fold a `member.directive <key>` fence
  correctly the moment `self.templates` is populated, but for a
  built-in kind it never is. Fix shape (per prior discussion, not
  re-litigated here): generalize `effective_governs` into one function
  overlaying both `governs` and `templates` from the matching lock row,
  one lookup, used at every current call site — not a second
  `effective_templates` alongside it (`specs/process/engineering.md`,
  "one job, one home"). Real-world repro: a `harness.ts` declaring a
  `directive`-shaped template against `rule`; three fence-embedded
  rules in the reporting harness are byte-present and correctly
  extractable in principle, but dark to `explain`/`check` today —
  `explain cls` (or any built-in-hosted directive) reports no nested
  members regardless of what the fence actually contains.
