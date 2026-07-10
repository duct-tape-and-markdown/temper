# Intent

`temper` is a type system for the documents that program agents. A harness —
the customization layer of a coding agent: skills, rules, memory files, hooks,
permissions, MCP servers, settings — is a codebase, and its primary author is
increasingly the agent itself. temper treats it as one: a typed model the
author composes and compiles, with a gate that checks the result against a
declared contract. The model reaches inside the documents: a kind may declare
its body's layout, so a spec's laws and invariants are typed, addressable,
contracted members — content under the gate, not prose beside it.

This file is the why and the cross-cutting invariants; the model is `model/`
— eight nouns in three files; history lives in `decisions/` and git tags.

## The problem

A malformed harness fails silently at runtime: a skill that never triggers, a
rule that fails to load, a hook command that resolves to nothing, drift nobody
noticed. The author learns of the failure from the agent's behavior, not from
an error message. temper moves that failure to author-time, the way a type
checker does, and earns soundness the same way: it checks the harness against
a contract its author declared — never against the tool's taste.

## The spine rule

**The engine ships no baked judgment.** Every opinion — every check whose
severity anyone could want to dial — is a clause in an overridable default
contract, adopted by import and overridden by composition. The only fixed
checks are well-formedness: the preconditions of checking at all
(`model/contract.md`). The rule binds the engine's own checks first.

## Invariants

1. **Declared, never mined.** An entity or relationship exists because an
   author declared it on a typed surface — a field, an edge, a mention, a
   syntax the target format itself executes. A declaration types a
   position — a heading, a slot, a fence — never a pattern within prose:
   matching is mining, and a check may read authored content but never
   derive model structure from it. The bound runs both ways: declaration
   is opt-in at every grain, and no check may demand declaration density.
2. **Decidable only.** A check enters temper iff it is expressible in the
   closed predicate vocabulary. What cannot be decided is behavior, and
   behavior is delegated to a wired verifier, never guessed. A gate that
   guesses produces false positives, and a gate that cries wolf gets disabled.
3. **Verbatim prose; deterministic compile.** temper never rewords,
   synthesizes, or drops authored words. `emit` is byte-reproducible and
   mechanically double-checked; nondeterminism is detected, never trusted.
4. **Structure, never intent.** temper checks that the harness fills the
   declared contract. It never decides the harness is missing something
   nobody declared. Surface gaps; do not fill them.
5. **Gate, don't lint.** Where blocking is cheap — CI, the author's
   terminal — a failing contract hard-fails. At session start, where blocking
   a live session would be hostile, the verdict is surfaced for approval,
   never silently passed. Enforcement mode is author-declared per placement,
   and a shipped coverage clause enters advisory — escalation is the
   corpus's declared act, never the default's.
6. **Loud or nothing.** A failure temper can detect is an error message at
   author-time — no path silently degrades, deletes, reconciles, or emits
   over an unresolved input. The per-surface refusal clauses are instances
   of this rule; a surface without one is a gap, not a license.
7. **Read or written, never both.** Every governed path is exactly one of: a
   source temper reads, or a projection temper writes. No projection is read
   back for meaning; no source is regenerated; no file is part-authored,
   part-emitted.

## Positioning

The product is the SDK — the typed surface the harness is composed from — and
the gate that surface earns: because the model is declared, malformed config
is caught at author-time, not discovered in the agent's output. Agents are
demonstrably poor at self-authoring harness artifacts unprompted, so the gate
catches the structural failures they most commonly commit, and each clause's
guidance channel delivers best practice at the moment of failure — to the
author who needs it most and retains it least. The human sets the contract;
the agent authors under it; temper holds the line between them.

`rulesync` makes a harness portable; marketplaces distribute artifacts;
temper makes it correct — downstream of both, checking what you installed.

## The honest bound

"Good harness" is not provable, and temper never pretends it is. What is
provable is conformance to a declared contract, so that is all temper
asserts. The undecidable remainder — does this skill trigger well, does this
tool work — is delegated: a requirement's prose carries the intent, its
verifier edge names the test; execution (CI, evals) judges it, never temper.

## Self-hosting

temper is built by an agentic pipeline, and it gates the harness that builds
it. The bound is two greens on temper's own harness: conformance to the
contract its assembly attaches, and that contract's admissibility. The plugin
a stranger installs is the one that gates this repo; no separate finish line.

## The corpus

Evergreen with a stable center: continuously reconciled against code, the
kernel nouns (`model/`) its stable core — a kernel change is a deliberate
ceremony recorded in `decisions/` and tagged, never an incidental edit.
Structure, budgets, ceremony: `process/spec-system.md`.
