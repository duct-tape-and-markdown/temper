/**
 * Emit — the six-noun face compiled to the seam's JSON pipe.
 * A harness authored in the face (`harness()`, `kind<T>()`,
 * clause values, `needs`, `file()`/`text`/`blocks()`) emits the declaration rows
 * (the lock's six families) and every projected member's erased payload — kind,
 * name, ordered fields, resolved body. The engine is the sole compiler of every
 * projection and the whole lock; the SDK writes neither.
 */

import assert from "node:assert/strict";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { test } from "node:test";

import {
  allowedChars,
  bash,
  blocks,
  clause,
  compileDeclarations,
  declarationsToJson,
  embeddedMemberValue,
  emit,
  file,
  forbiddenKeys,
  harness,
  kind,
  maxLen,
  maxLines,
  required,
  text,
} from "../src/index.js";
import * as sdk from "../src/index.js";
import { memory, rule, skill } from "../src/claude-code.js";

function projectedHarness() {
  return harness({
 members: [
      rule({
        name: "rust",
        paths: ["src/**/*.rs"],
        prose: text`
          # Rust conventions

          Errors via miette/thiserror; clippy clean under -D warnings.
        `,
      }),
      skill({
        name: "coordinate",
        description: "Use when driving a complex task across a team of agents.",
        prose: text`
          # Coordinate

          Drive the team.
        `,
      }),
 ],
 });
}

test("emit compiles the projected members and declarations in one pass", () => {
  const result = emit(projectedHarness());

  assert.deepEqual(
    result.members.map((m) => `${m.kind}:${m.name}`),
    ["rule:rust", "skill:coordinate"],
  );
  const rust = result.members.find((m) => m.name === "rust")!;
  assert.deepEqual(rust.fields, [["paths", ["src/**/*.rs"]]]);
  assert.equal(rust.body, "# Rust conventions\n\nErrors via miette/thiserror; clippy clean under -D warnings.\n");

  const coordinate = result.members.find((m) => m.name === "coordinate")!;
  assert.deepEqual(coordinate.fields, [
    ["name", "coordinate"],
    ["description", "Use when driving a complex task across a team of agents."],
  ]);

  // The declaration rows cover both kinds.
  assert.deepEqual(
    result.declarations.kinds.map((k) => k.name),
    ["rule", "skill"],
  );

  // The JSON pipe carries the same declarations and members, versioned.
  const seam = JSON.parse(result.seam);
  assert.equal(seam.version, 2);
  assert.deepEqual(
    seam.declarations.kinds.map((k: { name: string }) => k.name),
    ["rule", "skill"],
  );
  assert.deepEqual(
    seam.members.map((m: { name: string }) => m.name),
    ["rust", "coordinate"],
  );
});

test("emit is byte-stable across a double pass", () => {
  const a = emit(projectedHarness());
  const b = emit(projectedHarness());
  assert.equal(a.seam, b.seam);
});

// ---------------------------------------------------------------------------
// Declaration rows — the six families, and the `satisfies`/`mentions` join the roster/
// coverage tiers need.
// ---------------------------------------------------------------------------

function fullHarness() {
  return harness({
 members: [
      rule({
        name: "rust",
        paths: ["src/**/*.rs"],
        needs: [bash("git diff")],
        satisfies: ["dev-standards"],
        prose: text`# Rust

See ${{ address: "dev-standards", display: "dev standards" }}.`,
      }),
 ],
    expect: [
 {
        kind: rule,
        clauses: [
          clause(maxLines(300), { severity: "advisory" }),
          clause(required("paths"), { severity: "required" }),
 ],
 },
 ],
    require: {
      "dev-standards": {
        means: "the harness maintains development standards",
        kind: rule,
 required: true,
        verifiedBy: "tests/dev-standards.test.ts",
 },
 },
 });
}

