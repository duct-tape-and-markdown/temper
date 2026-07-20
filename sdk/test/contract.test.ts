/**
 * The node-set/edge-scope clause constructors: `count`/`unique`/`membership`/`degree` compose a
 * set-/edge-scope demand as an ordinary `Predicate` value, peers of the
 * node-scope constructors (`required`, `extent`, …) already in `contract.ts`.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import {
  clause,
  count,
  degree,
  enumOf,
  extent,
  formatPlacesEdges,
  harness,
  membership,
  mentionReachable,
  mustDefine,
  optional,
  range,
  required,
  requireSections,
  script,
  sectionContains,
  telemetry,
  text,
  unique,
  when,
} from "../src/index.js";
import type { Predicate, Requirement } from "../src/index.js";
import { compileDeclarations } from "../src/declarations.js";
import type { ClauseRow, RequirementRow } from "../src/declarations.js";
import { skill } from "../src/claude-code.js";

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

// The five evaluable predicates that became SDK-authorable — each composes an
// ordinary `Predicate` value whose arguments erase into the lock row's own columns,
// the wire form the engine decodes (`src/contract.rs` `predicate_from_row`).

/**
 * Compile a single `expect` clause on `skill` and return its lock row — the erased
 * wire form the engine reads back. Filtering by the predicate's own key isolates it
 * from `skill`'s floor clauses.
 */
function skillClauseRow(predicate: Predicate): ClauseRow {
  const h = harness({
    members: [skill({ name: "gate", description: "Use when gating the run.", prose: text`# Gate` })],
    expect: [{ kind: skill, clauses: [clause(predicate, { severity: "required" })] }],
  });
  const rows = compileDeclarations(h).clauses.filter(
    (row) => row.kind === "skill" && row.predicate === predicate.key,
  );
  assert.equal(rows.length, 1, `exactly one ${predicate.key} row`);
  return rows[0]!;
}

test("optional composes a schema-membership predicate landing its field column", () => {
  assert.deepEqual(optional("model"), { key: "optional", field: "model" });
  assert.equal(skillClauseRow(optional("model")).field, "model");
});

test("range composes an inclusive numeric bound landing its range column", () => {
  assert.deepEqual(range("priority", 1, 5), {
    key: "range",
    field: "priority",
    range: { min: 1, max: 5 },
  });
  const row = skillClauseRow(range("priority", 1, 5));
  assert.deepEqual(row.range, { min: 1, max: 5 });
  assert.equal(row.field, "priority");
});

test("enumOf composes a permitted-value set riding the deny-precedent values column", () => {
  assert.deepEqual(enumOf("status", ["draft", "final"]), {
    key: "enum",
    field: "status",
    values: ["draft", "final"],
  });
  const row = skillClauseRow(enumOf("status", ["draft", "final"]));
  assert.deepEqual(row.values, ["draft", "final"]);
  assert.equal(row.field, "status");
});

test("mentionReachable composes both field ends, the target's gate landing its own column", () => {
  assert.deepEqual(mentionReachable("paths", "paths"), {
    key: "mention-reachable",
    field: "paths",
    gate: "paths",
  });
  // The one two-argument predicate: `field` carries the source's scope, `gate` the
  // target's — the column the engine reads back (`src/contract.rs`
  // `predicate_from_row`), so both halves of the seam must spell one name.
  const row = skillClauseRow(mentionReachable("scope", "gate-field"));
  assert.equal(row.field, "scope");
  assert.equal(row.gate, "gate-field");
});

test("mustDefine composes a body marker landing in the field column", () => {
  assert.deepEqual(mustDefine("disable-model-invocation"), {
    key: "must_define",
    field: "disable-model-invocation",
  });
  assert.equal(skillClauseRow(mustDefine("disable-model-invocation")).field, "disable-model-invocation");
});

test("sectionContains composes a heading/marker predicate landing its section column", () => {
  assert.deepEqual(sectionContains("Decision", "Rejected"), {
    key: "section_contains",
    section: { heading: "Decision", marker: "Rejected" },
  });
  assert.deepEqual(skillClauseRow(sectionContains("Decision", "Rejected")).section, {
    heading: "Decision",
    marker: "Rejected",
  });
});

test("formatPlacesEdges composes a format-verification predicate with no field", () => {
  assert.deepEqual(formatPlacesEdges(), { key: "format-places-edges" });
  assert.equal(skillClauseRow(formatPlacesEdges()).predicate, "format-places-edges");
});

test("extent composes a render-side budget landing its unit and bound columns", () => {
  assert.deepEqual(extent("lines", 300), {
    key: "extent",
    unit: "lines",
    args: { max: 300 },
  });
  // The row carries both the unit and the bound — the wire form the Rust reader lifts
  // into `Predicate::Extent`, so the seam spells one `unit` name on both sides.
  const row = skillClauseRow(extent("characters", 4000));
  assert.equal(row.unit, "characters");
  assert.deepEqual(row.bound, { min: undefined, max: 4000 });
});

