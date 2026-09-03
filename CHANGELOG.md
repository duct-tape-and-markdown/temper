# Changelog

All notable changes to `temper` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims
to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

`temper` is pre-1.0: while the major version is `0`, minor releases may carry
breaking changes. Releases are small and frequent.

## [Unreleased]

### Fixed

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

### Changed

- `coverage.checked` marks embedded-locus kinds as `<kind> (N embedded)`
  instead of `(0)`. The zero was correct for artifact members and read as
  "this kind's clauses never evaluate," which they do.
- The `InstructionsLoaded` tap hook registers for every documented
  `load_reason` (`session_start`, `nested_traversal`, `path_glob_match`,
  `include`, `compact`), not only `path_glob_match`, so always-on members
  loaded at session start are recorded.

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
