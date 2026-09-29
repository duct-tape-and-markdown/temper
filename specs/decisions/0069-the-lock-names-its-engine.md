# 0069 — the lock names its engine; the version is the tag's

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(build-version-identity)` fork. A source build and the published
binary print the same version, and that string is written into two
artifacts — the bundled plugin manifest and the SARIF driver. An adopter
ran hooks on a cargo-installed 0.0.15 against a 0.0.18 pin, writing 834
tap records at the wrong version with nothing flagging it; this repo ran
0.0.18 against a published 0.0.20. Resolving the pinned binary from
`node_modules` would break the stranger gate. Ruled by John 2026-09-29
on the session's recommendation.

## Decision

- **The version string is the release tag's**, identical in every build
  of that tag and every artifact it stamps. Build provenance rides the
  long `--version` form alone.
- **The lock names the engine version that wrote it**, and a gate run by
  a different engine says so at its clause's declared severity — a root
  default clause at advisory. The check stays offline and lock-only, so
  it works for the bare binary too.

## Rejected

- **A `git describe` suffix on the version.** It makes the version a
  build-environment fact, so a tarball and a git build of one tag
  disagree and the bundle stops reproducing from the tag.
- **An environment override naming a binary.** Identity travels by the
  SDK's pin; a second identity source is a second place to be wrong.

## Consequences

A lock column for the emitting engine, normalized at read for older
locks under 0024. A root default clause for engine mismatch. A long
`--version` with the build's commit.
