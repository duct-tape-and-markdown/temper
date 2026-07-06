/**
 * Emit — the six-noun face compiled to the committed seam
 * (`specs/architecture/20-surface.md`, "Emit — total"; "The seam — one
 * implementation"). A harness authored in the face (`harness()`, `kind<T>()`,
 * clause values, `needs`, `file()`/`text`/`blocks()`) emits three things: the
 * declaration rows the engine reads (the lock's `[declaration]` families + the
 * internal JSON pipe), a byte-faithful `.claude/**` projection, and the lock.
 *
 * The projection byte fixtures are known-good Rust projector output, captured by
 * running `temper import` + `temper emit` over the identical member; the paired
 * `emit_hash` is the fingerprint that Rust `emit` stamped, so the SDK's stamped
 * fingerprint must equal it for the lock to agree cross-tool. The declaration-row
 * bytes match the Rust lock's `[declaration]` shape (`src/drift.rs`) — the
 * byte-parity lockstep two writers keep until single-writer lands.
 */

import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { test } from "node:test";

import {
  bash,
  clause,
  compileDeclarations,
  declarationsToJson,
  emit,
  file,
  genre,
  harness,
  lockRow,
  maxLines,
  placementLines,
  projectBytes,
  projectionPath,
  projectMember,
  required,
  sha256Hex,
  stampLock,
  text,
  writeEmit,
} from "../src/index.js";
import type { ProjectionInput } from "../src/index.js";
import { memory, rule, skill } from "../src/claude-code.js";

// ---------------------------------------------------------------------------
// Projection byte-parity — the SDK projector compiles a member to its `.claude/**`
// file byte-identical to the Rust emit projector (`src/drift.rs` `project_bytes`).
// ---------------------------------------------------------------------------

const RULE_PROJECTION = `---
paths: ["src/**/*.rs"]
---
# Rust conventions

Errors via miette/thiserror; clippy clean under -D warnings.
`;
const RULE_EMIT_HASH = "f3c0876a42c602e8d6b65775cb7346312a6f963ea48a904d7d4b1b8181586b86";

const SKILL_PROJECTION = `---
name: "coordinate"
description: "Use when driving a complex task across a team of agents."
---
# Coordinate

Drive the team.
`;
const SKILL_EMIT_HASH = "8d9d761634c1fc9f3705e5095ed8c97a5bcb64b20bbfb08c05d65c7f75374d6a";

const PLAIN_PROJECTION = "# Plain rule\n\nNo frontmatter here.\n";
const PLAIN_EMIT_HASH = "7a2c466d149cadddec9ca7a2669a659877a7eac785eecb4a80bc643c12346e33";

/** A member built via the face, plus the resolved projection input emit hands the projector. */
function ruleInput(): ProjectionInput {
  const member = rule({
    name: "rust",
    paths: ["src/**/*.rs"],
    prose: text`
      # Rust conventions

      Errors via miette/thiserror; clippy clean under -D warnings.
    `,
  });
  return { facts: member.facts, name: member.name, fields: member.fields, body: "# Rust conventions\n\nErrors via miette/thiserror; clippy clean under -D warnings.\n" };
}

function skillInput(): ProjectionInput {
  const member = skill({
    name: "coordinate",
    description: "Use when driving a complex task across a team of agents.",
    prose: text`# Coordinate\n\nDrive the team.\n`,
  });
  return { facts: member.facts, name: member.name, fields: member.fields, body: "# Coordinate\n\nDrive the team.\n" };
}

function plainInput(): ProjectionInput {
  const member = rule({ name: "plain", prose: text`# Plain rule\n\nNo frontmatter here.\n` });
  return { facts: member.facts, name: member.name, fields: member.fields, body: "# Plain rule\n\nNo frontmatter here.\n" };
}

const PROJECTION_CORPUS: ReadonlyArray<
  readonly [string, () => ProjectionInput, string, string, string]
> = [
  ["rule", ruleInput, RULE_PROJECTION, RULE_EMIT_HASH, ".claude/rules/rust.md"],
  ["skill", skillInput, SKILL_PROJECTION, SKILL_EMIT_HASH, ".claude/skills/coordinate/SKILL.md"],
  ["fieldless rule", plainInput, PLAIN_PROJECTION, PLAIN_EMIT_HASH, ".claude/rules/plain.md"],
];

