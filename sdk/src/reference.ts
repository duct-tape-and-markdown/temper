/**
 * The harness reference — temper's operate skill as a value a harness declares. Listed in
 * `harness({ members })` it is an ordinary `skill` member named `temper`; at emit its
 * description and body render from the program's own declarations: the product's
 * mechanics, then one index line per kind in play and per contracted domain, and a
 * description naming those domains so every session starts knowing which are taken.
 * Each kind and domain the index names gets its topic flat beside `SKILL.md`, a
 * `supporting-doc`: `kind-<k>.md` carries the kind's counsel, locus, fields, edges in and
 * out with their intent, the clauses bound over it and its domains; `domain-<r>.md` the
 * requirement's prose, what holds it, and its owners. A kind topic names no member, so it
 * grows with declarations, never with documents.
 *
 * It renders only committed declarations: a kind at a `local` locus (the dial among
 * them) and every member of one stay out of the index, since a committed projection
 * cannot carry what one machine alone declared.
 */

import type { Harness } from "./assembly.js";
import type { Clause, Verifier } from "./contract.js";
import type { KindFacts, Member } from "./kind.js";
import { skill, supportingDoc } from "./builtins.js";
import {
  clauseField,
  compareStrings,
  contractedRequirements,
  kindEdges,
  kindLocusGlobs,
  kindsInPlay,
  targetList,
} from "./declarations.js";
import type { ContractedRequirement, EdgeIntent } from "./declarations.js";
import { span } from "./prose.js";

/** The reference's skill name — its member lands at `.claude/skills/temper/SKILL.md`. */
export const REFERENCE_NAME = "temper";

/** What the reference may declare of its own: the domains it fills. */
export interface ReferenceInit {
  readonly satisfies?: readonly string[];
}

/** The declared references, awaiting their render from the harness that lists them. */
const DECLARED = new WeakSet<Member>();

/**
 * The method an agent operating the gate needs, whatever the harness — run the gate,
 * read a finding, fix the member or challenge the contract. No opinion about what a good
 * harness is: that lives in the contracts.
 */
const MECHANICS = `# Operating the temper gate

\`temper\` is one gate over a Claude Code harness, placed wherever the harness is
authored, changed, or used. This skill is how to *operate* that gate. It carries no
opinion about what a good harness is — that lives in the SDK program's contracts,
which you bind, extend, or fork.

## Run the gate

- \`temper install\` — the one on-ramp: a discovery report, then one question
  (represent this project as a temper program? — every answer flag-spelled, e.g.
  \`--yes\`, no invisible state). Yes lifts the harness into an SDK program, runs the
  first \`emit\`, and places the session-start hook, guard hook, schema modeline, and
  managed-by notes; no wires the session-start hook alone. Re-running converges: an
  unchanged harness is a no-op.
- \`temper emit\` — compile the program into its projections and lock. Change the
  module or prose file that owns a member, then emit; never edit a projection, since a
  direct edit is drift that \`check\` and \`guard\` report.
- \`temper check .\` — the gate, run at the harness root. It validates two greens: every built-in
  contract is itself admissible, and every artifact conforms to the contract for its
  kind. It exits non-zero on a \`required\`-severity violation; add \`--deny-advisories\`
  to also fail on advisory ones.
- \`temper schema --kind <kind>\` — emit the active contract as an editor JSON Schema,
  the same gate shifted to the keystroke.

## Author a custom kind's layout

When a host document interleaves prose and typed members — a spec with its
invariants, a rule with its clauses — declare a **layout** on the kind and author
the document in place. A layout types the body's heading tree in three
primitives: **prose** (verbatim words), a **field section** (a heading whose span
fills a named field, intent among them), and a **member collection** (a heading
whose child headings are each one member of a named kind). A field section the
kind marks as an edge field declares the member's edges, \`satisfies\` among them.

A layout has one face — the reader: the document is the authored home, so \`check\`
reads its members off the file and \`emit\` never regenerates it. Author the prose
and the members directly; declaring the layout is what puts that content under
the gate.

## Read a diagnostic

Each finding names its clause (the \`code\`, e.g. \`skill.forbidden_keys\`), the artifact
it is about, and what failed. A \`required\` finding blocks; an \`advisory\` one is a
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
  program's \`expect\` overrides, compiled into the lock at \`emit\`. Do not silence a
  finding by contorting the artifact around a clause you disagree with — change the
  clause.

Never paper over a gap. If the contract and the artifact disagree and you are unsure
which is wrong, surface it rather than guessing which way to bend.
`;

/** The trigger every reference description opens with, before its domains. */
const TRIGGER =
  "Use when operating the temper gate on this harness — running `temper emit` or `temper check`, " +
  "reading a temper finding, deciding whether to fix a member or challenge its contract — " +
  "or before adding to or restructuring the harness.";

/**
 * Declare the harness reference: a `skill` member named {@link REFERENCE_NAME} whose
 * description and body `emit` renders from the harness listing it. `satisfies` lets it
 * fill a domain — and so list itself as that domain's owner.
 */
