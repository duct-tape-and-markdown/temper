---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
name: "temper"
description: "Use when operating the temper gate on this harness — running `temper emit` or `temper check`, reading a temper finding, deciding whether to fix a member or challenge its contract — or before adding to or restructuring the harness."
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

Each finding names its clause (the `code`, e.g. `skill.forbidden_keys`), the artifact
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

- When you add or change a `agent` member, read [kind-agent.md](kind-agent.md).
- When you add or change a `command` member, read [kind-command.md](kind-command.md).
- When you add or change a `handler` member, read [kind-handler.md](kind-handler.md).
- When you add or change a `hook` member, read [kind-hook.md](kind-hook.md).
- When you add or change a `installed-plugin` member, read [kind-installed-plugin.md](kind-installed-plugin.md).
- When you add or change a `known-marketplace` member, read [kind-known-marketplace.md](kind-known-marketplace.md).
- When you add or change a `marketplace` member, read [kind-marketplace.md](kind-marketplace.md).
- When you add or change a `mcp` member, read [kind-mcp.md](kind-mcp.md).
- When you add or change a `mcp-server` member, read [kind-mcp-server.md](kind-mcp-server.md).
- When you add or change a `memory` member, read [kind-memory.md](kind-memory.md).
- When you add or change a `output-style` member, read [kind-output-style.md](kind-output-style.md).
- When you add or change a `plugin-manifest` member, read [kind-plugin-manifest.md](kind-plugin-manifest.md).
- When you add or change a `requirement` member, read [kind-requirement.md](kind-requirement.md).
- When you add or change a `rule` member, read [kind-rule.md](kind-rule.md).
- When you add or change a `settings` member, read [kind-settings.md](kind-settings.md).
- When you add or change a `skill` member, read [kind-skill.md](kind-skill.md).
- When you add or change a `supporting-doc` member, read [kind-supporting-doc.md](kind-supporting-doc.md).
- When you add or change a `tool` member, read [kind-tool.md](kind-tool.md).
