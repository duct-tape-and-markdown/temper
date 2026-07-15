import { skill } from "@dtmd/temper/claude-code";
import { span } from "../kinds.ts";

/**
 * The Governance procedure: how this harness grows. The domain skeleton is
 * complete on day one; maturing is only ever adding members to slots that
 * already exist, and this skill is the filing protocol for each addition.
 */
export const skill_growHarness = skill({
  name: "grow-harness",
  description:
    "How to add knowledge to this harness: which kind owns a new fact, convention, procedure, or enforcement, and the trigger that says it is time. Use when adding or reorganizing a rule, skill, hook, or memory entry.",
  satisfies: ["governance"],
  prose: span(
    `# Growing this harness

The harness is organized domain first, mechanism second. Its five domains —
conduct, orientation, standards, operations, governance — are declared as
requirements in \`.temper/harness.ts\`, and every member names the domain it
fills with a \`satisfies\` entry. Growth is additive: new knowledge joins an
existing domain; it never creates a parallel structure.

## The filing rule

Pick the domain, then the delivery tier, and the kind falls out:

- Needed **every session** (a fact, a always-true convention) — memory, or
  a path-less rule when it would crowd the map.
- Needed **when touching certain files** — a rule with \`paths\`, or a
  skill with \`paths\` when it is a procedure rather than a convention.
- Needed **on demand** (a playbook, reference material) — a skill.
- Must hold **every time, without asking** — a hook; an instruction is a
  request, a hook is a guarantee.

## The triggers

Add a member when its trigger fires, not before (the harness's own vendor
documents this adoption order — code.claude.com/docs/en/features-overview,
retrieved 2026-07-15):

- The agent gets a convention or command wrong **twice** — a memory entry
  or rule line (domain: standards, usually).
- The same playbook is pasted a **third** time — a skill (operations).
- Something must happen **every time** — a hook (governance).

## The one rule of authorship

A fact more than one surface states is authored once in
\`.temper/facts.ts\` and interpolated everywhere it is spoken. Never retype
a command or count into a second surface — projections repeat facts,
authorship does not.
`,
  ),
});
