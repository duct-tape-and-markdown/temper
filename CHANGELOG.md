# Changelog

All notable changes to `temper` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project aims
to adhere to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

`temper` is pre-1.0: while the major version is `0`, minor releases may carry
breaking changes. Releases are small and frequent.

## [Unreleased]

_Nothing yet._

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