test("compileDeclarations produces all seven families, satisfies and mentions included", () => {
  const declarations = compileDeclarations(fullHarness());

  assert.deepEqual(declarations.kinds, [
 {
      name: "rule",
      provider: undefined,
      governs_root: ".claude/rules",
      governs_glob: "*.md",
      format: "yaml-frontmatter",
      unit_shape: "file",
      registration: ["paths-match(paths)"],
      templates: undefined,
 },
  ]);
  assert.deepEqual(declarations.clauses, [
 {
      kind: "rule",
      predicate: "max_lines",
      field: undefined,
      severity: "advisory",
      guidance: undefined,
      cite: undefined,
      count: undefined,
      target: undefined,
      degree: undefined,
      bound: { min: undefined, max: 300 },
      charset: undefined,
      keys: undefined,
      values: undefined,
 },
 {
      kind: "rule",
      predicate: "required",
      field: "paths",
      severity: "required",
      guidance: undefined,
      cite: undefined,
      count: undefined,
      target: undefined,
      degree: undefined,
      bound: undefined,
      charset: undefined,
      keys: undefined,
      values: undefined,
 },
  ]);
  assert.deepEqual(declarations.requirements, [
 {
      name: "dev-standards",
      kind: "rule",
 required: true,
      clauses: [],
      verified_by: "tests/dev-standards.test.ts",
 },
  ]);
  assert.deepEqual(declarations.assembly, [{ fact: "mode", value: "warn" }]);
  assert.deepEqual(declarations.satisfies, [{ member: "rust", requirement: "dev-standards" }]);
  assert.deepEqual(declarations.mentions, [{ member: "rule:rust", target: "dev-standards" }]);
  // No member declares a blocks()-composed embedded member in this harness.
  assert.deepEqual(declarations.nested_members, []);
});

test("compileDeclarations emits no uncoined `authority` fact, and the root member's declared mode round-trips", () => {
  // The hardcoded `{ fact: "authority", value: "shared" }` the corpus never coined
  // (MODE-ROOT-MEMBER-FIELD) is retired — the root member's own declared `mode`
 // field is the sole source of this row now.
  const defaulted = compileDeclarations(fullHarness());
  assert.ok(
    !defaulted.assembly.some((fact) => fact.fact === "authority"),
    "no uncoined `authority` fact is emitted",
  );
  assert.deepEqual(
    defaulted.assembly.find((fact) => fact.fact === "mode"),
    { fact: "mode", value: "warn" },
  );

  const blocked = harness({ ...fullHarness(), mode: "block" });
  assert.deepEqual(
    compileDeclarations(blocked).assembly.find((fact) => fact.fact === "mode"),
    { fact: "mode", value: "block" },
  );
});

test("clauseRow serializes a node-scope predicate's own argument onto the row", () => {
  // A kind's own `expect` clause carries its predicate's bound/charset/keys/values
  // argument, not identity+severity alone — the row a floor Contract must be
 // reconstructable from.
 const h = harness({
    members: [],
    expect: [
 {
        kind: rule,
        clauses: [
          clause(maxLen("name", 64), { severity: "required" }),
          clause(forbiddenKeys(["globs", "alwaysApply"]), { severity: "required" }),
          clause(allowedChars("name", { ranges: ["a-z"], chars: "-" }), { severity: "required" }),
 ],
 },
 ],
 });

  const declarations = compileDeclarations(h);
  assert.deepEqual(
    declarations.clauses.map((c) => c.bound),
    [{ min: undefined, max: 64 }, undefined, undefined],
  );
  assert.deepEqual(
    declarations.clauses.map((c) => c.keys),
    [undefined, ["globs", "alwaysApply"], undefined],
  );
  assert.deepEqual(
    declarations.clauses.map((c) => c.charset),
    [undefined, undefined, { ranges: ["a-z"], chars: "-" }],
  );
});

test("the JSON pipe carries the reduced declaration rows and the pinned version", () => {
  const seam = JSON.parse(declarationsToJson(compileDeclarations(fullHarness())));
  assert.equal(seam.version, 2);
  assert.deepEqual(seam.satisfies, [{ member: "rust", requirement: "dev-standards" }]);
  assert.deepEqual(seam.mentions, [{ member: "rule:rust", target: "dev-standards" }]);
});

test("a member with no satisfies claim contributes no row", () => {
  const h = harness({ members: [rule({ name: "rust", prose: text`# Rust` })] });
  assert.deepEqual(compileDeclarations(h).satisfies, []);
});

test("a member with no mentions in its prose contributes no mention row", () => {
  const h = harness({ members: [rule({ name: "rust", prose: text`# Rust` })] });
  assert.deepEqual(compileDeclarations(h).mentions, []);
});

