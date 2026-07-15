<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `temper check <harness-root>` is a **silent half-gate**: with workspace
  `.` at a harness root, `drift::read_declarations(workspace)` finds no
  lock, so requirements, satisfies fills, custom kinds, and declared edges
  all vanish while built-in kinds still resolve off disk — exit 0, looks
  green. Verified in the field (examples/base-harness): `check .` stayed
  green with a `required: true` requirement unfilled; `check .temper` exits
  1 with `requirement.unfilled`. Bare `temper check` (no arg) resolves
  correctly, so the trap is only the explicit spelling — and
  `src/install.rs:88` **hardcodes** `temper check . --reporter
  session-start` into every adopted harness's SessionStart hook, so every
  install ships the half-gate reporter. The empty-assembly tripwire
  (check.rs) only fires at resolved_members == 0. Both this repo's and the
  example's hooks hand-fixed to `.temper` (aba7e47, 549969f); the product
  fix is install's wiring + the argument semantics (resolve `<arg>/.temper`
  like the default does, or fail loud). Verify `temper guard .`'s rooting
  while there — same argument shape. observed at aba7e47
- A composed **mention cannot target a discovered member**: authoring
  `text\`… ${{address: "source:main", display: "src/main.js"}} …\`` in the
  example's skill fails emit with "mention of `source:main` resolves to no
  declared value — a mention cannot dangle" (sdk/src/emit.ts,
  renderTextBody). But `source:main` is a real corpus member — the engine
  resolves `implemented-by` edges against it and `graph.route` fires on it.
  The SDK enforces corpus law against a program-scoped universe (declared
  program values), the same shape the fill-check deferral resolved: SDK
  keeps failing fast on definitely-dangling program addresses, and defers
  discovery-locus kinds' addresses to the engine's check-time mention
  edges. Blocks the primer's skill→script edge demo. observed at aba7e47
- `temper emit --into <dir>` pointed **inside an adopted harness** reaped 7
  live projections at the repo root (CLAUDE.md, five rules, a skill) —
  `--into .temper` re-rooted the projection tree, the real projections
  became ownerless, and the byte-faithful safety line let them be deleted.
  Restored via git; plain `emit` then reported 0/8/0 clean. The reap fix
  (e7b859a) covered path spelling, not re-rooting: emit should refuse (or
  dry-run-report) a reap wave caused by an `--into` that re-roots an
  already-adopted harness. observed at aba7e47

- Decision 0022 (`specs/decisions/0022-glob-validity-joins-the-vocabulary.md`,
  human-ruled 07-15) resolves the `(builtins-coverage-predicates)` fork:
  admit a **glob-validity predicate family** (globs parse under `globset`,
  brace-aware), first consumers the `rule` and `skill` default contracts
  over `paths`; `tools-must-resolve` rejected permanently on invariant 2
  (recorded in 0022 — do not re-file). Work: `Predicate` enum variant +
  schema surface in `src/contract.rs`, the two default-contract clauses in
  `sdk/src/builtins.ts` with fresh raw cites, frozen-lock re-derive +
  tests. The two deferred skill shape predicates are NOT in scope (0022 is
  explicit they need their own design). observed at dc43554
