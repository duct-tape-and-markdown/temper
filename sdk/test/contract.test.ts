/**
 * The node-set/edge-scope clause constructors (`REQUIREMENT-CLAUSES-ALGEBRA`,
 * `specs/model/contract.md`, "Judged at the node-set scope" / "Judged
 * at the edge scope"): `count`/`unique`/`membership`/`degree` compose a
 * set-/edge-scope demand as an ordinary `Predicate` value, peers of the
 * node-scope constructors (`required`, `maxLines`, …) already in `contract.ts`.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { clause, count, degree, membership, unique } from "../src/index.js";

test("count composes a satisfier-set-size bound as an ordinary predicate", () => {
  assert.deepEqual(count({ min: 1, max: 3 }), { key: "count", args: { min: 1, max: 3 } });
  // A one-sided bound carries only the given endpoint.
  assert.deepEqual(count({ min: 1 }), { key: "count", args: { min: 1 } });
});

test("unique composes a field's set-wide uniqueness as an ordinary predicate", () => {
  assert.deepEqual(unique("name"), { key: "unique", field: "name" });
});

test("membership composes a target-requirement draw as an ordinary predicate", () => {
  assert.deepEqual(membership("model", "approved-models"), {
    key: "membership",
    field: "model",
    target: "approved-models",
  });
});

test("degree composes an in/out edge-count bound as an ordinary predicate", () => {
  assert.deepEqual(degree({ incoming: { min: 1 }, outgoing: { max: 3 } }), {
    key: "degree",
    args: { incoming_min: 1, outgoing_max: 3 },
  });
});

test("every set-/edge-scope predicate composes into a clause value like any other", () => {
  const demand = clause(count({ min: 1, max: 1 }), {
    severity: "required",
    guidance: "exactly one release-tool",
  });
  assert.equal(demand.predicate.key, "count");
  assert.equal(demand.severity, "required");
  assert.equal(demand.guidance, "exactly one release-tool");
});