test("mention rows are member-then-target sorted regardless of authoring order", () => {
  const h = harness({
    members: [
      rule({
        name: "b",
        prose: text`See ${{ address: "rule:a", display: "a" }} and ${{ address: "rule:c", display: "c" }}.`,
      }),
      rule({ name: "a", prose: text`See ${{ address: "rule:b", display: "b" }}.` }),
      rule({ name: "c", prose: text`# C` }),
    ],
  });
  assert.deepEqual(compileDeclarations(h).mentions, [
    { member: "rule:a", target: "rule:b" },
    { member: "rule:b", target: "rule:a" },
    { member: "rule:b", target: "rule:c" },
  ]);
});

test("satisfies rows are member-then-requirement sorted regardless of authoring order", () => {
 const h = harness({
 members: [
      rule({ name: "b", prose: text`# B`, satisfies: ["y", "x"] }),
      rule({ name: "a", prose: text`# A`, satisfies: ["z"] }),
 ],
    require: {
      x: { means: "x" },
      y: { means: "y" },
      z: { means: "z" },
 },
 });
  assert.deepEqual(compileDeclarations(h).satisfies, [
    { member: "a", requirement: "z" },
    { member: "b", requirement: "x" },
    { member: "b", requirement: "y" },
  ]);
});

test("needs derive the permission union, deduped and sorted, never authored twice", () => {
  const twoNeeds = harness({
 members: [
      rule({ name: "a", needs: [bash("git status"), bash("git diff")], prose: text`# A` }),
      rule({ name: "b", needs: [bash("git diff")], prose: text`# B` }),
 ],
 });
  const result = emit(twoNeeds);
  assert.deepEqual(result.permissions, ["Bash(git diff)", "Bash(git status)"]);
});

// ---------------------------------------------------------------------------
// Kinds in play — an embedded kind carries no locus-bearing kind fact and no
// standalone projection.
// ---------------------------------------------------------------------------

/** An embedded-locus kind, built via `kind()` directly. */
function embeddedKind<T extends object>(name: string, withinHosts: readonly string[]) {
  return kind<T>({
    name,
    locus: { kind: "embedded", withinHosts },
    unitShape: "file",
    registration: [{ via: "always" }],
  });
}

test("an embedded member neither projects nor takes a kind-fact row", () => {
  const decisionBlock = embeddedKind<Record<never, never>>("decision-block", ["spec"]);
  const mixed = harness({
 members: [
      rule({ name: "rust", prose: text`# Rust` }),
      decisionBlock({ name: "surface-authority" }),
 ],
 });
  const result = emit(mixed);
  // Only the rule projects — the embedded kind lives inside a host document.
  assert.deepEqual(result.members.map((m) => m.name), ["rust"]);
  // The declaration kinds carry the rule, never the embedded kind (residue inherits through the host).
  assert.deepEqual(result.declarations.kinds.map((k) => k.name), ["rule"]);
});

test("a host kind's fact row carries its declared embedded children as templates", () => {
  const decisionBlock = embeddedKind<Record<never, never>>("decision", ["rule"]);
  const mixed = harness({
    members: [rule({ name: "rust", prose: text`# Rust` }), decisionBlock({ name: "surface-authority" })],
 });
  const declarations = compileDeclarations(mixed);
  const ruleRow = declarations.kinds.find((k) => k.name === "rule")!;
  assert.deepEqual(ruleRow.templates, ["decision"]);
});

// ---------------------------------------------------------------------------
// Embedded-member values — the generic mechanism survives; no prescribed
// ontology ships. `decision`/`law`/`bound`/`Alternative` are gone —
// a corpus that wants them declares its own child kind with `embeddedMemberValue()`.
// ---------------------------------------------------------------------------

test("embeddedMemberValue() composes an author-declared child kind, no built-in ontology needed", () => {
  const value = embeddedMemberValue({
    kind: "ruling",
    key: "unship-prescribed-child-kinds",
    leaves: { statement: "the SDK ships no built-in child-kind ontology" },
    collections: { bounds: [{ key: "scope", leaves: { claim: "sdk/ only" } }] },
 });
  assert.deepEqual(value, {
    kind: "ruling",
    key: "unship-prescribed-child-kinds",
    leaves: { statement: "the SDK ships no built-in child-kind ontology" },
    collections: { bounds: [{ key: "scope", leaves: { claim: "sdk/ only" } }] },
 });
});

