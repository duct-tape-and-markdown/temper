/**
 * The node-set/edge-scope clause constructors: `count`/`unique`/`membership`/`degree`/
 * `reachedFrom` compose a
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
  fresh,
  locusDeclared,
  harness,
  membership,
  mentionReachable,
  mustDefine,
  optional,
  range,
  reachable,
  reachedFrom,
  required,
  requireSections,
  rootDefaultContract,
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

test("degree carries the by-incidence field set its bounds range over", () => {
  // The filter is the clause's own, not either direction's: one `fields` slot beside
  // the direction args, the slot `reachedFrom`'s via set reuses.
  assert.deepEqual(degree({ incoming: { max: 1 }, fields: ["writes", "clobbers"] }), {
    key: "degree",
    args: { incoming_max: 1 },
    fields: ["writes", "clobbers"],
 });
});

test("a degree filter lands the lock row's shared fields column, absent when unfiltered", () => {
  const filtered = skillClauseRow(degree({ incoming: { max: 1 }, fields: ["writes"] }));
  assert.deepEqual(filtered.fields, ["writes"]);
  assert.deepEqual(filtered.degree, { incoming: { min: undefined, max: 1 }, outgoing: undefined });
  assert.equal(skillClauseRow(degree({ incoming: { max: 1 } })).fields, undefined);
});

test("reachedFrom composes a roots requirement and a via field set as an ordinary predicate", () => {
  // The roots ride `membership`'s own `target` slot and the via set `degree`'s own
  // `fields` slot — the two channels the clause already has, never a third scheme.
  assert.deepEqual(reachedFrom("entrypoint", ["routes_to", "delegates_to"]), {
    key: "reached-from",
    target: "entrypoint",
    fields: ["routes_to", "delegates_to"],
 });
  // No via set ⇒ unfiltered, the slot absent exactly as an unfiltered `degree` leaves it.
  assert.deepEqual(reachedFrom("entrypoint"), { key: "reached-from", target: "entrypoint" });
});

test("a reachedFrom clause lands its roots in target and its via set in the shared fields column", () => {
  const predicate = reachedFrom("entrypoint", ["routes_to", "delegates_to"]);
  const row = skillClauseRow(predicate);
  assert.equal(row.target, "entrypoint");
  // Authored order, and a fresh array: the predicate's set is read-only, the row's
  // column is the mutable one the engine decodes (`src/contract.rs` `predicate_from_row`).
  assert.deepEqual(row.fields, ["routes_to", "delegates_to"]);
  assert.notEqual(row.fields, predicate.fields);
  // Unfiltered: the column is absent, which the engine lifts as `via: None`.
  assert.equal(skillClauseRow(reachedFrom("entrypoint")).fields, undefined);
});

test("a reachedFrom clause rides the same severity/guidance/cite channels as any clause", () => {
  const demand = clause(reachedFrom("entrypoint", ["routes_to"]), {
    severity: "advisory",
    guidance: "every flow hangs off an entrypoint",
    cite: "specs/decisions/0056-reached-from-joins-the-vocabulary.md",
 });
  assert.equal(demand.predicate.key, "reached-from");
  assert.equal(demand.severity, "advisory");
  assert.equal(demand.guidance, "every flow hangs off an entrypoint");
  assert.equal(demand.cite, "specs/decisions/0056-reached-from-joins-the-vocabulary.md");
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

/**
 * Compile several `expect` clauses of one predicate on `skill` and return their lock
 * rows in authored order — [`skillClauseRow`]'s plural sibling, for the collision
 * cases where the point is that one kind carries more than one row of a key.
 */
