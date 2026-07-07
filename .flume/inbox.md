<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

Field report 2026-07-07 (Windows 11 / centercode-platform testbed, 16
members, at 18dca38). Nine findings; verified fix shapes noted. Route each:

- WIN npm spawn: `install --yes` dies at `Command::new("npm")`
  (src/install.rs:1032) — Windows ships only `npm.cmd`, Rust spawn resolves
  `.exe` only. Fix verified: spawn `npm.cmd` under `cfg!(windows)` (or
  `cmd /c npm`). SAME ENTRY: the failure lands after scaffold, leaving a
  half-scaffolded `.temper/` — reorder so dependency assurance precedes
  scaffold (pipeline.md install: "no half-scaffolded state").
- WIN verbatim path: `emit` feeds Node `fs::canonicalize`'s `\\?\`-form
  entry path (src/drift.rs:382); Node dies in resolveMainPath. Keep the
  canonicalize, strip the verbatim prefix (`dunce` crate is the standard
  fix). Verified: with prefix stripped, first emit reproduces 16 projections
  byte-identically.
- SDK version range: src/install.rs:58 pins `^0.0.2`; npm caret on 0.0.x
  pins the PATCH (>=0.0.2 <0.0.3), so fresh installs get 0.0.2 — which
  predates the `file` export the scaffold imports — on every OS. 0.0.4
  (seam v2, has `file`) is published. Bump the range to match the seam the
  binary speaks; scaffold output + version range are one contract, changed
  in one commit.
- Path separators: scaffolded modules and lock rows embed `\` on Windows
  (`file("../.claude/rules\\cls.md")`, mixed-sep source_path). `.temper/` is
  committed and cross-platform; normalize both writes to `/`.
- guard vs own_path: guard regex-matches any `.claude/` file_path without
  consulting the lock; for `file()`-carried members the `.claude/` file IS
  the authored source (install itself skips placements on them; emit adopts
  direct edits). Under block it denies editing the source of truth. Guard
  reads the lock; own_path targets pass through.
- bundle report: on a 16-member surface `bundle` reports "surface: 0 skills,
  0 rules". Spec reads gate-delivery-only (distribution.md channel 3: the
  skill + SessionStart hook), so the composition is right and the REPORT
  line is the bug — name what the bundle ships, not member counts. If plan
  reads member delivery as intended instead, that is a fork, not an entry.
- explain not-found: `explain protocol` prints the not-found error (with a
  retired `import` verb suggestion) then narrates the member anyway — the
  resolver finds it one namespace later than the message checks.
- help-text recut: `guard --help` still speaks shared/surface (0006 recut to
  note/warn/block; runtime already migrated); most verbs' help cites the
  deleted specs/architecture/* paths. User-facing strings — OUTSIDE
  CITE-RETAG's comment scope; file separately.
- settings.json fidelity: install's hook merge is semantically correct but
  re-serializes the whole file (keys reordered, EOL churn). The round-trip
  discipline the corpus states for TOML/markdown has no stated JSON
  equivalent — if no clean cite exists, route as an open question
  (key-order-preserving insertion is the candidate ruling).

Field report 2 (2026-07-07, same testbed, deeper behavior probes at
18dca38). Route each:

- T1 requirement rows never gated: the lock carries the full
  [[declaration.requirement]]/[[declaration.satisfies]] families (44 rows on
  the testbed) but `check` evaluates NONE — a live probe violating
  count({max:1}) produced zero findings on `check .` and `check --harness .`;
  the require/satisfies layer enforces only at emit. contract.md
  (requirement: attached clauses over its opt-in selection, shipped default
  cardinality clause at error severity) + pipeline.md (the gate reads
  declarations from the lock) sanction the fold-in. The largest correctness
  gap on the board.
- T2 explain ignores lock declarations: with a satisfier locked, `explain
  <requirement>` reports "required, and unfilled — which check reports as an
  error" (check reports nothing — doubly wrong per T1); member explain still
  prints the not-found error (citing the removed `import` verb) then
  narrates the member anyway. contract.md Read verbs: projections over the
  same resolved edges the gate uses.
- T3 removed member orphans its projection: deleting a member from the
  program and re-emitting reports "16 unchanged" and leaves the emitted
  .claude/ file on disk — unowned, still-loading. The lock forgets the path;
  no removed state, no reap, no report. Live hazard. If emit's "Total"
  (pipeline.md) doesn't decide reap-vs-report, route the remedy as a fork.
- T4 ungoverned-entry sweep: coverage notes enumerate known-unmodeled
  surfaces (.claude/commands, settings.json) but a bogus `.clauignore`
  sails through. A decidable "entry in .claude/ matching no kind's governs
  and no known surface" advisory is exactly the charter.
- T5 file() resolution: prose.ts documents module-relative; emit resolves
  against baseDir ?? cwd (workspace-relative). Scaffold's ../.claude paths
  work only because .temper/ is one directory deep. One of the two changes;
  coordinate scaffold + resolution + docs in one entry.
- T6 scaffold emits type-invalid modules: Skill.description is required in
  the SDK types; scaffolded skill modules omit it — runtime-fine (types
  erase) but the program fails tsc, the advertised keystroke wall. Scaffold
  output must typecheck.
- T7 emit line endings on Windows: projections normalize to LF, tripping
  git's LF/CRLF warnings; projected files mix LF frontmatter with CRLF
  bodies. Byte-reproducibility needs a deterministic EOL policy — candidate:
  always LF, stated; if the corpus is silent, a small fork.
- T8 vocabulary for the remaining surfaces (FORK, needs John): slash
  commands need a registration value for user-invoked (existing vocabulary:
  always / description-trigger / paths-match / event / connection); hooks
  and settings need kinds. Until ruled, the two coverage advisories are
  permanent. Joins the (json-projection-format) family.
