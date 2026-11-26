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

**The SDK's `starter` subpath ships temper's default harness as shapes the
consumer hydrates** (`distribution.md`, "The SDK"). Amended the same day,
before derivation, on John's refinement: "look at these starter kinds as
more generic, so the shape of the harness skill can be imported, and
hydrated to fit the user's need."

- Each export is a typed constructor. temper supplies the structure,
  wiring, edges, rendering and default contract. The consumer supplies
  names, prose, paths, and its own fields through a type parameter.
- A required parameter is where taste lives and has no default. An
  optional one is mechanics, with a default.
- The shapes are the gates, the typecheck, the harness reference (its
  description hydratable, its topics derived) and opt-in patterns whose
  only fields are temper's concepts: a standard and a spec layout holding
  invariants with a statement and a holder, and a procedure skill with
  topics.
- `starter()` composes them all with defaults, and a fresh install calls
  it (`adoption.md`). Customization is typed options, with `extend()` the
  escape hatch.

The shape split also answers versioning: an upgrade improves the shapes,
and the content stays the consumer's as arguments in its own program.

## Rejected

- **A frozen scaffold.** It is visible, but it never upgrades.
- **A tracked scaffold with three-way merge.** It is right in principle and
  too heavy for the gain.
- **Fixed members with an `omit` list.** They collide under one kind per
  name and need a second override mechanism.

## Consequences

`examples/base-harness` folds into "starter plus customization". This
harness composes the starter too, which closes the self-hosting gap: it
gets the reference skill and the same gates a stranger gets.
