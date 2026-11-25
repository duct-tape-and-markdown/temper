# 0100 — one command reference

- **Date:** 2026-11-25 · **Status:** accepted

## Context

Three decisions each added a "which binary" field: the gate hook (0082),
the tap (0088) and the typecheck (0098). A probe of a large real harness
found 28 hook scripts and a status-line script that temper could not
see, because they sit inside command strings, and parsing those strings
would be mining. John proposed emitting `.temper/package.json` with
command intent in the type. The repo-build-brief design session reviewed
it from a large monorepo's toolchain work:

- the costly failure is "does anything run this check": about half of
  one suite's declared checks ran nowhere;
- `package.json` has many writers, so as a projection every routine add
  becomes drift that agents learn to route around;
- intent in a type pays only where something reads it.

## Decision

**A command the harness runs is one typed reference, pointed at by what
runs it** (`authoring.md`):

- `{ package, bin }`: the version comes from the lockfile, read and never
  run, and the path is the anchored `.bin` (0086). The guard runs on every
  tool call, so the path is not resolved through the manager at run time;
- `{ file }`: a committed source that joins the roster as an input (0064);
- `{ system }`: claims presence on the PATH and no version.

Gate hooks, taps, the typecheck and script verifiers reach their command
through declared edges, and a command nothing runs is an advisory finding.
`package.json` is a source, scaffolded once and never written. Its
`packageManager` field declares the manager, read and never guessed. The
three `binary` fields fold into this reference.

## Rejected

- **`package.json` as a projection.** It has other writers, which
  invariant 7 forbids for a projection.
- **Resolving through the manager on each call.** That adds latency to
  every tool call to defend against rare layouts, which refuse loudly
  instead (Yarn PnP).
- **Consumer script subkinds that carry intent.** Nothing reads them.

## Consequences

Deferred, and named so they are not lost:

- environment and shell per edge, which wait for a real failure;
- system-tool presence, which goes in the session-start report, never in
  `check`'s exit code;
- cache inputs, which are out of scope.

UNVERIFIED: that Windows Claude Code runs hooks under a shell where the
`.bin` shell shim works. Cite it before shipping the Windows path.