// `Requirement.kind` carries only the kind's identity for coverage resolution,
// never its field type — so a kind whose fields carry required members (skill,
// hook) assigns, where the former `KindDefinition<never>` rejected it. A bare
// kind-name string resolves to itself.
function requirementRow(kind: Requirement["kind"]): RequirementRow {
  const h = harness({
    members: [skill({ name: "gate", description: "Use when gating the run.", prose: text`# Gate` })],
    require: { "front-door": { prose: "the harness ships a front-door skill", kind } },
  });
  const rows = compileDeclarations(h).requirements.filter((row) => row.name === "front-door");
  assert.equal(rows.length, 1, "exactly one front-door requirement row");
  return rows[0]!;
}

test("a requirement keyed to a required-member kind type-checks and emits its identity", () => {
  assert.equal(requirementRow(skill).kind, "skill");
});

test("a requirement keyed to a bare kind-name string emits it verbatim", () => {
  assert.equal(requirementRow("skill").kind, "skill");
});

/**
 * Compile a requirement carrying `verifier` and return its lock row's `verifier`
 * column — the species-tagged wire value the engine reads back.
 */
function verifierRowOf(verifier: Requirement["verifier"]): RequirementRow["verifier"] {
  const h = harness({
    members: [skill({ name: "gate", description: "Use when gating the run.", prose: text`# Gate` })],
    require: { "front-door": { prose: "the harness ships a front-door skill", verifier } },
  });
  const rows = compileDeclarations(h).requirements.filter((row) => row.name === "front-door");
  assert.equal(rows.length, 1, "exactly one front-door requirement row");
  return rows[0]!.verifier;
}

test("the script constructor composes a path-tagged verifier and lowers to its species row", () => {
  assert.deepEqual(script("tests/dev-standards.test.ts"), {
    species: "script",
    path: "tests/dev-standards.test.ts",
  });
  // The wire key is `species` on both halves of the seam — the Rust reader
  // (`src/drift.rs` `verifier_from_table`) matches it byte-for-byte.
  assert.deepEqual(verifierRowOf(script("tests/dev-standards.test.ts")), {
    species: "script",
    path: "tests/dev-standards.test.ts",
  });
});

test("the telemetry constructor composes an event-tagged verifier and lowers its names", () => {
  assert.deepEqual(telemetry(["SkillInvoked", "ToolUse"]), {
    species: "telemetry",
    events: ["SkillInvoked", "ToolUse"],
  });
  assert.deepEqual(verifierRowOf(telemetry(["SkillInvoked", "ToolUse"])), {
    species: "telemetry",
    events: ["SkillInvoked", "ToolUse"],
  });
});

test("a requirement with no verifier lowers to an absent verifier column", () => {
  assert.equal(verifierRowOf(undefined), undefined);
});

test("requireSections composes a heading-list predicate landing its sections column", () => {
  assert.deepEqual(requireSections(["Usage", "Decision"]), {
    key: "require_sections",
    sections: ["Usage", "Decision"],
  });
  const row = skillClauseRow(requireSections(["Installation", "Example"]));
  assert.deepEqual(row.sections, ["Installation", "Example"]);
});

test("when composes a guarded clause with a guard predicate and a body of clauses", () => {
  const guardClause = enumOf("source", ["./path", "{ plugin: ... }"]);
  const bodyClause = clause(required("object"), { severity: "required" });
  const whenClause = when(guardClause, [bodyClause]);

  assert.equal(whenClause.predicate.key, "when");
  assert.equal(whenClause.severity, "required");
  assert.deepEqual(whenClause.when_guard, guardClause);
  assert.deepEqual(whenClause.when_body, [bodyClause]);
});

test("when round-trips through compileDeclarations with the guard and body landed", () => {
  const h = harness({
    members: [skill({ name: "gate", description: "Use when gating the run.", prose: text`# Gate` })],
    expect: [
      {
        kind: skill,
        clauses: [
          when(enumOf("source", ["./path", "object"]), [
            clause(required("url"), { severity: "required" }),
          ]),
        ],
      },
    ],
  });

  const rows = compileDeclarations(h).clauses.filter((row) => row.kind === "skill" && row.predicate === "when");
  assert.equal(rows.length, 1, "exactly one when row");

  const row = rows[0]!;
  assert.equal(row.guard_predicate, "enum");
  assert.deepEqual(row.values, ["./path", "object"]);
  assert.equal(row.body?.length, 1);
  assert.equal(row.body?.[0]?.predicate, "required");
  assert.equal(row.body?.[0]?.field, "url");
});
