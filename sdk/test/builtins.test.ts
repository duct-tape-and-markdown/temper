/**
 * The built-in default contracts: every default contract exported from `claude-code.ts` is a
 * well-formed clause array, and every clause carries a non-empty `cite` — the
 * auditability guarantee a maintained default contract exists to keep.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import type { Clause } from "../src/index.js";
import {
  agent,
  agentDefaultContract,
  command,
  commandDefaultContract,
  hookDefaultContract,
  mcpServer,
  mcpServerDefaultContract,
  memory,
  memoryAnthropicDefaultContract,
  rule,
  ruleDefaultContract,
  skill,
  skillDefaultContract,
  supportingDoc,
  supportingDocDefaultContract,
} from "../src/claude-code.js";

const DEFAULT_CONTRACTS: ReadonlyArray<readonly Clause[]> = [
  agentDefaultContract,
  skillDefaultContract,
  commandDefaultContract,
  hookDefaultContract,
  mcpServerDefaultContract,
  ruleDefaultContract,
  memoryAnthropicDefaultContract,
  supportingDocDefaultContract,
];

test("every exported default contract is a well-formed clause array", () => {
  for (const defaultContract of DEFAULT_CONTRACTS) {
    assert.ok(Array.isArray(defaultContract));
    for (const entry of defaultContract) {
      assert.ok(entry.predicate && typeof entry.predicate.key === "string" && entry.predicate.key.length > 0);
      assert.ok(entry.severity === "required" || entry.severity === "advisory");
 }
 }
});

test("every default contract clause carries a non-empty cite", () => {
  for (const defaultContract of DEFAULT_CONTRACTS) {
    for (const entry of defaultContract) {
      assert.ok(typeof entry.cite === "string" && entry.cite.length > 0, `clause \`${entry.predicate.key}\` is uncited`);
 }
 }
});

test("skillDefaultContract carries the skill kind's decidable clauses, name-first", () => {
  assert.equal(skillDefaultContract.length, 13);
  assert.equal(skillDefaultContract[0].predicate.key, "required");
  assert.equal(skillDefaultContract[0].predicate.field, "name");
  assert.deepEqual(
    skillDefaultContract.map((c) => c.predicate.key),
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
      "glob-valid",
    ],
  );
  // The `glob-valid` clause ranges over the `paths` scope.
  assert.equal(skillDefaultContract[12].predicate.key, "glob-valid");
  assert.equal(skillDefaultContract[12].predicate.field, "paths");
});

test("commandDefaultContract is skillDefaultContract minus the directory-name clause", () => {
  assert.deepEqual(
    commandDefaultContract.map((c) => c.predicate.key),
    skillDefaultContract.map((c) => c.predicate.key).filter((key) => key !== "name-matches-dir"),
  );
  assert.equal(
    commandDefaultContract.some((c) => c.predicate.key === "name-matches-dir"),
    false,
    "a command is a lone file — no parent directory to match",
  );
  // `name` requiredness rides over unchanged: a command still declares no `name`
  // field for identity (file-stem, like `rule`), but the skill schema's own
  // `required`/`min_len`/`allowed_chars`/`max_len`/`deny` clauses over `name` still
  // apply by import.
  assert.equal(commandDefaultContract[0].predicate.key, "required");
  assert.equal(commandDefaultContract[0].predicate.field, "name");
});

test("ruleDefaultContract forbids Cursor keys, validates path globs, and budgets body size", () => {
  assert.deepEqual(
    ruleDefaultContract.map((c) => c.predicate.key),
    ["forbidden_keys", "glob-valid", "max_lines"],
  );
  assert.deepEqual(ruleDefaultContract[0].predicate.keys, ["description", "globs", "alwaysApply"]);
  // The `glob-valid` clause ranges over the one documented rules key, `paths`.
  assert.equal(ruleDefaultContract[1].predicate.field, "paths");
});

test("memoryAnthropicDefaultContract is a single advisory size budget", () => {
  assert.equal(memoryAnthropicDefaultContract.length, 1);
  assert.equal(memoryAnthropicDefaultContract[0].predicate.key, "max_lines");
  assert.equal(memoryAnthropicDefaultContract[0].severity, "advisory");
});

test("mcpServer is a fields-only manifest kind at the mcpServers.* collection address", () => {
  assert.equal(mcpServer.facts.shape, "fields");
  assert.equal(mcpServer.facts.unitShape, "file");
  assert.equal(mcpServer.facts.format, undefined);
  assert.deepEqual(mcpServer.facts.locus, { kind: "at", root: ".", glob: ".mcp.json" });
  assert.deepEqual(mcpServer.facts.registration, [{ via: "connection" }]);
  assert.deepEqual(mcpServer.facts.collectionAddress, {
    manifest: ".mcp.json",
    keyPath: "mcpServers.*",
  });
});

test("mcpServerDefaultContract gates the transport type against the documented set", () => {
  assert.deepEqual(
    mcpServerDefaultContract.map((c) => c.predicate.key),
    ["enum"],
  );
  assert.equal(mcpServerDefaultContract[0].predicate.field, "type");
  assert.deepEqual(mcpServerDefaultContract[0].predicate.values, [
    "stdio",
    "http",
    "streamable-http",
    "sse",
    "ws",
  ]);
  assert.equal(mcpServerDefaultContract[0].severity, "required");
});

test("the default contracts ride alongside their kinds through the claude-code subpath", () => {
  assert.equal(typeof agent, "function");
  assert.equal(typeof skill, "function");
  assert.equal(typeof command, "function");
  assert.equal(typeof rule, "function");
  assert.equal(typeof memory, "function");
  assert.equal(typeof supportingDoc, "function");
});

test("skill templates one file-child layer of supporting-doc at the directory's markdown", () => {
  assert.equal(skill.facts.templates?.length, 1);
  const [reference] = skill.facts.templates ?? [];
  // The child travels by import, never by string — the template holds the kind value.
  assert.equal(reference.kind, supportingDoc);
  assert.equal(reference.kind.key, "supporting-doc");
  // A file layer, so it carries the path its children sit at relative to the skill's
  // own unit: the documented `my-skill/reference.md` placement. A supporting file of
  // another type matches nothing here and stays unmodeled rather than mis-typed.
  assert.equal(reference.path, "*.md");
});

test("supporting-doc is a nested-file kind: fields-free, prose-only, channel-less, identity from the filename", () => {
  assert.deepEqual(supportingDoc.facts.locus, { kind: "nested-file" });
  // Frontmatterless — no declared format, so the whole file is body.
  assert.equal(supportingDoc.facts.format, undefined);
  // A lone file whose identity is its stem: no identityField carries the name.
  assert.equal(supportingDoc.facts.unitShape, "file");
  assert.equal(supportingDoc.facts.identityField, undefined);
  // Channel-less: it reaches the world only through the skill that references it.
  assert.deepEqual(supportingDoc.facts.registration, []);
  // Fields-free, but still body-bearing — never the fields-only registration shape.
  assert.equal(supportingDoc.facts.shape, undefined);
  const member = supportingDoc({ name: "reference", host: skill({ name: "demo", description: "A host." }) });
  assert.deepEqual(member.fields, []);
});

test("supportingDocDefaultContract ships empty — the format documents no schema to gate", () => {
  assert.deepEqual(supportingDocDefaultContract, []);
});

test("command is a file-shaped unit with no identityField, unlike the directory-shaped skill", () => {
  assert.equal(command.facts.unitShape, "file");
  assert.equal(command.facts.identityField, undefined);
  assert.equal(skill.facts.unitShape, "directory");
  assert.equal(skill.facts.identityField, "name");
});

test("agent is a named-field unit whose identity comes from its own name field", () => {
  assert.equal(agent.facts.unitShape, "named-field");
  assert.equal(agent.facts.identityField, "name");
  assert.equal(agent.facts.format, "yaml-frontmatter");
  assert.deepEqual(agent.facts.locus, { kind: "at", root: ".claude/agents", glob: "**/*.md" });
});