export function reference(init: ReferenceInit = {}): Member {
  const member = skill({
    name: REFERENCE_NAME,
    description: TRIGGER,
    prose: span(MECHANICS),
    satisfies: [...(init.satisfies ?? [])],
  });
  DECLARED.add(member);
  return member;
}

/** Whether a kind's documents are committed — every locus but an `at` locus marked `local`. */
function committed(facts: KindFacts): boolean {
  return !(facts.locus.kind === "at" && facts.locus.commitment === "local");
}

/** The program as the reference may render it: its committed members, and the kinds in play over them. */
function committedView(harness: Harness): Harness {
  return {
    ...harness,
    members: harness.members.filter((member) => committed(member.facts)),
    expect: harness.expect.filter((binding) => committed(binding.kind.facts)),
    admit: harness.admit.filter((admission) => committed(admission.host.facts)),
  };
}

/**
 * What holds a requirement's behavior, as its domain topic states it — a declared
 * verifier, or that it is unheld. The two read distinctly: a review names the reader
 * whose judgment holds the domain, while unheld names that nothing judges it.
 */
function heldBy(verifier: Verifier | undefined): string {
  switch (verifier?.species) {
    case undefined:
      return "It is unheld — no verifier is declared, so nothing judges its behavior.";
    case "review":
      return `It is held by review: ${verifier.prose}`;
    case "script":
      return `It is held by script: \`${verifier.path}\`.`;
    case "telemetry":
      return `It is held by telemetry over ${verifier.events.map((event) => `\`${event}\``).join(", ")}.`;
  }
}

/** The body of a domain's topic — its prose, what holds it, and the members that own it. */
function domainTopic(requirement: ContractedRequirement): string {
  const owners =
    requirement.satisfiers.length === 0
      ? "No member fills it yet."
      : requirement.satisfiers.map((owner) => `- \`${owner.address}\``).join("\n");
  return [
    `# The \`${requirement.name}\` domain`,
    requirement.prose,
    `## Verifier\n\n${heldBy(requirement.verifier)}`,
    `## Owners\n\n${owners}`,
  ].join("\n\n");
}

/** The topic file name a domain's index line links to — without its `.md`, the doc's identity. */
function domainTopicName(name: string): string {
  return `domain-${name}`;
}

/** The topic file name a kind's index line links to — without its `.md`, the doc's identity. */
function kindTopicName(name: string): string {
  return `kind-${name}`;
}

/** A list item whose text may span lines — continuation lines indent under the bullet. */
function item(head: string, text: string | undefined): string {
  const body = text?.trim();
  return body === undefined || body === "" ? `- ${head}` : `- ${head}: ${body.split("\n").join("\n  ")}`;
}

/** Where a kind's members live, as its topic states it — the locus and the globs it occupies. */
function locusSection(facts: KindFacts, globs: readonly string[]): string {
  const listed = globs.map((glob) => `- \`${glob}\``).join("\n");
  switch (facts.locus.kind) {
    case "at":
      return `Each member is its own document, at:\n\n${listed}`;
    case "nested-file":
      return globs.length === 0
        ? "Each member is a file nested in a host's unit, and no host in play templates it."
        : `Each member is a file nested in a host's unit, at:\n\n${listed}`;
    case "embedded":
      return globs.length === 0
        ? "Each member is embedded in a host's document, and no host in play templates or admits it."
        : `Each member is embedded in a host's document, at:\n\n${listed}`;
  }
}

/** The fields a kind's topic lists — its leaf set, then any counselled field beyond it, edge fields aside. */
function fieldLines(facts: KindFacts): string[] {
  const edges = new Set((facts.edgeFields ?? []).map((edge) => edge.field));
  const counsel = (facts.fieldGuidance ?? {}) as Record<string, string | undefined>;
  const names = [...new Set([...Object.keys(facts.leaves ?? {}), ...Object.keys(counsel)])];
  return names.filter((name) => !edges.has(name)).map((name) => item(`\`${name}\``, counsel[name]));
}

/** One clause bound over a kind, as its topic states it: the predicate, its field, its severity, its guidance. */
function clauseLine(clause: Clause): string {
  const field = clauseField(clause.predicate);
  const on = field === undefined ? "" : ` on \`${field}\``;
  return item(`\`${clause.predicate.key}\`${on} (${clause.severity})`, clause.guidance);
}

