/**
 * Assembly artifacts — emit compiles the two locus-less assembly facts (the kind
 * bindings and the requirement roster) into small committed temper-owned TOML
 * files (`specs/architecture/20-surface.md`, "the bindings, the roster — are
 * emitted as small committed temper-owned artifacts"). These pin the byte shape,
 * that every declared binding and requirement carries through, double-emit
 * stability, and that `writeEmit` lands both beside `temper.toml`/`lock.toml`.
 */

import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import {
  assemblyArtifacts,
  defineHarness,
  emit,
  emitManifestMembers,
  md,
  rule,
  serializeBindings,
  serializeRoster,
  skill,
  writeEmit,
} from "../src/index.js";

// ---------------------------------------------------------------------------
// Byte shape — the two artifacts serialize through the shared `toml.ts`
// encoders: sorted keys, one blank line before every table header but the first,
// a dotted kind quoted into a single sub-key.
// ---------------------------------------------------------------------------

test("serializeBindings emits a sorted `[binding.<kind>]` table per declared binding", () => {
  // Passed skill-first; the encoder sorts kinds (rule < skill). A dotted kind
  // rides one quoted sub-key so it never splits into nested tables.
  const bindings = serializeBindings({
    "claude-code.skill": { package: "skill.anthropic" },
    "claude-code.rule": { package: "rule.anthropic" },
  });
  assert.equal(
    bindings,
    `[binding."claude-code.rule"]\n` +
      `package = "rule.anthropic"\n` +
      `\n` +
      `[binding."claude-code.skill"]\n` +
      `package = "skill.anthropic"\n`,
  );
});

test("serializeRoster carries each requirement's means, kind, and required flag", () => {
  // Requirements name-sorted (agent-playbook < engineering-standards); `required`
  // is emitted only when set, and its absence leaves a two-line table.
  const roster = serializeRoster({
    "engineering-standards": {
      means: "the repo carries a rule fixing the engineering bar",
      kind: "claude-code.rule",
    },
    "agent-playbook": {
      means: "a shared agent playbook exists",
      kind: "claude-code.skill",
      required: true,
    },
  });
  assert.equal(
    roster,
    `[requirement.agent-playbook]\n` +
      `means = "a shared agent playbook exists"\n` +
      `kind = "claude-code.skill"\n` +
      `required = true\n` +
      `\n` +
      `[requirement.engineering-standards]\n` +
      `means = "the repo carries a rule fixing the engineering bar"\n` +
      `kind = "claude-code.rule"\n`,
  );
});

test("empty bindings and roster serialize to empty artifacts", () => {
  assert.equal(serializeBindings({}), "");
  assert.equal(serializeRoster({}), "");
});

// ---------------------------------------------------------------------------
// The authoring face end to end — a well-formed harness whose required
// requirement is filled, so emit's declare-side refusals pass.
// ---------------------------------------------------------------------------

function harness() {
  return defineHarness({
    kinds: {
      "claude-code.rule": { package: "rule.anthropic" },
      "claude-code.skill": { package: "skill.anthropic" },
    },
    requirements: {
      "engineering-standards": {
        means: "the repo carries a rule fixing the engineering bar",
        kind: "claude-code.rule",
      },
      "agent-playbook": {
        means: "a shared agent playbook exists",
        kind: "claude-code.skill",
        required: true,
      },
    },
    members: [
      rule({
        name: "rust",
        fields: { paths: ["src/**/*.rs"] },
        satisfies: { "engineering-standards": { rationale: "carries the Rust conventions" } },
        body: md`
          # Rust conventions

          Errors via miette/thiserror.
        `,
      }),
      skill({
        name: "coordinate",
        fields: { description: "Drive a complex task across a team of agents." },
        satisfies: { "agent-playbook": { rationale: "the shared playbook" } },
        body: md`
          # Coordinate

          Drive the team.
        `,
      }),
    ],
  });
}

test("assemblyArtifacts compiles both facts straight off the harness", () => {
  const artifacts = assemblyArtifacts(harness());
  assert.equal(artifacts.bindings, serializeBindings(harness().kinds));
  assert.equal(artifacts.roster, serializeRoster(harness().requirements));
});

test("emit returns the bindings and roster carrying every declared fact", () => {
  const result = emit(harness());

  // Every declared binding is present, each naming its bound package.
  assert.match(result.bindings, /^\[binding\."claude-code\.rule"\]$/m);
  assert.match(result.bindings, /package = "rule\.anthropic"/);
  assert.match(result.bindings, /^\[binding\."claude-code\.skill"\]$/m);
  assert.match(result.bindings, /package = "skill\.anthropic"/);

  // Every declared requirement is present with its means/kind, and the `required`
  // one carries its flag.
  assert.match(result.roster, /^\[requirement\.agent-playbook\]$/m);
  assert.match(result.roster, /means = "a shared agent playbook exists"/);
  assert.match(result.roster, /kind = "claude-code\.skill"/);
  assert.match(result.roster, /required = true/);
  assert.match(result.roster, /^\[requirement\.engineering-standards\]$/m);
});

test("emit is additive: the manifest and lock are unchanged by the new artifacts", () => {
  const result = emit(harness());
  // The manifest bytes still equal the standalone manifest emitter — the assembly
  // artifacts sit beside it, they do not perturb it.
  assert.equal(result.manifest, emitManifestMembers(harness()));
  // The lock still carries a row per projected member.
  assert.match(result.lock, /^\[\[rule\]\]$/m);
  assert.match(result.lock, /^\[\[skill\]\]$/m);
});

test("the assembly artifacts are byte-deterministic across two emits", () => {
  const a = emit(harness());
  const b = emit(harness());
  assert.equal(a.bindings, b.bindings);
  assert.equal(a.roster, b.roster);
});

test("writeEmit lands bindings.toml and roster.toml beside temper.toml and lock.toml", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-assembly-"));
  try {
    const result = writeEmit(harness(), dir);
    assert.equal(readFileSync(join(dir, "bindings.toml"), "utf8"), result.bindings);
    assert.equal(readFileSync(join(dir, "roster.toml"), "utf8"), result.roster);
    // The additive outputs still land too.
    assert.equal(readFileSync(join(dir, "temper.toml"), "utf8"), result.manifest);
    assert.equal(readFileSync(join(dir, "lock.toml"), "utf8"), result.lock);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
