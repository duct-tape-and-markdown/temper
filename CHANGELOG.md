# Changelog

All notable changes to `temper` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims
to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

`temper` is pre-1.0: while the major version is `0`, minor releases may carry
breaking changes. Releases are small and frequent.

## [Unreleased]

## [0.0.20] — 2026-09-23

### Upgrading from 0.0.19

Each line is what a 0.0.19 harness may see first, then what to do.

- **A lock row's `source_path` changes on the next `emit`.** An `input()`,
  `include()` or layout import outside the program root used to record an
  absolute, machine-specific path. It now records a path relative to the
  program root (`../../src/x.txt`). Re-emit once and commit the lock; it is
  then the same on every checkout. A target on a different Windows drive has
  no relative path and is refused with the path named.
- **`membership` over a list-valued field may now report findings.** It used
  to pass silently whatever the list held. It now checks each element, and
  each element outside the allowed set is its own finding.
- **`unique` over a list-valued field is now a finding** saying that
  `unique` over a list is not defined, where it used to pass silently. If
  the field is meant to be a scalar, narrow it with a `type` clause.
- **The `guard` stops binding stray documents unless the contract asks it
  to.** A write of a document the program never declared, at a governed
  locus, binds the guard only where a `locus-declared` clause is bound. The
  shipped root default binds one, so nothing changes unless your root
  contract drops it. If you dropped it and still want the guard to catch
  strays, bind it again.

### Fixed

- **A source dependency outside the program root no longer breaks the
  lock's portability.** The committed lock is now byte-identical across
  checkouts, worktrees and machines, and it no longer carries Windows'
  `//?/` path prefix.
- **`membership` and `unique` no longer pass silently over a list.**

### Changed

- **The `guard` asks the contract.** Whether an undeclared document at a
  governed locus is a problem is the contract's call (`locus-declared`);
  what happens about it is the enforcement mode's (`block`, `warn`, `note`).
- **The reserved-`prose` refusal says to rename the field.** On an embedded
  value it no longer suggests authoring the words as member prose, which an
  embedded value cannot do.

## [0.0.19] — 2026-09-23

### Upgrading from 0.0.18

Each line is what a 0.0.18 harness may see first, then what to do.

- **`emit` fails with "leaf `prose` is reserved".** `prose` now names a
  member's own words, so an embedded value may not use it as a field name.
  Rename the field (`leaves: { words: prose }`) and the renderer that reads it.
  The projection bytes do not change.
- **`root.locus-declared` on `.claude/settings.json`.** The committed settings
  file is now governed whole. Declare a `settings` member (`settings({ name:
  "settings", … })` from `@dtmd/temper/claude-code`) and move the harness-level
  `settings:` keys into it. A key the settings reference does not document goes
  in its `residue: { … }` bag.
- **A hand edit to `.claude/settings.json` is refused by `guard` and reported
  by `check`.** The file is a projection now, so 0.0.18's allowance for
  residue-only edits is gone. Author the change in the program and re-emit.
- **A dial entry or CI filter stops matching.** These rule ids moved to clause
  labels: `config.stale`, `prose.include-stale` and `layout.import-stale` are
  now `root.fresh`; `locus.undeclared-member` and `layout.undeclared-member` are
  now `root.locus-declared`. `when` labels now carry the guard's values
  (`mcp-server.when.type=stdio`; seven shipped `marketplace` and `mcp-server`
  labels moved, six of them former collisions), a `when` body clause has its
  own label under its host's, and a filtered `degree` clause carries its fields
  (`state.degree.writes`). Respell each entry from the finding's new `rule` id.
- **A set predicate inside a `when` body is refused.** `degree`,
  `membership`, `count`, `unique` and `kind` in a guard's body were accepted and
  never judged. Move the clause to the kind's top level, or split the members
  into two kinds.
- **New findings may appear.**
  - `root.reachable` names a member whose every registration channel is dead;
    this check had not run since 0.0.8.
  - A member-grain clause bound through a requirement is now judged over the
    requirement's satisfiers.
  - An `@import` ring among memory files now fails `graph.acyclic`.
