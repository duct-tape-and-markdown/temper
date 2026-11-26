/**
 * The harness reference — temper's operate skill declared as a member and rendered at
 * emit from the harness that lists it.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { clause, dial, emit, harness, kind, reference, required, requirement, review, text } from "../src/index.js";
import { rule, settingsLocal, skill } from "../src/claude-code.js";

function declared(...extra: Parameters<typeof harness>[0]["members"]) {
  return harness({
    members: [
      rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust` }),
      ...extra,
    ],
    require: {
      "release-procedure": requirement({ prose: "How a release is cut." }),
      conventions: requirement({ prose: "The code conventions.", required: false }),
    },
  });
}

function referenceOf(result: ReturnType<typeof emit>) {
  const member = result.members.find((m) => m.kind === "skill" && m.name === "temper");
  assert.ok(member, "the declared reference projects as the `temper` skill");
  return member;
}

test("declared reference emits its SKILL.md with the mechanics and one index line per kind and per domain", () => {
  const result = emit(declared(reference()));
  const skillMember = referenceOf(result);
  const body = skillMember.body;
  assert.ok(body.startsWith("# Operating the temper gate\n"), body);
  assert.match(body, /## Fix the artifact, or challenge the contract/);
  assert.match(body, /`temper check \.`/);

  const lines = body.split("\n");
  const kindLines = lines.filter((line) => line.includes("](kind-"));
  assert.deepEqual(kindLines, [
    "- When you add or change a `rule` member, read [kind-rule.md](kind-rule.md).",
    "- When you add or change a `skill` member, read [kind-skill.md](kind-skill.md).",
  ]);
  const domainLines = lines.filter((line) => line.includes("](domain-"));
  assert.deepEqual(domainLines, [
    "- When you fill, change, or move the `conventions` domain, read [domain-conventions.md](domain-conventions.md).",
    "- When you fill, change, or move the `release-procedure` domain, read [domain-release-procedure.md](domain-release-procedure.md).",
  ]);
  // The body is the method and its index — no member is listed.
  assert.doesNotMatch(body, /\brust\b/);
});

test("declared reference description names the contracted domains", () => {
  const description = referenceOf(emit(declared(reference()))).fields.find(([key]) => key === "description")?.[1];
  assert.equal(typeof description, "string");
  assert.match(description as string, /^Use when operating the temper gate/);
  assert.match(description as string, /Its contracted domains: `conventions`, `release-procedure`\.$/);
});

test("declared reference may declare satisfies and so own its domain", () => {
  const program = harness({
    members: [reference({ satisfies: ["gate-operation"] })],
    require: { "gate-operation": requirement({ prose: "How to operate the gate." }) },
  });
  const result = emit(program);
  assert.deepEqual(
    result.declarations.satisfies.map((row) => [row.member, row.requirement]),
    [["skill:temper", "gate-operation"]],
  );
  assert.match(referenceOf(result).body, /\[domain-gate-operation\.md\]/);
});

test("declared reference renders only committed declarations", () => {
  const program = harness({
    members: [
      reference(),
      settingsLocal({ name: "settings.local", requires: { "machine-only": requirement({ prose: "One machine's slot." }) } }),
      dial({ name: "dial" }),
    ],
  });
  const result = referenceOf(emit(program));
  assert.doesNotMatch(result.body, /settings-local|kind-dial|machine-only/);
  const description = result.fields.find(([key]) => key === "description")?.[1] as string;
  assert.doesNotMatch(description, /contracted domains/);
});

test("a harness without the reference projects no temper skill", () => {
  const result = emit(declared(skill({ name: "other", description: "Use when testing.", prose: text`# Other` })));
  assert.equal(
    result.members.find((m) => m.name === "temper"),
    undefined,
  );
});

function domainTopicOf(result: ReturnType<typeof emit>, name: string) {
  const topic = result.members.find((m) => m.kind === "supporting-doc" && m.name === `domain-${name}`);
  assert.ok(topic, `the reference carries a domain-${name}.md topic`);
  assert.equal(topic.host, "skill:temper");
  return topic.body;
}

test("declared reference renders one domain topic per requirement: prose, verifier, owners", () => {
  const program = harness({
    members: [
      reference({ satisfies: ["gate-operation"] }),
      rule({ name: "release", prose: text`# Release`, satisfies: ["release-procedure"] }),
    ],
    require: {
      "gate-operation": requirement({ prose: "How to operate the gate." }),
      "release-procedure": requirement({
        prose: "How a release is cut.",
        verifier: review("the release captain reads the cut against the recipe."),
      }),
    },
  });
  const result = emit(program);

  assert.equal(
    domainTopicOf(result, "release-procedure"),
    [
      "# The `release-procedure` domain",
      "",
      "How a release is cut.",
      "",
      "## Verifier",
      "",
      "It is held by review: the release captain reads the cut against the recipe.",
      "",
      "## Owners",
      "",
      "- `rule:release`",
      "",
    ].join("\n"),
  );
  const unheld = domainTopicOf(result, "gate-operation");
  assert.match(unheld, /^How to operate the gate\.$/m);
  assert.match(unheld, /It is unheld — no verifier is declared/);
  assert.doesNotMatch(unheld, /held by review/);
  assert.match(unheld, /^- `skill:temper`$/m);
});

test("a domain topic names that no member fills an unfilled domain", () => {
  const result = emit(declared(reference()));
  assert.match(domainTopicOf(result, "conventions"), /## Owners\n\nNo member fills it yet\.\n$/);
});

test("the reference's domain topics are only its committed domains", () => {
  const program = harness({
    members: [
      reference(),
      settingsLocal({ name: "settings.local", requires: { "machine-only": requirement({ prose: "One machine's slot." }) } }),
    ],
  });
  const topics = emit(program).members.filter((m) => m.kind === "supporting-doc" && m.name.startsWith("domain-"));
  assert.deepEqual(topics, []);
});

function kindTopicOf(result: ReturnType<typeof emit>, name: string) {
  const topic = result.members.find((m) => m.kind === "supporting-doc" && m.name === `kind-${name}`);
  assert.ok(topic, `the reference carries a kind-${name}.md topic`);
  assert.equal(topic.host, "skill:temper");
  return topic.body;
}

test("declared reference renders one kind topic per kind: counsel, locus, fields, edges with intent, clauses, domains — and no member", () => {
  const standard = kind<{ readonly title?: string; readonly owner?: string }>({
    name: "standard",
    locus: { kind: "at", root: "standards", glob: "*.md" },
    format: "yaml-frontmatter",
    unitShape: "file",
    registration: [{ via: "always" }],
    edgeFields: [{ field: "owner", to: [skill] }],
    guidance: "A standard states one rule an agent can follow.",
    fieldGuidance: {
      title: "Name the rule, not the topic.",
      owner: "The skill that keeps this standard current.",
    },
  });
  const program = harness({
    members: [
      reference(),
      skill({ name: "standards", description: "Use when touching standards.", prose: text`# Standards` }),
      standard({ name: "naming-standard", owner: "standards", satisfies: ["code-standards"] }),
    ],
    expect: [
      {
        kind: standard,
        clauses: [clause(required("title"), { severity: "required", guidance: "Every standard is titled." })],
      },
    ],
    require: {
      "code-standards": requirement({ prose: "The code standards are written down.", kind: standard }),
    },
  });
  const result = emit(program);
  const body = kindTopicOf(result, "standard");

  assert.equal(
    body,
    [
      "# The `standard` kind",
      "",
      "A standard states one rule an agent can follow.",
      "",
      "## Locus",
      "",
      "Each member is its own document, at:",
      "",
      "- `standards/*.md`",
      "",
      "## Fields",
      "",
      "- `title`: Name the rule, not the topic.",
      "",
      "## Edges",
      "",
      "### Out",
      "",
      "- `owner` → `skill`: The skill that keeps this standard current.",
      "",
      "## Clauses",
      "",
      "- `required` on `title` (required): Every standard is titled.",
      "",
      "## Domains",
      "",
      "- `code-standards` (declared over this kind; a member of this kind fills it) — read [domain-code-standards.md](domain-code-standards.md).",
      "",
    ].join("\n"),
  );
  // The edge's intent reads at its other end too.
  assert.match(
    kindTopicOf(result, "skill"),
    /### In\n\n- `standard`\.`owner` → here: The skill that keeps this standard current\./,
  );
  // A topic grows with declarations, never with documents: no member is named.
  for (const topic of result.members.filter((m) => m.kind === "supporting-doc" && m.name.startsWith("kind-"))) {
    assert.doesNotMatch(topic.body, /naming-standard|`standards`|skill:temper/);
  }
});

test("a kind topic states an unbound kind and an uncounselled edge plainly", () => {
  const page = kind<object>({
    name: "page",
    locus: { kind: "at", root: "pages", glob: "*.md" },
    format: "yaml-frontmatter",
    unitShape: "file",
    registration: [{ via: "always" }],
  });
  const linked = kind<{ readonly next?: string }>({
    name: "linked",
    locus: { kind: "at", root: "docs", glob: "*.md" },
    format: "yaml-frontmatter",
    unitShape: "file",
    registration: [{ via: "always" }],
    edgeFields: [{ field: "next", to: [page] }],
  });
  const result = emit(harness({ members: [reference(), page({ name: "p" }), linked({ name: "a" })] }));
  const body = kindTopicOf(result, "linked");
  assert.match(body, /^The kind declares no counsel\.$/m);
  assert.match(body, /### Out\n\n- `next` → `page`: no intent declared\./);
  assert.match(kindTopicOf(result, "page"), /### In\n\n- `linked`\.`next` → here: no intent declared\./);
  assert.match(body, /## Clauses\n\nThe program binds no clauses over it/);
  assert.doesNotMatch(body, /## Domains/);
});

test("the reference's kind topics are only its committed kinds", () => {
  const program = harness({ members: [reference(), settingsLocal({ name: "settings.local" }), dial({ name: "dial" })] });
  const names = emit(program)
    .members.filter((m) => m.kind === "supporting-doc")
    .map((m) => m.name);
  assert.deepEqual(names, ["kind-skill"]);
});
