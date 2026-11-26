# 0110 — temper ships a starter harness

- **Date:** 2026-11-26 · **Status:** accepted

## Context

John proposed a default harness, like flume's harness package: a starting
point that puts the core patterns in place, which a project then
customizes. Three shapes were weighed:

- an importable package;
- an example-style scaffold of owned files, readable but frozen at install;
- a tracked scaffold that merges upstream changes, which works but is
  heavy to build.

John, 2026-11-26: "a simpler approach is to provide it as a package, and
also keep example and guidance for building with the sdk available."

## Decision

**The SDK's `starter` subpath ships temper's default harness as importable
values** (`distribution.md`, "The SDK"):

- the mechanics of using temper well: gate hooks, the typecheck tool, the
  emitted harness reference, the SDK counsel rule;
- opt-in patterns as kinds to extend: a standard over `rule`, a spec layout
  with invariants, a procedure skill with topics.

It never carries taste. There is no memory content and no conventions, the
patterns' vocabulary is the consumer's, and telemetry is opt-in. A fresh
represented install composes it (`adoption.md`, "Install"), and it upgrades
with the package. The shipped example composes the starter with a worked
customization, so the structure stays readable, and the SDK's counsel and
the reference teach building over it.

## Rejected

- **A frozen scaffold.** It is visible, but it never upgrades.
- **A tracked scaffold with three-way merge.** It is right in principle and
  too heavy for the gain.

## Consequences

`examples/base-harness` folds into "starter plus customization". This
harness composes the starter too, which closes the self-hosting gap: it
gets the reference skill and the same gates a stranger gets.