/** The body of a kind's topic — every declaration over the kind, and no member of it. */
function kindTopic(
  facts: KindFacts,
  view: Harness,
  kinds: readonly KindFacts[],
  requirements: readonly ContractedRequirement[],
): string {
  const counsel = facts.guidance?.trim();
  const fields = fieldLines(facts);
  const { outbound, inbound } = kindEdges(facts, kinds);
  const edge = (head: string, intent: EdgeIntent["intent"]) => item(head, intent ?? "no intent declared.");
  const edges = [
    ...(outbound.length === 0
      ? []
      : [`### Out\n\n${outbound.map((e) => edge(`\`${e.field}\` → ${targetList(e.to)}`, e.intent)).join("\n")}`]),
    ...(inbound.length === 0
      ? []
      : [`### In\n\n${inbound.map((e) => edge(`\`${e.from}\`.\`${e.field}\` → here`, e.intent)).join("\n")}`]),
  ];
  const bindings = view.expect.filter((binding) => binding.kind.facts.name === facts.name);
  const clauses = bindings.flatMap((binding) => binding.clauses);
  const contract =
    bindings.length === 0
      ? "The program binds no clauses over it — any default contract the kind ships is what holds it."
      : clauses.length === 0
        ? "The program binds it an empty contract — no clause holds its members."
        : clauses.map(clauseLine).join("\n");
  const domains = requirements.flatMap((requirement) => {
    const declared = requirement.kind === facts.name;
    const filled = requirement.satisfiers.some((satisfier) => satisfier.kind === facts.name);
    if (!declared && !filled) return [];
    const how = [declared ? "declared over this kind" : "", filled ? "a member of this kind fills it" : ""]
      .filter((part) => part !== "")
      .join("; ");
    const topic = domainTopicName(requirement.name);
    return [`- \`${requirement.name}\` (${how}) — read [${topic}.md](${topic}.md).`];
  });
  return [
    `# The \`${facts.name}\` kind`,
    counsel === undefined || counsel === "" ? "The kind declares no counsel." : counsel,
    `## Locus\n\n${locusSection(facts, kindLocusGlobs(facts, view))}`,
    ...(fields.length === 0 ? [] : [`## Fields\n\n${fields.join("\n")}`]),
    ...(edges.length === 0 ? [] : [`## Edges\n\n${edges.join("\n\n")}`]),
    `## Clauses\n\n${contract}`,
    ...(domains.length === 0 ? [] : [`## Domains\n\n${domains.join("\n")}`]),
  ].join("\n\n");
}

/** A topic the reference carries beside `SKILL.md`: its doc name and its body. */
interface Topic {
  readonly name: string;
  readonly body: string;
}

/**
 * The rendered reference over `harness`: its description, its body, and its topics —
 * one per kind in play, then one per contracted domain.
 */
export function renderReference(harness: Harness): {
  readonly description: string;
  readonly body: string;
  readonly topics: readonly Topic[];
} {
  const view = committedView(harness);
  const inPlay = kindsInPlay(view);
  const kinds = inPlay.filter(committed).sort((a, b) => compareStrings(a.name, b.name));
  const requirements = contractedRequirements(view);
  const domains = requirements.map((requirement) => requirement.name);
  const kindLines = kinds.map(
    ({ name }) => `- When you add or change a \`${name}\` member, read [${kindTopicName(name)}.md](${kindTopicName(name)}.md).`,
  );
  const domainLines = domains.map(
    (name) =>
      `- When you fill, change, or move the \`${name}\` domain, read [${domainTopicName(name)}.md](${domainTopicName(name)}.md).`,
  );
  const index = [
    "## This harness",
    "Read the topic file for what you are about to change before changing it: it carries " +
      "what this harness declares there and why.",
    ...(kindLines.length === 0 ? [] : [`### Kinds\n\n${kindLines.join("\n")}`]),
    ...(domainLines.length === 0 ? [] : [`### Domains\n\n${domainLines.join("\n")}`]),
  ].join("\n\n");
  const named = domains.map((name) => `\`${name}\``).join(", ");
  const description =
    domains.length === 0 ? TRIGGER : `${TRIGGER} Its contracted domains: ${named}.`;
  return {
    description,
    body: `${MECHANICS}\n${index}\n`,
    topics: [
      ...kinds.map((facts) => ({
        name: kindTopicName(facts.name),
        body: `${kindTopic(facts, view, inPlay, requirements)}\n`,
      })),
      ...requirements.map((requirement) => ({
        name: domainTopicName(requirement.name),
        body: `${domainTopic(requirement)}\n`,
      })),
    ],
  };
}

/**
 * `harness` with every declared reference rendered from it — the one view emit compiles,
 * so the projection and the declaration rows agree. Each rendered reference brings its
 * kind and domain topics as `supporting-doc` members hosted by it. A harness carrying none comes
 * back as is.
 */
export function withReference(harness: Harness): Harness {
  if (!harness.members.some((member) => DECLARED.has(member))) return harness;
  const { description, body, topics } = renderReference(harness);
  return {
    ...harness,
    members: harness.members.flatMap((member) => {
      if (!DECLARED.has(member)) return [member];
      const rendered: Member = {
        ...member,
        prose: span(body),
        fields: member.fields.map(([key, value]) =>
          key === "description" ? (["description", description] as const) : ([key, value] as const),
        ),
      };
      return [
        rendered,
        ...topics.map((topic) => supportingDoc({ name: topic.name, host: rendered, prose: span(topic.body) })),
      ];
    }),
  };
}