test("the prescribed child-kind constructors are gone from the SDK's exports", () => {
  const exports = sdk as Record<string, unknown>;
  for (const removed of ["decision", "law", "bound", "genre", "genreValue"]) {
    assert.equal(exports[removed], undefined, `${removed} should no longer be exported`);
 }
});

// ---------------------------------------------------------------------------
// Body resolution — `file()` assets read in, `text` mentions resolution-checked,
// `blocks()` refused until the fence format lands.
// ---------------------------------------------------------------------------

test("a file() body resolves byte-faithfully", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-file-"));
  try {
    const content = "# Long rule\n\nBody line one.\nBody line two.\n";
    writeFileSync(join(dir, "long.md"), content);
    const moduleUrl = pathToFileURL(join(dir, "mod.ts")).href;
    const h = harness({
      members: [rule({ name: "long", paths: ["src/**"], prose: file(moduleUrl, "./long.md") })],
    });

    const result = emit(h);
    assert.equal(result.members[0].body, content);
    // Byte-deterministic across a second emit over the unchanged asset.
    assert.equal(result.seam, emit(h).seam);
  } finally {
    rmSync(dir, { recursive: true, force: true });
 }
});

test("a file() body's payload member carries the resolved source path; text does not", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-file-"));
  try {
    writeFileSync(join(dir, "long.md"), "# Long rule\n");
    const moduleUrl = pathToFileURL(join(dir, "mod.ts")).href;
 const h = harness({
 members: [
        rule({ name: "long", prose: file(moduleUrl, "./long.md") }),
        rule({ name: "short", prose: text`# Short` }),
 ],
 });
    const result = emit(h);
    assert.equal(result.members.find((m) => m.name === "long")!.source_path, join(dir, "long.md"));
    assert.equal(result.members.find((m) => m.name === "short")!.source_path, undefined);
  } finally {
    rmSync(dir, { recursive: true, force: true });
 }
});

test("a missing file() asset is a loud emit error", () => {
  const moduleUrl = pathToFileURL(join(tmpdir(), "mod.ts")).href;
  const h = harness({ members: [rule({ name: "long", prose: file(moduleUrl, "./absent.md") })] });
  assert.throws(() => emit(h), /did not resolve/);
});

test("a resolved mention renders to its declared value's display form", () => {
  const citer = rule({
    name: "citations",
    prose: text`A ${{ address: "rule:rust", display: "rust" }} is declared.`,
 });
  const h = harness({ members: [rule({ name: "rust", prose: text`# Rust` }), citer] });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "citations")!;
  assert.match(member.body, /A rust is declared\./);
});

test("an unresolved mention is a loud emit error", () => {
  const citer = rule({
    name: "citations",
    prose: text`A ${{ address: "rule:ghost", display: "ghost" }} is declared.`,
 });
  assert.throws(() => emit(harness({ members: [citer] })), /a mention cannot dangle/);
});

test("a blocks() body renders an embedded member as a member.<kind> <key> TOML fence", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "surface-authority",
            leaves: { chosen: "the composition surface is canonical" },
          }),
        ),
      }),
    ],
  });
  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(
    member.body,
    '```member.decision surface-authority\nchosen = "the composition surface is canonical"\n```\n',
  );
});

test("a kind()'s render hook replaces the default TOML view inside the member fence; a kind() without one stays byte-identical", () => {
  const embeddedFacts = {
    locus: { kind: "embedded" as const, withinHosts: ["memory"] },
    unitShape: "file" as const,
    registration: [],
  };
  const decisionWithRender = kind<object>(
    { name: "decision", ...embeddedFacts },
    { render: (value) => `${value.key} chose: ${value.leaves.chosen}` },
  );
  const decisionWithoutRender = kind<object>({ name: "decision", ...embeddedFacts });

  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: decisionWithRender,
            key: "surface-authority",
            leaves: { chosen: "the composition surface is canonical" },
          }),
          embeddedMemberValue({
            kind: decisionWithoutRender,
            key: "second",
            leaves: { chosen: "unchanged" },
          }),
        ),
      }),
    ],
  });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(
    member.body,
    "```member.decision surface-authority\n" +
      "surface-authority chose: the composition surface is canonical\n" +
      "```\n" +
      "\n" +
      '```member.decision second\nchosen = "unchanged"\n```\n',
  );
});

