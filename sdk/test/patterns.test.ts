/**
 * The patterns subpath — the mechanics shapes as typed constructors, and `defaultPatterns()`
 * composing them into a harness that emits the gates, the typecheck and the reference.
 */

import assert from "node:assert/strict";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import { harness, input, requirement, span, text, tool } from "../src/index.js";
import { emit } from "../src/emit.js";
import { rule } from "../src/claude-code.js";
import { defaultPatterns, gateHooks, playbook, reference, typecheck } from "../src/patterns.js";

const project = input(import.meta.url, "./tsconfig.json");

/**
 * `emit` run from a scratch npm program locking `typescript` — the reference's `tool`
 * topic reads a `{ package, bin }` tool's version from the lockfile of the working
 * directory's program, and the default patterns' compiler is one.
 */
function emitInProgram(program: Parameters<typeof emit>[0]): ReturnType<typeof emit> {
  const dir = mkdtempSync(join(tmpdir(), "temper-patterns-"));
  const cwd = process.cwd();
  try {
    writeFileSync(join(dir, "package.json"), JSON.stringify({ packageManager: "npm@10.9.0" }));
    writeFileSync(
      join(dir, "package-lock.json"),
      JSON.stringify({ lockfileVersion: 3, packages: { "node_modules/typescript": { version: "5.6.3" } } }),
    );
    process.chdir(dir);
    return emit(program);
  } finally {
    process.chdir(cwd);
    rmSync(dir, { recursive: true, force: true });
  }
}

/** The gate hooks a result registers, as `event:matcher` (matcher absent, the bare event). */
function gates(result: ReturnType<typeof emit>): string[] {
  return result.registrations
    .filter((registration) => registration.kind === "hook")
    .map((registration) => {
      const matcher = registration.fields.find(([key]) => key === "matcher")?.[1];
      return matcher === undefined ? registration.key : `${registration.key}:${String(matcher)}`;
    })
    .sort();
}

function referenceDescription(result: ReturnType<typeof emit>): unknown {
  const member = result.members.find((m) => m.kind === "skill" && m.name === "temper");
  assert.ok(member, "the reference projects as the `temper` skill");
  return member.fields.find(([key]) => key === "description")?.[1];
}

test("defaultPatterns() composes into a harness that emits the gates, the typecheck and the reference", () => {
  const result = emitInProgram(harness(defaultPatterns({ typecheck: { project } })));

  assert.deepEqual(gates(result), ["PostToolUse:Bash", "PreToolUse:Write|Edit|MultiEdit", "SessionStart"]);

  const row = result.declarations.typecheck;
  assert.ok(row, "the default patterns declare the program's typecheck");
  assert.equal(row.tool, "root/tool/tsc");
  assert.match(row.project, /tsconfig\.json$/);

  const skill = result.members.find((m) => m.kind === "skill" && m.name === "temper");
  assert.ok(skill, "the reference projects as the `temper` skill");
  assert.ok(skill.body.startsWith("# Operating the temper gate\n"), skill.body);
  assert.match(referenceDescription(result) as string, /^Use when operating the temper gate/);
  const toolTopic = result.members.find((m) => m.kind === "supporting-doc" && m.name === "kind-tool");
  assert.ok(toolTopic, "the reference derives a topic for the default patterns' tool kind");
  assert.match(toolTopic.body, /`typescript` — version 5\.6\.3/);
});

test("defaultPatterns() carries no taste: no memory, no rule, no telemetry", () => {
  const base = defaultPatterns({ typecheck: { project } });
  assert.deepEqual(base.members.map((member) => member.kind).sort(), ["hook", "hook", "hook", "skill", "tool"]);
  const result = emitInProgram(harness(base));
  assert.ok(!result.members.some((m) => m.kind === "memory"));
  assert.ok(!result.registrations.some((r) => r.fields.some(([, value]) => JSON.stringify(value).includes("tap"))));
});

test("a consumer composes its own members beside the default patterns'", () => {
  const base = defaultPatterns({ typecheck: { project } });
  const result = emitInProgram(
    harness({
      ...base,
      members: [...base.members, rule({ name: "conventions", satisfies: ["conventions"] })],
      require: { conventions: requirement({ prose: "The code conventions." }) },
    }),
  );
  assert.ok(result.members.some((m) => m.kind === "rule" && m.name === "conventions"));
  assert.match(referenceDescription(result) as string, /Its contracted requirements: `conventions`\.$/);
});

test("defaultPatterns() options override each shape's defaults", () => {
  const compiler = tool({ package: "typescript", bin: "tsc", name: "compiler" });
  const temper = tool({ package: "@dtmd/temper", bin: "temper", name: "gate" });
  const base = defaultPatterns({
    typecheck: { project, tool: compiler },
    gates: { tool: temper },
    reference: { description: "Use when changing this repo's harness." },
  });
  assert.deepEqual(
    base.members.filter((member) => member.kind === "tool").map((member) => member.name),
    ["compiler", "gate"],
  );
  const result = emitInProgram(harness(base));
  assert.equal(result.declarations.typecheck?.tool, "root/tool/compiler");
  assert.ok(result.declarations.runs.some((row) => row.tool === "root/tool/gate"));
  assert.equal(referenceDescription(result), "Use when changing this repo's harness.");
});

test("defaultPatterns() composes the playbooks it is given after its own shapes", () => {
  const area = playbook({
    name: "releasing",
    description: "Cut a release: bump the versions, tag, push. Use when asked to release.",
    facts: text`A release is a tag push.`,
    topics: [{ name: "cutting", when: "cutting a release", prose: span("# Cutting\n") }],
    invocation: "user",
  });
  const base = defaultPatterns({ typecheck: { project }, playbooks: [area] });
  assert.deepEqual(base.members.slice(-3), [...area.members]);
  const result = emitInProgram(harness(base));
  assert.ok(result.members.some((m) => m.kind === "rule" && m.name === "releasing"));
  assert.ok(result.members.some((m) => m.kind === "skill" && m.name === "releasing"));
  assert.ok(result.members.some((m) => m.kind === "supporting-doc" && m.name === "cutting"));
});

test("a tool the typecheck and the gates share is composed once", () => {
  const shared = tool({ package: "@dtmd/temper", bin: "temper" });
  const base = defaultPatterns({ typecheck: { project, tool: shared }, gates: { tool: shared } });
  assert.equal(base.members.filter((member) => member.kind === "tool").length, 1);
});

test("the hydrated reference description still closes on the contracted requirements", () => {
  const result = emit(
    harness({
      members: [reference({ description: "Use when touching the harness." })],
      require: { procedures: requirement({ prose: "How work is done.", required: false }) },
    }),
  );
  assert.equal(referenceDescription(result), "Use when touching the harness. Its contracted requirements: `procedures`.");
});

test("each shape hydrates standalone", () => {
  assert.equal(gateHooks().length, 3);
  const checked = typecheck({ project });
  assert.equal(checked.tool.kind, "tool");
  assert.equal(checked.project, project);
  assert.throws(() => reference({ description: "  " }), /description/);
  assert.throws(() => typecheck({ project, tool: rule({ name: "x" }) }), /tool\(\.\.\.\)/);
  assert.throws(() => gateHooks({ tool: rule({ name: "x" }) }), /tool\(\.\.\.\)/);
});
