/**
 * The playbook — one area as a path rule, a routing skill and its topic files: each axis
 * the shape offers, what it derives, its replaceable wording, and the axes it refuses.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { harness, span, text } from "../src/index.js";
import { emit } from "../src/emit.js";
import { agent } from "../src/claude-code.js";
import { playbook } from "../src/patterns.js";
import type { PlaybookInit } from "../src/patterns.js";
import type { Text } from "../src/index.js";

const base: PlaybookInit = {
  name: "rust-code",
  description: "Procedures for changing the Rust code. Use when adding a module or writing a test.",
  facts: text`# Rust code\n\nRun cargo fmt before committing.`,
  topics: [
    { name: "adding-a-module", when: "adding a module", prose: span("# Adding a module\n") },
    { name: "writing-a-test", when: "writing a test", prose: span("# Writing a test\n") },
  ],
};

type Result = ReturnType<typeof emit>;

function projected(result: Result, kind: string, name: string): Result["members"][number] {
  const member = result.members.find((m) => m.kind === kind && m.name === name);
  assert.ok(member, `the playbook projects ${kind} \`${name}\``);
  return member;
}

function field(member: Result["members"][number], key: string): unknown {
  return member.fields.find(([k]) => k === key)?.[1];
}

function emitted(init: PlaybookInit): Result {
  return emit(harness({ members: [...playbook(init).members] }));
}

test("a path-triggered playbook scopes its rule to the area and keeps its skill listed", () => {
  const result = emitted({ ...base, trigger: { paths: ["src/**/*.rs"] } });
  assert.deepEqual(field(projected(result, "rule", "rust-code"), "paths"), ["src/**/*.rs"]);
  const skill = projected(result, "skill", "rust-code");
  assert.equal(field(skill, "paths"), undefined);
  assert.equal(field(skill, "description"), base.description);
});

test("an on-touch listing gates the skill's listing on the trigger's paths", () => {
  const result = emitted({ ...base, trigger: { paths: ["src/**/*.rs"], listing: "on-touch" } });
  assert.deepEqual(field(projected(result, "skill", "rust-code"), "paths"), ["src/**/*.rs"]);
  assert.deepEqual(field(projected(result, "rule", "rust-code"), "paths"), ["src/**/*.rs"]);
});

test("a playbook with no trigger loads its rule at launch", () => {
  const result = emitted(base);
  assert.equal(field(projected(result, "rule", "rust-code"), "paths"), undefined);
});

test("a nested playbook places its rule and skill under the subdirectory's own .claude folders", () => {
  const result = emitted({ ...base, trigger: { nested: "apps/web" } });
  const rule = projected(result, "rule", "apps:web:rust-code");
  assert.equal(rule.location, "apps/web/.claude/rules/rust-code.md");
  assert.equal(field(rule, "paths"), undefined);
  const skill = projected(result, "skill", "apps:web:rust-code");
  assert.equal(skill.location, "apps/web/.claude/skills/rust-code/SKILL.md");
  assert.deepEqual(skill.fields[0], ["name", "rust-code"]);
  assert.match(skill.body, /read \[adding-a-module\]\(adding-a-module\.md\)/);
  assert.match(rule.body, /use the `\/apps\/web:rust-code` skill/);
});

test("a nested user-only playbook asks the user to run the skill by its nested invocation", () => {
  const result = emitted({ ...base, trigger: { nested: "apps/web" }, invocation: "user" });
  assert.match(projected(result, "rule", "apps:web:rust-code").body, /ask the user to run `\/apps\/web:rust-code`/);
});

test("a nested playbook refuses a directory that is no plain relative path", () => {
  for (const nested of ["", "/apps", "apps/../web", "apps/.claude", "apps/*"]) {
    assert.throws(() => playbook({ ...base, trigger: { nested } }), /nested directory .* is no plain relative path/);
  }
});

test("the rule holds the area's facts, then points at the skill by a typed mention", () => {
  const book = playbook(base);
  const result = emit(harness({ members: [...book.members] }));
  const body = projected(result, "rule", "rust-code").body;
  assert.equal(
    body,
    "# Rust code\n\nRun cargo fmt before committing.\n\n" +
      "For a procedure in this area, use the `rust-code` skill: it routes to one topic per procedure.\n",
  );
  const ruleBody = book.rule.prose;
  assert.ok(ruleBody?.kind === "blocks");
  const pointer = ruleBody.values[1] as Text;
  assert.deepEqual(
    pointer.mentions.map((m) => m.target.address),
    ["skill:rust-code"],
  );
});

