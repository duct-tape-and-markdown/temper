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

**The program declares its typecheck, and emit runs it before the program,
refusing on a type error** (`pipeline.md`, "Emit", Refusing). The
declaration is the consumer's own toolchain: the compiler command and the
project file, never resolved by guess, as a gate hook states its binary
(0082). The project file is a declared input (0064), so a change to it
marks the lock stale. A program that declares no typecheck refuses,
naming the declaration. Amended the same day, before derivation, on
John's point that the compiler is the consumer's package to configure and
register.

## Rejected

- **A warning first, a refusal a release later.** A type error is a shape
  the author spelled wrong, and refusing is how emit already treats such
  shapes. The break lands with the version bump.
- **`typescript` as a runtime dependency of the SDK.** That makes a second
  compiler beside the program's, which can disagree with its editor.
- **Resolving the program's TypeScript implicitly.** That is temper guessing
  at the consumer's toolchain, the PATH guess 0082 retired for the gate
  binary.
- **Report and continue.** It runs a program the compiler has called
  wrong, so its lock is built from a shape that was never checked.

## Consequences

`install` scaffolds the declaration, a `tsconfig` and `typescript` as a
dependency of `.temper/`, so a new harness gets one line it owns. The
migration note names the break and the fix: declare the typecheck, run it,
and correct what it reports. Emit pays one typecheck per run, the price of
delivery.