function skillClauseRowsFor(key: string, predicates: readonly Predicate[]): ClauseRow[] {
  const h = harness({
    members: [skill({ name: "gate", description: "Use when gating the run.", prose: text`# Gate` })],
    expect: [
      {
        kind: skill,
        clauses: predicates.map((predicate) => clause(predicate, { severity: "required" })),
      },
    ],
  });
  return compileDeclarations(h).clauses.filter(
    (row) => row.kind === "skill" && row.predicate === key,
  );
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

test("reachable composes a field-less graph-scope predicate lowering to a kind-less root row", () => {
  // The one predicate naming neither a field nor a target: its selection is the root's
  // — the whole governed forest — and its verdict is the reachability closure's, so
  // there is no member field for it to carry.
  assert.deepEqual(reachable(), { key: "reachable" });

  // Lowered off the root's own `contract`, it takes neither a `kind` column (the
  // discriminator that makes the row the root's) nor a `field` one.
  const rows = compileDeclarations(
    harness({ members: [], contract: [clause(reachable(), { severity: "advisory" })] }),
  ).clauses;
  assert.equal(rows.length, 1, `exactly one root row, got ${JSON.stringify(rows)}`);
  assert.equal(rows[0]!.predicate, "reachable");
  assert.equal(rows[0]!.kind, undefined);
  assert.equal(rows[0]!.field, undefined);
  assert.equal(rows[0]!.severity, "advisory");

  // And the shipped default binds this clause, `fresh` and `locus-declared`, all three at
  // advisory — the tool never decides that a dead registration, a drifted projection or an
  // undeclared document blocks.
  assert.deepEqual(
    rootDefaultContract.map((c) => [c.predicate.key, c.severity]),
    [
      ["reachable", "advisory"],
      ["fresh", "advisory"],
      ["locus-declared", "advisory"],
    ],
  );
  assert.ok(
    rootDefaultContract.every((c) => c.guidance),
    "every shipped floor clause teaches",
  );
  // And cites where its verdict rests on an external fact: `reachable`'s dead-channel
  // criteria are Claude Code's documented behaviour. `fresh` and `locusDeclared` compare
  // temper's own lock against disk, so there is no external fact for either to cite and a
  // URL here would be invented.
  assert.ok(
    rootDefaultContract.find((c) => c.predicate.key === "reachable")?.cite,
    "`reachable` cites the docs its dead-channel criteria read off",
  );
});

test("fresh composes a field-less lock-vs-disk predicate lowering to a kind-less root row", () => {
  // Argument-free like `reachable()`: the argument is the committed lock read against
  // disk, so there is no member field, target or gate for it to carry.
  assert.deepEqual(fresh(), { key: "fresh" });

  // Lowered off the root's own `contract`, it takes neither a `kind` column (the
  // discriminator that makes the row the root's) nor a `field` one.
  const rows = compileDeclarations(
    harness({ members: [], contract: [clause(fresh(), { severity: "required" })] }),
  ).clauses;
  assert.equal(rows.length, 1, `exactly one root row, got ${JSON.stringify(rows)}`);
  assert.equal(rows[0]!.predicate, "fresh");
  assert.equal(rows[0]!.kind, undefined);
  assert.equal(rows[0]!.field, undefined);
  // Severity is the author's: hardening drift to blocking is composition, never a dial
  // the tool pre-decides.
  assert.equal(rows[0]!.severity, "required");
});

test("locusDeclared composes a field-less walk-vs-lock predicate lowering to a kind-less root row", () => {
  // Argument-free like `fresh()`: the subject is the discovery walk read against the
  // lock's declarations, so there is no member field, target or gate for it to carry.
  assert.deepEqual(locusDeclared(), { key: "locus-declared" });

  // Lowered off the root's own `contract`, it takes neither a `kind` column (the
  // discriminator that makes the row the root's) nor a `field` one.
  const rows = compileDeclarations(
    harness({ members: [], contract: [clause(locusDeclared(), { severity: "required" })] }),
  ).clauses;
  assert.equal(rows.length, 1, `exactly one root row, got ${JSON.stringify(rows)}`);
  assert.equal(rows[0]!.predicate, "locus-declared");
  assert.equal(rows[0]!.kind, undefined);
  assert.equal(rows[0]!.field, undefined);
  // Severity is the author's, and it is its own dial: hardening an undeclared document to
  // blocking leaves `fresh`'s stale pins exactly where the author left them.
  assert.equal(rows[0]!.severity, "required");
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

// Both predicates address a *section* rather than a field, so neither factory sets
// `Predicate.field` — and the `field` column is what emit stamps a clause's label
// from. Reading the predicate's own (absent) field left every clause of one of these
// predicates on one kind wearing the same label, which admissibility refuses as a
// malformed lock: the author could declare only one of each per kind.
test("two section_contains clauses on one kind land two distinct field columns", () => {
  const rows = skillClauseRowsFor("section_contains", [
    sectionContains("Invariant", "Test"),
    sectionContains("Invariant", "Standard"),
    sectionContains("Decision", "Rejected"),
  ]);
  assert.deepEqual(
    rows.map((row) => row.field),
    ["Invariant.Test", "Invariant.Standard", "Decision.Rejected"],
  );
  // The section column the Rust reader actually reconstructs the predicate from is
  // untouched by the synthesized identity.
  assert.deepEqual(rows[0]!.section, { heading: "Invariant", marker: "Test" });
  assert.deepEqual(rows[1]!.section, { heading: "Invariant", marker: "Standard" });
});

test("two require_sections clauses on one kind land two distinct field columns", () => {
  const rows = skillClauseRowsFor("require_sections", [
    requireSections(["Usage", "Decision"]),
    requireSections(["Installation", "Example"]),
  ]);
  assert.deepEqual(
    rows.map((row) => row.field),
    ["Usage+Decision", "Installation+Example"],
  );
  assert.deepEqual(rows[0]!.sections, ["Usage", "Decision"]);
  assert.deepEqual(rows[1]!.sections, ["Installation", "Example"]);
});

test("two reached-from clauses on one kind land two distinct field columns", () => {
  // A `reached-from` clause names no field, so the column emit stamps the label from
  // carries its identity instead: the roots, plus the via filter that tells two
  // closures off one requirement apart. Sorted and `+`-joined, so two spellings of one
  // via set are one address; an unfiltered closure is the roots alone.
  const rows = skillClauseRowsFor("reached-from", [
    reachedFrom("entrypoint", ["routes_to"]),
    reachedFrom("entrypoint", ["delegates_to", "routes_to"]),
    reachedFrom("root-rule"),
  ]);
  assert.deepEqual(
    rows.map((row) => row.field),
    ["entrypoint.routes_to", "entrypoint.delegates_to+routes_to", "root-rule"],
 );
  // The columns the Rust reader actually reconstructs the predicate from are untouched
  // by the synthesized identity — the via set keeps its authored order.
  assert.deepEqual(
    rows.map((row) => row.target),
    ["entrypoint", "entrypoint", "root-rule"],
 );
  assert.deepEqual(rows[1]!.fields, ["delegates_to", "routes_to"]);
  assert.equal(rows[2]!.fields, undefined);
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
