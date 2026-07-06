/**
 * Emit — the six-noun face compiled to the seam's JSON pipe
 * (`specs/architecture/20-surface.md`, "Emit — total"; "The seam — one
 * implementation"). A harness authored in the face (`harness()`, `kind<T>()`,
 * clause values, `needs`, `file()`/`text`/`blocks()`) emits the declaration rows
 * (the lock's five families) and every projected member's erased payload — kind,
 * name, ordered fields, resolved body. The engine is the sole compiler of every
 * projection and the whole lock; the SDK writes neither.
 */

import assert from "node:assert/strict";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import {
  bash,
  blocks,
  clause,
  compileDeclarations,
  declarationsToJson,
  emit,
  file,
  genre,
  genreValue,
  harness,
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
  assert.equal(seam.version, 1);
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
// Declaration rows — the five families, and the `satisfies` join the roster/
// coverage tiers need (`specs/architecture/20-surface.md`, "The lock and drift").
// ---------------------------------------------------------------------------

function fullHarness() {
  return harness({
    members: [
      rule({
        name: "rust",
        paths: ["src/**/*.rs"],
        needs: [bash("git diff")],
        satisfies: ["dev-standards"],
        prose: text`# Rust`,
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
    reachability: "advisory",
  });
}

test("compileDeclarations produces all five families, satisfies included", () => {
  const declarations = compileDeclarations(fullHarness());

  assert.deepEqual(declarations.kinds, [
    {
      name: "rule",
      provider: undefined,
      governs_root: ".claude/rules",
      governs_glob: "*.md",
      format: "yaml-frontmatter",
      unit_shape: "file",
      activation: "paths-match(paths)",
    },
  ]);
  assert.deepEqual(declarations.clauses, [
    { kind: "rule", predicate: "max_lines", field: undefined, severity: "advisory" },
    { kind: "rule", predicate: "required", field: "paths", severity: "required" },
  ]);
  assert.deepEqual(declarations.requirements, [
    {
      name: "dev-standards",
      kind: "rule",
      required: true,
      verified_by: "tests/dev-standards.test.ts",
    },
  ]);
  assert.deepEqual(declarations.assembly, [
    { fact: "authority", value: "shared" },
    { fact: "reachability", value: "advisory" },
  ]);
  assert.deepEqual(declarations.satisfies, [{ member: "rust", requirement: "dev-standards" }]);
});

test("the JSON pipe carries the reduced declaration rows and the pinned version", () => {
  const seam = JSON.parse(declarationsToJson(compileDeclarations(fullHarness())));
  assert.equal(seam.version, 1);
  assert.deepEqual(seam.satisfies, [{ member: "rust", requirement: "dev-standards" }]);
});

test("a member with no satisfies claim contributes no row", () => {
  const h = harness({ members: [rule({ name: "rust", prose: text`# Rust` })] });
  assert.deepEqual(compileDeclarations(h).satisfies, []);
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
// Kinds in play — a genre carries no locus-bearing kind fact and no projection.
// ---------------------------------------------------------------------------

test("a genre member neither projects nor takes a kind-fact row", () => {
  const decisionBlock = genre<Record<never, never>>({ name: "decision-block", withinHosts: ["spec"] });
  const mixed = harness({
    members: [
      rule({ name: "rust", prose: text`# Rust` }),
      decisionBlock({ name: "surface-authority" }),
    ],
  });
  const result = emit(mixed);
  // Only the rule projects — the genre lives inside a host document.
  assert.deepEqual(result.members.map((m) => m.name), ["rust"]);
  // The declaration kinds carry the rule, never the genre (residue inherits through the host).
  assert.deepEqual(result.declarations.kinds.map((k) => k.name), ["rule"]);
});

// ---------------------------------------------------------------------------
// Genre values — the generic mechanism survives; no prescribed ontology ships
// (`specs/architecture/15-kinds.md`, "a genre is a full kind, and genre checks
// are data, never engine"). `decision`/`law`/`bound`/`Alternative` are gone —
// a corpus that wants them declares its own genre with `genreValue()`.
// ---------------------------------------------------------------------------

test("genreValue() composes an author-declared genre, no built-in ontology needed", () => {
  const value = genreValue({
    genre: "ruling",
    key: "unship-prescribed-genres",
    leaves: { statement: "the SDK ships no built-in genre ontology" },
    collections: { bounds: { scope: { claim: "sdk/ only" } } },
  });
  assert.deepEqual(value, {
    genre: "ruling",
    key: "unship-prescribed-genres",
    leaves: { statement: "the SDK ships no built-in genre ontology" },
    collections: { bounds: { scope: { claim: "sdk/ only" } } },
  });
});

test("a genreValue() reaches blocks() and still hits the pending fence-format gate", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(genreValue({ genre: "ruling", key: "x", leaves: { statement: "y" } })),
      }),
    ],
  });
  assert.throws(() => emit(h), /genre-fence-format/);
});

test("the prescribed genre constructors are gone from the SDK's exports", () => {
  const exports = sdk as Record<string, unknown>;
  for (const removed of ["decision", "law", "bound"]) {
    assert.equal(exports[removed], undefined, `${removed} should no longer be exported`);
  }
});

// ---------------------------------------------------------------------------
// Body resolution — `file()` assets read in, `text` mentions resolution-checked,
// `blocks()` refused until the fence format lands (`20-surface.md`, "Prose").
// ---------------------------------------------------------------------------

test("a file() body resolves byte-faithfully", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-file-"));
  try {
    const content = "# Long rule\n\nBody line one.\nBody line two.\n";
    writeFileSync(join(dir, "long.md"), content);
    const h = harness({ members: [rule({ name: "long", paths: ["src/**"], prose: file("./long.md") })] });

    const result = emit(h, { baseDir: dir });
    assert.equal(result.members[0].body, content);
    // Byte-deterministic across a second emit over the unchanged asset.
    assert.equal(result.seam, emit(h, { baseDir: dir }).seam);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("a file() body's payload member carries the resolved source path; text does not", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-file-"));
  try {
    writeFileSync(join(dir, "long.md"), "# Long rule\n");
    const h = harness({
      members: [
        rule({ name: "long", prose: file("./long.md") }),
        rule({ name: "short", prose: text`# Short` }),
      ],
    });
    const result = emit(h, { baseDir: dir });
    assert.equal(result.members.find((m) => m.name === "long")!.source_path, join(dir, "long.md"));
    assert.equal(result.members.find((m) => m.name === "short")!.source_path, undefined);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("a missing file() asset is a loud emit error", () => {
  const h = harness({ members: [rule({ name: "long", prose: file("./absent.md") })] });
  assert.throws(() => emit(h, { baseDir: tmpdir() }), /did not resolve/);
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

test("a blocks() body is refused until the genre fence format lands", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        // A blocks() body composes now; its render awaits (genre-fence-format).
        prose: { kind: "blocks", values: [] },
      }),
    ],
  });
  assert.throws(() => emit(h), /genre-fence-format/);
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