test("a kind()'s render hook refuses on a dangling embedded-kind leaf mention, the same as the hook-less default TOML view", () => {
  const embeddedFacts = {
    locus: { kind: "embedded" as const, withinHosts: ["memory"] },
    unitShape: "file" as const,
    registration: [],
  };
  const decisionWithRender = kind<object>(
    { name: "decision", ...embeddedFacts },
    { render: (value) => `${value.key} chose: ${value.leaves.chosen}` },
  );

  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: decisionWithRender,
            key: "surface-authority",
            leaves: { chosen: text`See ${{ address: "rule:ghost", display: "ghost" }}.` },
          }),
        ),
      }),
    ],
  });

  assert.throws(() => emit(h), /a mention cannot dangle/);
});

test("a kind()'s render hook receives a resolvable leaf mention already rendered to a plain string, not a Text object", () => {
  const embeddedFacts = {
    locus: { kind: "embedded" as const, withinHosts: ["memory"] },
    unitShape: "file" as const,
    registration: [],
  };
  const decisionWithRender = kind<object>(
    { name: "decision", ...embeddedFacts },
    { render: (value) => `${value.key} chose: ${value.leaves.chosen}` },
  );

  const h = harness({
    members: [
      rule({ name: "rust", prose: text`# Rust` }),
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: decisionWithRender,
            key: "surface-authority",
            leaves: { chosen: text`See ${{ address: "rule:rust", display: "rust" }} for the standard.` },
          }),
        ),
      }),
    ],
  });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(
    member.body,
    "```member.decision surface-authority\n" +
      "surface-authority chose: See rust for the standard.\n" +
      "```\n",
  );
});

test("a blocks() body renders a keyed collection entry as its own [collection.entry] table", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "surface-authority",
            leaves: { chosen: "the composition surface is canonical" },
            collections: {
              rejected: [
                { key: "read-only-lens", leaves: { because: "you cannot compose a harness you only mirror" } },
                { key: "baked-projection", leaves: { because: "a stamping projector breaks law 5" } },
              ],
            },
          }),
        ),
      }),
    ],
  });
  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  // Authored out of alphabetical order (`read-only-lens` before `baked-projection`)
  // — the rendered fence preserves that authored order, not a re-sort.
  assert.equal(
    member.body,
    '```member.decision surface-authority\n' +
      'chosen = "the composition surface is canonical"\n' +
      "\n" +
      "[rejected.read-only-lens]\n" +
      'because = "you cannot compose a harness you only mirror"\n' +
      "\n" +
      "[rejected.baked-projection]\n" +
      'because = "a stamping projector breaks law 5"\n' +
      "```\n",
  );
});

test("multiple blocks() values render as sibling fences, and a leaf's quotes/newlines TOML-escape", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "one",
            leaves: { chosen: 'a "quoted" word\nand a new line' },
          }),
          embeddedMemberValue({ kind: "decision", key: "two", leaves: { chosen: "second" } }),
        ),
      }),
    ],
  });
  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(
    member.body,
    "```member.decision one\n" +
      'chosen = "a \\"quoted\\" word\\nand a new line"\n' +
      "```\n" +
      "\n" +
      "```member.decision two\n" +
      'chosen = "second"\n' +
      "```\n",
  );
});

test("an empty blocks() body renders no fences", () => {
  const h = harness({
    members: [memory({ name: "CLAUDE", prose: blocks() })],
  });
  const result = emit(h);
  assert.equal(result.members.find((m) => m.name === "CLAUDE")!.body, "\n");
});