test("the router links each topic by its path, one topic file per procedure beside SKILL.md", () => {
  const result = emitted(base);
  const body = projected(result, "skill", "rust-code").body;
  assert.match(body, /- When adding a module: read \[adding-a-module\]\(adding-a-module\.md\)\./);
  assert.match(body, /- When writing a test: read \[writing-a-test\]\(writing-a-test\.md\)\./);
  assert.equal(projected(result, "supporting-doc", "adding-a-module").body.trim(), "# Adding a module");
  projected(result, "supporting-doc", "writing-a-test");
});

test("a user-only playbook asks the user to run it, and its skill keeps its own description", () => {
  const result = emitted({ ...base, trigger: { paths: ["src/**"] }, invocation: "user" });
  const skill = projected(result, "skill", "rust-code");
  assert.equal(field(skill, "disable-model-invocation"), true);
  assert.equal(field(skill, "description"), base.description);
  assert.match(projected(result, "rule", "rust-code").body, /ask the user to run `\/rust-code`/);
});

test("a model-only playbook hides its skill from the menu", () => {
  const result = emitted({ ...base, invocation: "model" });
  const skill = projected(result, "skill", "rust-code");
  assert.equal(field(skill, "user-invocable"), false);
  assert.equal(field(skill, "disable-model-invocation"), undefined);
});

test("the default invocation leaves both channels live", () => {
  const skill = projected(emitted(base), "skill", "rust-code");
  assert.equal(field(skill, "user-invocable"), undefined);
  assert.equal(field(skill, "disable-model-invocation"), undefined);
});

test("the router and pointer wording are renderers the consumer replaces", () => {
  const result = emitted({
    ...base,
    render: {
      router: (view) => text`# Routes\n\nFirst: ${view.topics[0].link}.\n`,
      pointer: (view) => text`See ${view.skill}.`,
    },
  });
  assert.match(projected(result, "skill", "rust-code").body, /^# Routes\n\nFirst: adding-a-module\.md\.\n/);
  assert.match(projected(result, "rule", "rust-code").body, /See rust-code\./);
});

test("replacing one renderer keeps the other's default", () => {
  const result = emitted({ ...base, render: { pointer: () => text`Use the skill.` } });
  assert.match(projected(result, "skill", "rust-code").body, /Open the one topic the step at hand needs/);
  assert.match(projected(result, "rule", "rust-code").body, /Use the skill\./);
});

test("setting execution refuses, naming the axis and where to request it", () => {
  assert.throws(
    () => playbook({ ...base, execution: { fork: "Explore" } }),
    /execution is declared but not yet offered.*github\.com\/duct-tape-and-markdown\/temper\/issues/,
  );
  assert.throws(() => playbook({ ...base, execution: "inline" }), /execution/);
});

test("setting preloading refuses, naming the axis and where to request it", () => {
  const reviewer = agent({ name: "reviewer", description: "Reviews changes." });
  assert.throws(
    () => playbook({ ...base, preloadedBy: [reviewer] }),
    /preloading is declared but not yet offered.*github\.com\/duct-tape-and-markdown\/temper\/issues/,
  );
});

test("a playbook refuses a blank description, no topics, and a duplicated topic", () => {
  assert.throws(() => playbook({ ...base, description: " " }), /description/);
  assert.throws(() => playbook({ ...base, topics: [] }), /at least one topic/);
  const [first] = base.topics;
  assert.throws(() => playbook({ ...base, topics: [first, first] }), /two topics are named `adding-a-module`/);
});

test("a playbook's rule and skill each fill the requirements named for them", () => {
  const book = playbook({ ...base, satisfies: { rule: ["standards"], skill: ["operations"] } });
  assert.deepEqual(book.rule.satisfies, ["standards"]);
  assert.deepEqual(book.skill.satisfies, ["operations"]);
  for (const topic of book.topics) assert.deepEqual(topic.satisfies, []);
  const bare = playbook(base);
  assert.deepEqual([bare.rule.satisfies, bare.skill.satisfies], [[], []]);
});