for (const [label, build, expected, emitHash, expectedPath] of PROJECTION_CORPUS) {
  test(`projection byte-parity: ${label} projects byte-identical to the Rust projector`, () => {
    const projection = projectMember(build());
    assert.equal(projection.path, expectedPath);
    assert.equal(projection.bytes, expected);
  });

  test(`lock fingerprint agreement: ${label}'s SDK emit_hash matches the Rust emitter`, () => {
    const projection = projectMember(build());
    assert.equal(sha256Hex(projection.bytes), emitHash);
    const row = lockRow(build().facts.name, projection);
    assert.equal(row.emitHash, emitHash);
    assert.equal(row.sourceHash, emitHash);
    assert.equal(row.sourcePath, expectedPath);
  });
}

test("projectionPath maps the built-in projected kinds; a genre is a loud error", () => {
  assert.equal(projectionPath(rule.facts, "rust"), ".claude/rules/rust.md");
  assert.equal(projectionPath(skill.facts, "coordinate"), ".claude/skills/coordinate/SKILL.md");
  assert.equal(projectionPath(memory.facts, "CLAUDE"), "CLAUDE.md");
  // A genre member lives inside a host document — no standalone projection.
  const spec = genre<Record<never, never>>({ name: "decision-block", withinHosts: ["spec"] });
  assert.throws(() => projectionPath(spec.facts, "x"), /is a genre/);
});

test("projectBytes: fieldless body-only, and null fields drop from the frontmatter", () => {
  assert.equal(projectBytes([["paths", null]], "# Body\n"), "# Body\n");
  assert.equal(
    projectBytes(
      [
        ["name", "x"],
        ["paths", null],
      ],
      "# Body\n",
    ),
    '---\nname: "x"\n---\n# Body\n',
  );
});

// ---------------------------------------------------------------------------
// The full emit — projection + lock + declaration rows + JSON pipe + permissions,
// one deterministic pass over the six-noun face.
// ---------------------------------------------------------------------------

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

test("emit compiles projection + lock + declarations + seam in one pass", () => {
  const result = emit(projectedHarness());

  const paths = result.projections.map((p) => p.path);
  assert.deepEqual(paths, [".claude/rules/rust.md", ".claude/skills/coordinate/SKILL.md"]);
  for (const projection of result.projections) {
    assert.match(result.lock, new RegExp(`emit_hash = "${sha256Hex(projection.bytes)}"`));
  }

  // The lock carries both rollup rows and the declaration families.
  assert.match(result.lock, /^\[\[rule\]\]$/m);
  assert.match(result.lock, /^\[\[skill\]\]$/m);
  assert.match(result.lock, /^\[\[declaration\.kind\]\]$/m);
  assert.match(result.lock, /^\[\[declaration\.assembly\]\]$/m);

  // The JSON pipe carries the same declarations, versioned.
  const seam = JSON.parse(result.seam);
  assert.equal(seam.version, 1);
  assert.deepEqual(
    seam.kinds.map((k: { name: string }) => k.name),
    ["rule", "skill"],
  );
});

test("emit is byte-stable across a double pass", () => {
  const a = emit(projectedHarness());
  const b = emit(projectedHarness());
  assert.equal(a.lock, b.lock);
  assert.equal(a.seam, b.seam);
  assert.deepEqual(
    a.projections.map((p) => `${p.path} ${p.bytes}`),
    b.projections.map((p) => `${p.path} ${p.bytes}`),
  );
});

