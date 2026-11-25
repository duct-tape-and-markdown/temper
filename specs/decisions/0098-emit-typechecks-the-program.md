# 0098 — emit typechecks the program

- **Date:** 2026-11-25 · **Status:** accepted

## Context

The `where-the-harness-program-typechecks` question. Emit runs the
harness program under Node's type stripping, and nothing runs `tsc`. So a
wrong field, a removed constant or a field typed `never` passes both emit
and check. One SDK adopter carried a standing type error (`registration:
[]` on an embedded kind) without seeing it, and the remedy-naming type
errors the SDK now ships (544fcb9c) reach no one. Intent invariant 8 names
the compiler's own errors as a delivery channel, which holds only if a
compiler runs. `check` is ruled out, because checking never needs Node
(`authoring.md`, "The SDK"). A scaffolded script is exposure, not
delivery. That leaves emit, the one verb that runs the program. Ruled by
John 2026-11-25, who accepted the break in the next release.

## Decision

**Emit typechecks the program before running it, and refuses on a type
error** (`pipeline.md`, "Emit", Refusing). It uses the program's own
resolved TypeScript and names the compiler's diagnostics. A program with
no resolvable TypeScript refuses, naming the install that fixes it. The
toolchain is held by its version and packages by their lockfile
(`authoring.md`), so the compiler is the program's own and never pinned
in the lock.

## Rejected

- **A warning first, a refusal a release later.** A type error is a shape
  the author spelled wrong, and refusing is how emit already treats such
  shapes. The break lands with the version bump.
- **`typescript` as a runtime dependency of the SDK.** That makes a second
  compiler beside the program's, which can disagree with its editor.
- **Report and continue.** It runs a program the compiler has called
  wrong, so its lock is built from a shape that was never checked.

## Consequences

`install` scaffolds `typescript` and a `tsconfig` into `.temper/`. The
migration note names the break and the fix: run `tsc` and correct what it
reports. Emit pays one typecheck per run, the price of delivery.
