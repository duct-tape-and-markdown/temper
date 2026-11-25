/**
 * The harness reference — temper's operate skill declared as a member and rendered at
 * emit from the harness that lists it.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { dial, emit, harness, reference, requirement, text } from "../src/index.js";
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
