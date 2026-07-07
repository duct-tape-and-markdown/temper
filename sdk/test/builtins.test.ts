/**
 * The built-in floors (`FIRST-PARTY-MODULE-COMPLETE`, `specs/architecture/10-contracts.md`,
 * "A shared clause set — a floor"): every floor exported from `claude-code.ts` is a
 * well-formed clause array, and every clause carries a non-empty `cite` — the
 * auditability guarantee a maintained floor exists to keep.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import type { Clause } from "../src/index.js";
import {
  memory,
  memoryAgentsMdFloor,
  memoryAnthropicFloor,
  rule,
  ruleFloor,
  skill,
  skillFloor,
} from "../src/claude-code.js";

const FLOORS: ReadonlyArray<readonly Clause[]> = [skillFloor, ruleFloor, memoryAnthropicFloor, memoryAgentsMdFloor];

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
  assert.equal(typeof rule, "function");
  assert.equal(typeof memory, "function");
});
