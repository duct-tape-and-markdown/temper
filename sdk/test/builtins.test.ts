/**
 * The built-in floors: every floor exported from `claude-code.ts` is a
 * well-formed clause array, and every clause carries a non-empty `cite` — the
 * auditability guarantee a maintained floor exists to keep.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import type { Clause } from "../src/index.js";
import {
  command,
  commandFloor,
  memory,
  memoryAgentsMdFloor,
  memoryAnthropicFloor,
  rule,
  ruleFloor,
  skill,
  skillFloor,
} from "../src/claude-code.js";

const FLOORS: ReadonlyArray<readonly Clause[]> = [
  skillFloor,
  commandFloor,
  ruleFloor,
  memoryAnthropicFloor,
  memoryAgentsMdFloor,
];

test("every exported floor is a well-formed clause array", () => {
  for (const floor of FLOORS) {
    assert.ok(Array.isArray(floor));
    for (const entry of floor) {
      assert.ok(entry.predicate && typeof entry.predicate.key === "string" && entry.predicate.key.length > 0);
      assert.ok(entry.severity === "required" || entry.severity === "advisory");
 }
 }
});

test("every floor clause carries a non-empty cite", () => {
  for (const floor of FLOORS) {
    for (const entry of floor) {
      assert.ok(typeof entry.cite === "string" && entry.cite.length > 0, `clause \`${entry.predicate.key}\` is uncited`);
 }
 }
});

test("skillFloor carries the skill kind's decidable clauses, name-first", () => {
  assert.equal(skillFloor.length, 12);
  assert.equal(skillFloor[0].predicate.key, "required");
  assert.equal(skillFloor[0].predicate.field, "name");
  assert.deepEqual(
    skillFloor.map((c) => c.predicate.key),
    [
      "required",
      "min_len",
      "allowed_chars",
      "max_len",
      "deny",
      "name-matches-dir",
      "required",
      "min_len",
      "max_len",
      "max_len",
      "max_lines",
      "forbidden_keys",
    ],
  );
});

test("commandFloor is skillFloor minus the directory-name clause", () => {
  assert.deepEqual(
    commandFloor.map((c) => c.predicate.key),
    skillFloor.map((c) => c.predicate.key).filter((key) => key !== "name-matches-dir"),
  );
  assert.equal(
    commandFloor.some((c) => c.predicate.key === "name-matches-dir"),
    false,
    "a command is a lone file — no parent directory to match",
  );
  // `name` requiredness rides over unchanged: a command still declares no `name`
  // field for identity (file-stem, like `rule`), but the skill schema's own
  // `required`/`min_len`/`allowed_chars`/`max_len`/`deny` clauses over `name` still
  // apply by import.
  assert.equal(commandFloor[0].predicate.key, "required");
  assert.equal(commandFloor[0].predicate.field, "name");
});

test("ruleFloor forbids Cursor keys and budgets body size", () => {
  assert.deepEqual(
    ruleFloor.map((c) => c.predicate.key),
    ["forbidden_keys", "max_lines"],
  );
  assert.deepEqual(ruleFloor[0].predicate.keys, ["description", "globs", "alwaysApply"]);
});

test("memoryAnthropicFloor is a single advisory size budget", () => {
  assert.equal(memoryAnthropicFloor.length, 1);
  assert.equal(memoryAnthropicFloor[0].predicate.key, "max_lines");
  assert.equal(memoryAnthropicFloor[0].severity, "advisory");
});

test("memoryAgentsMdFloor is guidance-only — zero clauses", () => {
  assert.deepEqual(memoryAgentsMdFloor, []);
});

test("the floors ride alongside their kinds through the claude-code subpath", () => {
  assert.equal(typeof skill, "function");
  assert.equal(typeof command, "function");
  assert.equal(typeof rule, "function");
  assert.equal(typeof memory, "function");
});

test("command is a file-shaped unit with no identityField, unlike the directory-shaped skill", () => {
  assert.equal(command.facts.unitShape, "file");
  assert.equal(command.facts.identityField, undefined);
  assert.equal(skill.facts.unitShape, "directory");
  assert.equal(skill.facts.identityField, "name");
});

test("skill/command register on both documented invocation channels; rule/memory carry a singleton set", () => {
  assert.deepEqual(skill.facts.registration, [
    { via: "user-invoked" },
    { via: "description-trigger", field: "description" },
  ]);
  assert.deepEqual(command.facts.registration, [
    { via: "user-invoked" },
    { via: "description-trigger", field: "description" },
  ]);
  assert.deepEqual(rule.facts.registration, [{ via: "paths-match", field: "paths" }]);
  assert.deepEqual(memory.facts.registration, [{ via: "always" }]);
});

test("disable-model-invocation/user-invocable are ordinary declared fields on a skill member", () => {
  const member = skill({
    name: "demo",
    description: "Use when demonstrating a skill's modulating fields.",
    "disable-model-invocation": true,
    "user-invocable": false,
  });
  assert.deepEqual(member.fields, [
    ["name", "demo"],
    ["description", "Use when demonstrating a skill's modulating fields."],
    ["disable-model-invocation", true],
    ["user-invocable", false],
  ]);
});