- **Workarounds for field-edge cycles can go.** A cycle through declared
  fields (view → signal → path → view) is legal now; only the import relation
  must be acyclic.

### Added

- **A root contract.** `harness({ contract })` binds clauses over the whole
  harness, and `rootDefaultContract` applies when a program declares none.
  It binds three predicates at `advisory`:
  - `reachable`: a member whose every registration channel is dead;
  - `fresh`: a lock row that no longer matches disk;
  - `locus-declared`: a document at a governed locus that the program never
    declared.

  Compose the array to change a severity, or dial the label (`root.fresh`
  at `required` makes a drifted pin fail CI).
- **`reachedFrom(roots, via)`.** Each selected member must be reachable from
  the satisfiers of the `roots` requirement over the named edge fields. It
  catches a member stranded behind another unreachable member, which a
  one-hop `degree` cannot. It follows cycles and ships in no default contract.
- **Field-filtered `degree`.** `degree({ incoming: { min: 1 }, fields:
  ["writes"] })` counts only the named edge fields. Two filtered bounds on one
  kind are separate clauses with separate labels.
- **Member inputs.** `inputs: [input(import.meta.url, "./code/login.asp")]`
  fingerprints a file a member's claims rest on without copying any of its
  bytes into the projection. When the file changes, `root.fresh` names the
  member and says to re-verify its claims before re-emitting.
- **The `settings` kind.** It governs the committed `.claude/settings.json`
  whole: documented keys are typed, the rest goes in a named `residue` bag, and
  the lock fingerprints the file. A `residue` key that shadows a typed field is
  refused.
- **Containment edges.** A host carries one `contains:<kind>` edge to each
  embedded member its body composes. An unfiltered `degree` ignores them; name
  them in `fields` to count them.
- **`span(words)`** builds a prose span from a computed string. The `` text`…` ``
  tag treats its interpolations as references, never as words.
- **`explain kind:<name>` for an unused kind.** A built-in kind the harness has
  no member of now narrates its locus, registration and address form, and a
  kind's leaf set shows before any member exists.
- **A layout member's own span** lands on its reserved `prose` leaf, addressed
  `<host-address>/<kind>/<key>/prose`.

### Changed

- **Acyclicity covers the import relation only.** Cycles through declared
  field edges are legal. `@import` directives are now checked, and before
  this release they were not.
- **`install`** brings an existing `.claude/settings.json` into the program
  as a `settings` member, and places its gate hooks through the program.

### Fixed

- **A CRLF file no longer mismatches its own fingerprint.** An include,
  layout-import or input target with Windows line endings was hashed raw at
  emit and line-ending-normalized at check, so it read as changed on every run.

## [0.0.18] — 2026-09-08

### Upgrading from 0.0.17

Each line is what a 0.0.17 harness may see first, then what to do.

- **A hand `Write` or `Edit` into a governed locus exits 2 in `block` mode**
  — `specs/*` under a kind that governs it, or an undeclared path under
  `.claude/` — with the locus named. Declare the member in the program and
  `emit` it; the path was never temper's to leave open.
- **An `Edit` to `.claude/settings.json` is judged by the file it would
  land.** An edit touching only residue temper does not own (`permissions`,
  `autoMemoryEnabled`) passes. An edit that drops a hook the lock declares
  (`guard.manifest-dropped-member`) or leaves the file unparseable
  (`guard.manifest-unparseable`) is refused; author the hook change in the
  program instead.
- **A stray document at a governed locus is a warn**
  (`locus.undeclared-member`), and `coverage.checked` now splits declared
  from undeclared per kind. Under `--deny-advisories` that stray fails the
  run, while a clean harness with only `coverage.checked` no longer does.
  Declare the document or move it out of the locus.
- **A bare nested-member key that two hosts carry is refused**, naming both
  hosts; the same key declared twice under one host is a malformed lock
  (`nested-member.admissibility`). Spell the full
  `<host-address>/<kind>/<key>`.
- **SDK: two kinds in play under one name refuse**, naming the kind and both
  loci, where 0.0.17 kept the first by authored order. `relocate(base,
  delta)` is the sanctioned spelling for widening a built-in's edge fields
  or moving its locus.