test("skill/command register on both documented invocation channels; agent/rule/memory carry a singleton set", () => {
  assert.deepEqual(skill.facts.registration, [
    { via: "user-invoked" },
    { via: "description-trigger", field: "description" },
  ]);
  assert.deepEqual(command.facts.registration, [
    { via: "user-invoked" },
    { via: "description-trigger", field: "description" },
  ]);
  assert.deepEqual(agent.facts.registration, [{ via: "description-trigger", field: "description" }]);
  assert.deepEqual(rule.facts.registration, [{ via: "paths-match", field: "paths" }]);
  assert.deepEqual(memory.facts.registration, [{ via: "always" }]);
});

test("agentDefaultContract requires name and description, gates the lowercase-hyphen charset, and pins per-scope uniqueness", () => {
  assert.deepEqual(
    agentDefaultContract.map((c) => c.predicate.key),
    ["required", "allowed_chars", "unique-name", "required"],
  );
  assert.deepEqual(
    agentDefaultContract.map((c) => c.predicate.field),
    ["name", "name", undefined, "description"],
  );
  const charset = agentDefaultContract[1].predicate.charset;
  assert.deepEqual(charset, { ranges: ["a-z"], chars: "-" });
});

test("an agent member's identity field writes name first, then the typed description", () => {
  const member = agent({
    name: "code-reviewer",
    description: "Use when reviewing a pull request for correctness.",
  });
  assert.deepEqual(member.fields, [
    ["name", "code-reviewer"],
    ["description", "Use when reviewing a pull request for correctness."],
  ]);
});

test("disable-model-invocation/user-invocable/paths are ordinary declared fields on a skill member", () => {
  const member = skill({
    name: "demo",
    description: "Use when demonstrating a skill's modulating fields.",
    "disable-model-invocation": true,
    "user-invocable": false,
    paths: ["src/**"],
  });
  assert.deepEqual(member.fields, [
    ["name", "demo"],
    ["description", "Use when demonstrating a skill's modulating fields."],
    ["disable-model-invocation", true],
    ["user-invocable", false],
    ["paths", ["src/**"]],
  ]);
  // paths gates the existing invocation channels, so it adds no registration
  // channel of its own — unlike a rule's paths-match.
  assert.deepEqual(
    skill.facts.registration,
    [{ via: "user-invoked" }, { via: "description-trigger", field: "description" }],
  );
});
