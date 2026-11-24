# 0080 — the program's module closure is fingerprinted

- **Date:** 2026-11-24 · **Status:** accepted

## Context

GH #61: a `settings` member whose permissions are computed from a list
imported out of another workspace package passes `root.fresh` after that
list changes. Drift is reported only once the member declares the module
with `input()`. Emit fingerprints three source-dependency families (layout
imports, includes, declared inputs), and nothing fingerprints the
program's own modules, while the `fresh` clause's guidance says a
dependency the member "imports" has moved. The result is a false green
that invariant 6 forbids. Ruled by John 2026-11-24 on the session's
recommendation.

## Decision

**Emit fingerprints the program's module closure**: every local module
the run loaded, `node_modules` excluded, gets a provenance row
(`pipeline.md`, "The lock"). A moved module is an authored source owned
by the root member, so the finding is one root-level `fresh` with the
remedy to re-emit (`pipeline.md`, "Drift"). An import is a reference the
authoring language executes, declared by being written, the same shape as
a memory file's `@path` import edge, so fingerprinting it is not mining.
`input()` narrows to the non-code files a member's claims rest on
(`authoring.md`, "The SDK").

## Rejected

- **Per-member attribution.** Each member would own only the modules its
  own defining module reaches. The SDK would need a map from members to
  modules, and a shared helper module makes nearly every member stale
  anyway. The remedy, re-emit, is the same either way.
- **Keep `input()` as the only declared dependency** and reword the
  guidance. That is the cheapest option, but it ships the silent false
  green, since the author has to remember a dependency the program already
  states.

## Consequences

The SDK reports the loaded module set to emit, and check re-hashes it
offline, with no Node. The `fresh` guidance stops treating "imports" and
"includes" as one word. An `input()` that names a TS module becomes a
second spelling of one fact (0078), which a clause or load refusal can
name.