test("writeEmit lands the lock and projections on disk; the JSON pipe is not written", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-emit-"));
  try {
    const result = writeEmit(projectedHarness(), dir);
    assert.equal(readFileSync(join(dir, "lock.toml"), "utf8"), result.lock);
    for (const projection of result.projections) {
      assert.equal(readFileSync(join(dir, projection.path), "utf8"), projection.bytes);
    }
    // The seam is in-flight, not a committed artifact.
    assert.throws(() => readFileSync(join(dir, "seam.json"), "utf8"));
    // The retired manifest/roster/bindings dialect is gone — no temper.toml written.
    assert.throws(() => readFileSync(join(dir, "temper.toml"), "utf8"));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

// ---------------------------------------------------------------------------
// Declaration rows — the four families, byte-identical to the Rust lock's
// `[declaration]` shape (`src/drift.rs`), so the two writers stay in lockstep.
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

const EXPECTED_DECLARATIONS = `[[declaration.kind]]
name = "rule"
governs_root = ".claude/rules"
governs_glob = "*.md"
format = "yaml-frontmatter"
unit_shape = "file"
activation = "paths-match(paths)"

[[declaration.clause]]
kind = "rule"
predicate = "max_lines"
severity = "advisory"

[[declaration.clause]]
kind = "rule"
predicate = "required"
field = "paths"
severity = "required"

[[declaration.requirement]]
name = "dev-standards"
kind = "rule"
required = true
verified_by = "tests/dev-standards.test.ts"

[[declaration.assembly]]
fact = "authority"
value = "shared"

[[declaration.assembly]]
fact = "reachability"
value = "advisory"
`;

test("declaration rows serialize to the Rust lock's `[declaration]` byte shape", () => {
  // Declaration-only (no rollup) isolates the four-family byte shape.
  assert.equal(stampLock([], compileDeclarations(fullHarness())), EXPECTED_DECLARATIONS);
});

test("the full emit lock carries the rollup rows then the declaration families", () => {
  const result = emit(fullHarness());
  // The rollup row precedes the declaration table, joined the toml_edit way.
  const rollup = `[[rule]]\nname = "rust"\n`;
  assert.ok(result.lock.startsWith(rollup), "the rollup row leads the lock");
  assert.ok(result.lock.endsWith(EXPECTED_DECLARATIONS), "the declaration families close the lock");
  // Exactly one blank line joins the rollup's last row to the first declaration header.
  assert.match(result.lock, /emit_hash = "[0-9a-f]{64}"\n\n\[\[declaration\.kind\]\]/);
});

test("the JSON pipe carries the reduced declaration rows and the pinned version", () => {
  const declarations = compileDeclarations(fullHarness());
  const seam = JSON.parse(declarationsToJson(declarations));
  assert.equal(seam.version, 1);
  assert.deepEqual(seam.clauses, [
    { kind: "rule", predicate: "max_lines", severity: "advisory" },
    { kind: "rule", predicate: "required", field: "paths", severity: "required" },
  ]);
  assert.deepEqual(seam.requirements, [
    { name: "dev-standards", kind: "rule", required: true, verified_by: "tests/dev-standards.test.ts" },
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
  assert.deepEqual(result.projections.map((p) => p.path), [".claude/rules/rust.md"]);
  // The declaration kinds carry the rule, never the genre (residue inherits through the host).
  assert.deepEqual(result.declarations.kinds.map((k) => k.name), ["rule"]);
});

// ---------------------------------------------------------------------------
// Body resolution — `file()` assets read in, `text` mentions resolution-checked,
// `blocks()` refused until the fence format lands (`20-surface.md`, "Prose").
// ---------------------------------------------------------------------------

test("a file() body resolves and projects byte-faithfully", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-file-"));
  try {
    const content = "# Long rule\n\nBody line one.\nBody line two.\n";
    writeFileSync(join(dir, "long.md"), content);
    const h = harness({ members: [rule({ name: "long", paths: ["src/**"], prose: file("./long.md") })] });

    const result = emit(h, { baseDir: dir });
    const projection = result.projections[0];
    assert.equal(projection.path, ".claude/rules/long.md");
    assert.equal(projection.bytes, `---\npaths: ["src/**"]\n---\n${content}`);
    // Byte-deterministic across a second emit over the unchanged asset.
    assert.equal(result.lock, emit(h, { baseDir: dir }).lock);
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
  const projection = result.projections.find((p) => p.path === ".claude/rules/citations.md")!;
  assert.match(projection.bytes, /A rust is declared\./);
  assert.doesNotMatch(projection.bytes, / /);
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
// Memory projection — a frontmatterless body projected to the root `CLAUDE.md`.
// ---------------------------------------------------------------------------

test("a module-carried memory projects its frontmatterless body to CLAUDE.md", () => {
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
  assert.deepEqual(result.projections.map((p) => p.path), ["CLAUDE.md"]);
  const projection = result.projections[0];
  assert.equal(projection.bytes, claudeBody);
  assert.doesNotMatch(projection.bytes, /^---$/m);

  const hash = sha256Hex(claudeBody);
  assert.match(result.lock, /^\[\[memory\]\]$/m);
  assert.match(result.lock, /source_path = "CLAUDE\.md"/);
  assert.match(result.lock, new RegExp(`emit_hash = "${hash}"`));
});

// ---------------------------------------------------------------------------
// Lock rollup layout — `[[<kind>]]` rows, kinds then rows name-sorted, four
// columns in fixed order, one blank line before every table header but the first.
// ---------------------------------------------------------------------------

test("stampLock lays out `[[<kind>]]` rollup rows the toml_edit way", () => {
  const rustRow = lockRow("rule", projectMember(ruleInput()));
  const plainRow = lockRow("rule", projectMember(plainInput()));
  const skillRow = lockRow("skill", projectMember(skillInput()));

  const lock = stampLock([skillRow, rustRow, plainRow]);
  const expected =
    `[[rule]]\n` +
    `name = "plain"\n` +
    `source_path = ".claude/rules/plain.md"\n` +
    `source_hash = "${PLAIN_EMIT_HASH}"\n` +
    `emit_hash = "${PLAIN_EMIT_HASH}"\n` +
    `\n` +
    `[[rule]]\n` +
    `name = "rust"\n` +
    `source_path = ".claude/rules/rust.md"\n` +
    `source_hash = "${RULE_EMIT_HASH}"\n` +
    `emit_hash = "${RULE_EMIT_HASH}"\n` +
    `\n` +
    `[[skill]]\n` +
    `name = "coordinate"\n` +
    `source_path = ".claude/skills/coordinate/SKILL.md"\n` +
    `source_hash = "${SKILL_EMIT_HASH}"\n` +
    `emit_hash = "${SKILL_EMIT_HASH}"\n`;
  assert.equal(lock, expected);
});

// ---------------------------------------------------------------------------
// Placement round-through — install's frontmatter placements (schema modeline +
// managed-by note) ride the whole-file re-emit (`20-surface.md`, the two-projectors
// seam). Those lines ride `install`, never `emit`.
// ---------------------------------------------------------------------------

const MODELINE = "# yaml-language-server: $schema=../../.temper/schema/rule.json";
const NOTE =
  "# temper: managed projection — a direct edit here is drift; edit the owning " +
  ".temper/ module or document and re-run temper emit, never this generated file.";
const RULE_PROJECTION_WITH_PLACEMENTS =
  `---\n${MODELINE}\n${NOTE}\npaths: ["src/**/*.rs"]\n---\n` +
  "# Rust conventions\n\nErrors via miette/thiserror; clippy clean under -D warnings.\n";

test("placementLines reads install's modeline + note in on-disk order; nothing else", () => {
  assert.deepEqual(placementLines(RULE_PROJECTION_WITH_PLACEMENTS), [MODELINE, NOTE]);
  assert.deepEqual(placementLines(PLAIN_PROJECTION), []);
  assert.deepEqual(placementLines(RULE_PROJECTION), []);
});

test("projectMember rounds a committed projection's placement lines through the re-emit", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-placements-"));
  try {
    const path = join(dir, ".claude/rules/rust.md");
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, RULE_PROJECTION_WITH_PLACEMENTS);

    const preserved = projectMember(ruleInput(), { projectionDir: dir });
    assert.equal(preserved.bytes, RULE_PROJECTION_WITH_PLACEMENTS);

    // Without the committed read, the projection is the placement-free baseline.
    assert.equal(projectMember(ruleInput()).bytes, RULE_PROJECTION);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("writeEmit re-emit preserves install's placements the second time round", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-emit-placements-"));
  try {
    writeEmit(projectedHarness(), dir);
    const rulePath = join(dir, ".claude/rules/rust.md");
    assert.equal(readFileSync(rulePath, "utf8"), RULE_PROJECTION);

    // Simulate `install`: splice the modeline + note atop the rule's frontmatter.
    writeFileSync(rulePath, RULE_PROJECTION_WITH_PLACEMENTS);

    const result = writeEmit(projectedHarness(), dir);
    assert.equal(readFileSync(rulePath, "utf8"), RULE_PROJECTION_WITH_PLACEMENTS);
    assert.match(result.lock, new RegExp(`emit_hash = "${sha256Hex(RULE_PROJECTION_WITH_PLACEMENTS)}"`));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
