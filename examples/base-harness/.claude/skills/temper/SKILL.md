---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
name: "temper"
description: "How to operate the temper gate over this repository's harness and documentation corpus. Use when a temper finding appears, before editing anything under .claude/ or docs/, or when adding a member to .temper/. Its contracted domains: `conduct`, `documented-spine`, `governance`, `operations`, `orientation`, `standards`."
---
# Operating the temper gate

`temper` is one gate over a Claude Code harness, placed wherever the harness is
authored, changed, or used. This skill is how to *operate* that gate. It carries no
opinion about what a good harness is — that lives in the SDK program's contracts,
which you bind, extend, or fork.

## Run the gate

- `temper install` — the one on-ramp: a discovery report, then one question
  (represent this project as a temper program? — every answer flag-spelled, e.g.
  `--yes`, no invisible state). Yes lifts the harness into an SDK program, runs the
  first `emit`, and places the session-start hook, guard hook, schema modeline, and
  managed-by notes; no wires the session-start hook alone. Re-running converges: an
  unchanged harness is a no-op.
- `temper emit` — compile the program into its projections and lock. Change the
  module or prose file that owns a member, then emit; never edit a projection, since a
  direct edit is drift that `check` and `guard` report.
- `temper check .` — the gate, run at the harness root. It validates two greens: every built-in
  contract is itself admissible, and every artifact conforms to the contract for its
  kind. It exits non-zero on a `required`-severity violation; add `--deny-advisories`
  to also fail on advisory ones.
- `temper schema --kind <kind>` — emit the active contract as an editor JSON Schema,
  the same gate shifted to the keystroke.

## Author a custom kind's layout

When a host document interleaves prose and typed members — a spec with its
invariants, a rule with its clauses — declare a **layout** on the kind and author
the document in place. A layout types the body's heading tree in three
primitives: **prose** (verbatim words), a **field section** (a heading whose span
fills a named field, intent among them), and a **member collection** (a heading
whose child headings are each one member of a named kind). A field section the
kind marks as an edge field declares the member's edges, `satisfies` among them.

A layout has one face — the reader: the document is the authored home, so `check`
reads its members off the file and `emit` never regenerates it. Author the prose
and the members directly; declaring the layout is what puts that content under
the gate.

## Read a diagnostic

Each finding names its clause (the `code`, e.g. `skill.forbidden-keys`), the artifact
it is about, and what failed. A `required` finding blocks; an `advisory` one is a
recommendation. The clause id is the contract clause that fired — read it against the
contract for that kind to see exactly what was checked.

## Fix the artifact, or challenge the contract

A finding is a true positive by construction — the clause is decidable, so it never
guesses. That leaves two honest responses, never a third:

- **Fix the artifact** when the finding is right — the artifact really does violate a
  contract you stand behind.
- **Challenge the contract** when the clause is wrong for this harness — too strict,
  mis-scoped, or checking something you do not want gated. The contract is data: tune
  the clause's severity, change its parameters, or drop it — through the SDK
  program's `expect` overrides, compiled into the lock at `emit`. Do not silence a
  finding by contorting the artifact around a clause you disagree with — change the
  clause.

Never paper over a gap. If the contract and the artifact disagree and you are unsure
which is wrong, surface it rather than guessing which way to bend.

## This harness

Read the topic file for what you are about to change before changing it: it carries what this harness declares there and why.

### Kinds

- When you add or change a `alternative` member, read [kind-alternative.md](kind-alternative.md).
- When you add or change a `decision` member, read [kind-decision.md](kind-decision.md).
- When you add or change a `flow` member, read [kind-flow.md](kind-flow.md).
- When you add or change a `glossary` member, read [kind-glossary.md](kind-glossary.md).
- When you add or change a `handler` member, read [kind-handler.md](kind-handler.md).
- When you add or change a `hook` member, read [kind-hook.md](kind-hook.md).
- When you add or change a `invariant` member, read [kind-invariant.md](kind-invariant.md).
- When you add or change a `memory` member, read [kind-memory.md](kind-memory.md).
- When you add or change a `rule` member, read [kind-rule.md](kind-rule.md).
- When you add or change a `settings` member, read [kind-settings.md](kind-settings.md).
- When you add or change a `skill` member, read [kind-skill.md](kind-skill.md).
- When you add or change a `step` member, read [kind-step.md](kind-step.md).
- When you add or change a `superseded-decision` member, read [kind-superseded-decision.md](kind-superseded-decision.md).
- When you add or change a `system` member, read [kind-system.md](kind-system.md).
- When you add or change a `tool` member, read [kind-tool.md](kind-tool.md).

### Domains

- When you fill, change, or move the `conduct` domain, read [domain-conduct.md](domain-conduct.md).
- When you fill, change, or move the `documented-spine` domain, read [domain-documented-spine.md](domain-documented-spine.md).
- When you fill, change, or move the `governance` domain, read [domain-governance.md](domain-governance.md).
- When you fill, change, or move the `operations` domain, read [domain-operations.md](domain-operations.md).
- When you fill, change, or move the `orientation` domain, read [domain-orientation.md](domain-orientation.md).
- When you fill, change, or move the `standards` domain, read [domain-standards.md](domain-standards.md).