- **SDK types narrowed.** `embeddedMemberValue` leaves are
  `Record<keyof T, string | Text>` and `registration` is narrowed by locus,
  so `tsc` may fail where 0.0.17 passed; each failure is a leaf the engine
  would have refused at `check`.
- **Two lock re-spells show as drift once, then hold**: one
  `[[declaration.layout_source]]` row per layout document, and a label per
  clause on `section_contains` and `require_sections` rows. Re-emit once;
  both converge.
- **`check --reporter session-start` prints a payload on a load fault** (the
  fault's own rule id, `gate.load-fault` when the report carries none)
  where 0.0.17 printed nothing. A session that opened clean over a broken
  lock now opens with the fault in view; terminal, GitHub, and SARIF
  reporters still exit non-zero.

### Added

- `check` names a document at a governed locus the lock declares no member
  for (`locus.undeclared-member`, warn). A stray `.claude/rules/x.md` or
  `.claude/agents/x.md` that Claude Code would load, `emit` would never
  maintain, and `check` previously counted in silence now carries a finding
  naming the document and its kind's locus; the coverage line reports
  declared and undeclared counts per kind. Under `--deny-advisories` this
  exits non-zero. (GH #58)
- `guard` binds a write into a represented committed kind's governed locus
  when no member is declared there, at the kind's enforcement mode. In
  `block` mode a hand `Write` creating a file under any governed locus,
  `specs/` included for a harness that governs it, is refused with the
  locus named; declared projections keep their drift wording. (GH #58)
- `guard` refuses a `Write`, `Edit`, or `MultiEdit` that would leave a
  represented manifest unparseable (`guard.manifest-unparseable`). An
  unparseable `.claude/settings.json` makes the harness unloadable and
  Claude Code skips the file whole, hooks included, so the boundary is the
  only placement that can say so. (GH #40)
- `explain kind:<name>` is the authoring entry point: alongside guidance and
  cite it now renders the document shape a layout kind reads, the child
  kinds a host admits, the kind's locus and commitment, and the address form
  a reference to one of its members takes, before any member exists. `explain` also accepts the engine's own
  `<kind>:<name>` member address and the full `<host-address>/<kind>/<key>`
  nested spelling. (GH #47, #49)
- A nested member can be addressed by its full host-qualified address
  (`<host-address>/<kind>/<key>`) in edge fields and in the SDK's
  `embeddedMemberValue`, so two same-keyed members under different hosts no
  longer force a rename. (GH #50, #51; decision 0049)
- A built-in kind gains a sanctioned relocation form for added edge fields:
  `relocate(builtinKind, { edgeFields })` emits one `edge` fact row and no
  kind-fact change, and `{ governs: { root, glob } }` moves the kind's
  locus, the one fact the engine's relocation overlay applies. (GH #53, #54)
- SDK: `embeddedMemberValue`'s leaves are typed against the kind's field
  schema, and `KindFacts.registration` is narrowed per locus, so an omitted
  leaf or a registration on an embedded kind fails `tsc` rather than
  `check`. (GH #46)

### Changed

- A bare nested-member key that more than one host carries is refused at
  resolution, naming every host, in edges, mentions, and `explain`; it
  previously resolved to whichever member the scan reached first. A
  same-host duplicate `(kind, key)` is refused at admissibility. Address
  the member by its full spelling. (decision 0049, amended)
- Two `edge` fact rows spelling one `(from, field)` slot are refused at
  admissibility as a malformed lock; they previously resolved every
  reference twice and reported spurious `graph.route` findings.
- A layout declaring more than one verbatim prose region is refused at
  admissibility; only the first could ever carry bytes.
- `--deny-advisories` no longer fails a clean harness: disclosure notes such
  as `coverage.checked` are reported at a `note` severity that never
  blocks, and only declared-clause violations escalate. (GH #42)
- SDK: two kinds in play under one name are refused, naming the kind and
  both loci, unless one is a relocation of the other (the relocation wins)
  or the two are structurally equal. `kindsInPlay` previously kept the
  first by authored order, which dropped a relocated kind's added edge
  whenever the corpus had no member of it yet. (GH #53)
- Admissibility refusals name the declaration to open, not only the
  collision that exists.
- `emit` places the managed-by note for a frontmatter projection itself,
  so a fresh emit no longer reports `install.gate-installed` until a
  separate `install` converges it. (GH #57)
- Every `section_contains` and `require_sections` clause carries its own
  compiled label, so a kind may declare more than one of each. (GH #48)
- An embedded member's host rides its identity, not a synthetic key in its
  fields; a `closed-keys` clause on an embedded kind no longer indicts a key
  no author wrote.

### Fixed

- The `session-start` reporter runs over a harness that cannot be loaded.
  A load fault (malformed lock, non-UTF-8 member, duplicate declaration
  key) previously aborted before any reporter, so the SessionStart hook
  received empty stdout and the session opened with no verdict, the shape
  of a clean pass. It now carries one blocking finding whose rule is the
  fault's own code; terminal, GitHub, and SARIF reporters still exit
  non-zero.
- `guard` blocks under the invocation `install` wires (`temper guard .`):
  a relative root is resolved before a `file_path` is relativized against
  it, and an absolute `file_path` matches a single-segment target such as
  `CLAUDE.md`. `mode: block` on the root memory file was cosmetic before
  this. (GH #39)
- `guard` judges an `Edit` or `MultiEdit` to a represented manifest by the
  manifest it would land, so an edit touching only co-owned residue of
  `.claude/settings.json` (`permissions`, `autoMemoryEnabled`) is allowed
  and a member the lock declares cannot be dropped through an edit.
  (GH #38, #40)
- `.claude/settings.json` is recognised as emit-owned: registration
  members live under `[[declaration.registration]]`, which the ownership
  scan did not read, leaving the manifest unguarded.
- A discovered layout document the lock declares no member for is named
  (`layout.undeclared-member`) instead of reading as an empty host. Emit
  records every layout source it reads on the lock, so a declared layout
  member whose document lowers into no content row (field regions only, or
  an empty collection) is not reported as undeclared. (GH #43)
- A layout's positional heading binding reports the mismatch when a leading
  title or a missing section shifts every later region. (GH #41, #45)
- `install` reports a synthesized hook placement an authored hook member
  supersedes as its own outcome, and `check` no longer flags that hook as
  needing install forever.
- A render hook or embedded value can cite a nested member as an edge
  target; the member table previously indexed top-level members only.
  (GH #50)
- A local-commitment layout member's captured prose reaches `explain`;
  the read kept its collection and fill rows and dropped the spans.
- `explain` on a full nested-member address renders the member and stops;
  it previously appended a "not a well-formed leaf address" refusal after
  a successful resolution.

## [0.0.17] — 2026-09-04

### Fixed

- `emit` no longer refuses a harness that authors two `hook` members on one
  event. 0.0.16's duplicate-identity refusal keyed every member by
  `kind:name`, and a hook's name is its event, so two `PostToolUse` groups
  (a shape Claude Code admits) failed with `duplicate identity key
  hook:PostToolUse`. A projected member's address is its file and still
  refuses a duplicate; a registration member's address is its group key
  and admits several. If 0.0.16 broke your `emit`, this is the fix.

## [0.0.16] — 2026-09-04

### Fixed
- `rule.mention-reachable.paths` no longer reports a strict-subset scope as
  unreachable. Containment compared glob strings for equality, so a rule
  scoped to `database/x/**/*.sql` mentioning a skill gated on `**/*.sql`
  was flagged. Containment is now decided over the path sets the globs
  denote, brace alternation and negated classes included.
- An `at` locus rooted under `.temper/` is refused at `emit` and at `check`
  with a named error. Discovery fences the workspace by design, so such a
  kind emitted and locked its members while `check` never read them back:
  a silent gate blind spot that surfaced only as a zero in the coverage
  line.
- A requirement's declared `kind` is held against its satisfiers. A member
  of another kind that claims to satisfy a kind-narrowed requirement is a
  required-severity finding, and `explain` no longer lists it as filling
  the requirement.
- `temper guard` binds `.claude/settings.json`. The file is the spliced
  projection of every `hook`, `installed-plugin`, and `known-marketplace`
  member, but it was not in the guard's target set, so every spelling of a
  direct edit to it was silent.
- Two `hook` members declaring the same event no longer share one address
  silently on the edge-resolution side of a duplicate-key map; the engine
  and the SDK now build every identity map through one constructor that
  refuses a duplicate key by name. (The hook kind's own identity is a
  separate open question, tracked as GH #32.)

- `temper guard` no longer binds unrelated files that merely share a
  projection's filename. The guard compared a declared projection against the
  incoming path as a bare string suffix, so a root `CLAUDE.md` projection
  bound every `CLAUDE.md` anywhere in the tree, the authoring source under
  `.temper/memory/` included. The match now has to land on a path-segment
  boundary.
- A mention addressing an embedded leaf (`<member>/<kind>/<key>/<path>`) no
  longer dangles at `check`. `emit` accepted the address while `check`
  parsed it as a bare requirement and looked in the roster; both verbs now
  resolve it through the same leaf resolver.
- Two kinds declaring the same `collectionAddress` are refused at
  declaration with a named finding (`kind.collection-address-collision`).
  Previously each kind's selection silently became the union of every entry
  at that address, so counts multiplied by the number of kinds and one
  kind's clauses indicted another kind's members.
- `temper tap` records for `InstructionsLoaded` now join to their members.
  The record carried an absolute path and the reader compared it against a
  member id, so no instructions-loaded evidence had ever joined on any
  machine. The tap record is now version 2: identity is repo-relative,
  each record carries an ISO-8601 timestamp, and the reader maps a path to
  its member through the lock, so nested `CLAUDE.md` members with folded
  ids join too. Version 1 records still read.
- The synthesized tap hook runs `temper tap "$CLAUDE_PROJECT_DIR"`, so a
  hook firing from a subdirectory or a linked worktree appends to the
  primary checkout's log instead of a log that dies with the worktree.
  Re-run `temper install` to pick up the new hook command.

### Added

- The `skill` default contract carries `mention-reachable(paths, paths)`,
  matching the `rule` contract: a skill that mentions a member gated
  narrower than its own `paths` is flagged at advisory severity.
- `EdgeTargetFacts` gains `repoRootedPath` beside `path`. `path` stays
  relative to the host's projection directory; `repoRootedPath` is what a
  reader resolving from the repository root needs, such as a skill body
  citing a rule.
- `temper install` places a `PostToolUse` hook on the `Bash` tool that runs
  drift detection over emit-owned targets after the call. The `PreToolUse`
  guard binds tool-mediated writes only (`Write`, `Edit`, `MultiEdit`), and
  its messages now say so; a shell-mediated write is caught after the fact
  instead of never.
- The built-in kind × clause matrix is a reviewed snapshot in the test
  suite, so a clause present on one kind and absent on its twin is a
  visible diff rather than a default.
- An emit-then-discover round-trip test over every locus shape asserts
  that `check` reads back exactly the member set `emit` locked.

### Changed

- `coverage.checked` marks embedded-locus kinds as `<kind> (N embedded)`
  instead of `(0)`. The zero was correct for artifact members and read as
  "this kind's clauses never evaluate," which they do.
- The `InstructionsLoaded` tap hook registers for every documented
  `load_reason` (`session_start`, `nested_traversal`, `path_glob_match`,
  `include`, `compact`), not only `path_glob_match`, so always-on members
  loaded at session start are recorded.
- `emit` places the managed-projection banner on every frontmatterless
  markdown projection it owns. The banner was placed by `install`, so a
  member added by a flow that ran `emit` alone shipped bannerless and only
  an advisory noticed. The banner is now part of the projected bytes and
  the lock hash; on first `emit` after upgrading, frontmatterless
  projections that lacked it are rewritten once. The read face strips it
  from a member's `body`, so `extent`, headings, and layout contracts see
  authored prose only.

## [0.0.15] — 2026-07-27

### Fixed

- `temper tap` no longer loses records inside a linked git worktree. The tap
  resolved its log path against the working directory, so a hook firing in a
  worktree wrote to that checkout's own `.temper/tap.jsonl`, an untracked
  file deleted with the worktree. The tap now follows the worktree's `.git`
  file to the primary checkout (relative `gitdir` and `commondir` paths
  included, per gitrepository-layout) and appends there, so telemetry
  survives worktree cleanup. Hooks stay `temper tap` with no argument.
- `explain` on an unfilled requirement with a telemetry verifier narrated no
  evidence at all. It now reports counts against the declared member corpus
  when the requirement has no satisfiers.

### Added

- `explain <member>` states telemetry absence instead of staying silent:
  when the lock declares tap registrations and the log holds no records, the
  field strand says so. A quiet log with wired hooks is now a visible fact,
  which is how a stale pre-tap binary on PATH was caught in the field.
- `explain <requirement>` narrates a telemetry verifier's field record: per
  declared event, the record count and the distinct members and sessions,
  plus the declared members with zero records (the dead-weight list).

## [0.0.14] — 2026-07-24

### Fixed

- CRLF checkouts no longer lose managed-by notes or report drift on correct
  projections. The frontmatter reader accepted only an `---\n` opening
  delimiter, so on a working tree git rewrote to CRLF (the `core.autocrlf`
  default) every frontmatter-carrying projection read as frontmatterless:
  `emit` dropped the managed-by note it should have preserved, the schema
  modeline was never placed, `install` did not converge, and `check` reported
  `install.gate-installed` drift on files that were correct. The reader now
  accepts `---\r\n` as well. A repository that committed stripped notes
  restores them with one `temper install`.

## [0.0.13] — 2026-07-24

### Added

- `explain` now narrates a bare kind: `temper explain <kind>` (or
  `kind:<name>`) reports a kind's authoring guidance before any member of
  it exists.
- An embedded kind's guidance now reaches every delivery surface. It takes
  a kind-fact row in the lock (locus columns absent), so `schema` and
  `explain` carry its counsel like any other kind's.
- `schema --kind` serves every YAML-frontmatter kind, built-in or declared,
  not just `skill` and `rule`. Kind and field guidance ride the schema as
  editor hover text.

### Fixed

- `install`'s lift scaffolds each kind into its own directory. A command
  and an agent sharing a name no longer overwrite each other's scaffolded
  module.
- `schema --kind`'s help and unknown-kind error report the live kind
  domain instead of a hardcoded list.

## [0.0.12] — 2026-07-23

### Fixed

- Windows checkouts no longer report every projection as drifted. Drift
  comparison treats line endings as layout: a working tree git rewrote to
  CRLF (the `core.autocrlf` default) reads clean against the LF-emitted lock,
  while `emit` still writes LF.
- `check` no longer aborts on a malformed or nameless member. The load fault
  is collected as a diagnostic and the run continues, so every finding
  surfaces on a foreign harness instead of the first crash hiding the rest.
- `command` frontmatter is fully optional, matching Claude Code — the
  invocation name comes from the filename. A command missing `name` or
  `description` is no longer flagged.
- The `install` gate-installed advisory no longer fires on a repository that
  has not adopted temper (no `.temper/`).
- Placed hook commands fail loudly when the `temper` binary is not on `PATH`,
  instead of dying silently and leaving the gate unreported.
- An engine/SDK version skew reports a version hint rather than a bare
  payload-parse error.
- The session-start reporter surfaces advisory diagnostics, not only blocking
  findings.

## [0.0.11] — 2026-07-21

Entries begin here; earlier `0.0.x` releases predate this changelog.

### Added

- Kind guidance now flows into the contract and rides its findings: `schema`
  carries it as editor hover text and `explain` narrates it, so guidance and
  the member it advises travel together whether or not a clause failed.

### Fixed

- `when`-body clauses now evaluate at the guarded array element's scope, so a
  conditional requirement no longer judges the wrong element.
- Root-scoped `when`-guard findings no longer carry a stray `: ` prefix.
- Import recursion is capped at four hops (the guard was off by one).