test("a Text-valued leaf's mention resolves and renders inline — in the fence and the nested_member row", () => {
  const h = harness({
    members: [
      rule({ name: "rust", prose: text`# Rust` }),
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "surface-authority",
            leaves: { chosen: text`See ${{ address: "rule:rust", display: "rust" }} for the standard.` },
          }),
        ),
      }),
    ],
  });
  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(
    member.body,
    '```member.decision surface-authority\nchosen = "See rust for the standard."\n```\n',
  );
  assert.deepEqual(result.declarations.nested_members, [
    {
      host: "memory:CLAUDE",
      kind: "decision",
      key: "surface-authority",
      leaves: { chosen: "See rust for the standard." },
      collections: {},
    },
  ]);
});

test("an unresolved mention inside a leaf is a loud emit error, mirroring a member-level dangling mention", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "surface-authority",
            leaves: { chosen: text`See ${{ address: "rule:ghost", display: "ghost" }}.` },
          }),
        ),
      }),
    ],
  });
  assert.throws(() => emit(h), /a mention cannot dangle/);
});

test("a leaf's mention contributes a mention row keyed to the leaf's own structural address", () => {
  const h = harness({
    members: [
      rule({ name: "rust", prose: text`# Rust` }),
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "surface-authority",
            leaves: { chosen: text`See ${{ address: "rule:rust", display: "rust" }}.` },
            collections: {
              rejected: [
                {
                  key: "baked-projection",
                  leaves: { because: text`Breaks ${{ address: "rule:rust", display: "rust" }}.` },
                },
              ],
            },
          }),
        ),
      }),
    ],
  });
  assert.deepEqual(compileDeclarations(h).mentions, [
    { member: "CLAUDE/decision/surface-authority/chosen", target: "rule:rust" },
    { member: "CLAUDE/decision/surface-authority/rejected.baked-projection.because", target: "rule:rust" },
  ]);
});

test("a bare-string leaf is unchanged — no mention row, no resolution check", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "surface-authority",
            leaves: { chosen: "the composition surface is canonical" },
          }),
        ),
      }),
    ],
  });
  assert.deepEqual(compileDeclarations(h).mentions, []);
});

test("a blocks()-declared embedded member surfaces a matching nested_member row alongside its unchanged rendered fence", () => {
  // NESTED-MEMBER-LOCK-ROW (0018): the composed value feeds both the rendered fence
  // (untouched — `renderMemberFence`) and, additively, a `nested_member` declaration
  // row carrying the identical facts.
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: "decision",
            key: "surface-authority",
            leaves: { chosen: "the composition surface is canonical" },
            collections: {
              rejected: [
                { key: "read-only-lens", leaves: { because: "you cannot compose a harness you only mirror" } },
                { key: "baked-projection", leaves: { because: "a stamping projector breaks law 5" } },
              ],
            },
          }),
        ),
      }),
    ],
  });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  // Authored out of alphabetical order — both the rendered fence and the row
  // preserve it.
  assert.equal(
    member.body,
    '```member.decision surface-authority\n' +
      'chosen = "the composition surface is canonical"\n' +
      "\n" +
      "[rejected.read-only-lens]\n" +
      'because = "you cannot compose a harness you only mirror"\n' +
      "\n" +
      "[rejected.baked-projection]\n" +
      'because = "a stamping projector breaks law 5"\n' +
      "```\n",
  );

  assert.deepEqual(result.declarations.nested_members, [
    {
      host: "memory:CLAUDE",
      kind: "decision",
      key: "surface-authority",
      leaves: { chosen: "the composition surface is canonical" },
      collections: {
        rejected: [
          { key: "read-only-lens", leaves: { because: "you cannot compose a harness you only mirror" } },
          { key: "baked-projection", leaves: { because: "a stamping projector breaks law 5" } },
        ],
      },
    },
  ]);
});

// ---------------------------------------------------------------------------
// Memory — a frontmatterless kind carries an empty fields list.
// ---------------------------------------------------------------------------

test("a module-carried memory member carries no frontmatter fields", () => {
  const claudeBody = "# Project\n\nThe house rules Claude reads each session.\n";
 const h = harness({
 members: [
      memory({
        name: "CLAUDE",
        prose: text`
          # Project

          The house rules Claude reads each session.
        `,
      }),
 ],
 });

  const result = emit(h);
  assert.deepEqual(result.members.map((m) => m.name), ["CLAUDE"]);
  const member = result.members[0];
  assert.equal(member.body, claudeBody);
  assert.deepEqual(member.fields, []);
});
