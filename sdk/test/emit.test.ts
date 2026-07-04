/**
 * Byte-parity — the SDK emitter compiles to the exact inert manifest the Rust
 * `toml_edit` emitter produces (`specs/architecture/20-surface.md`,
 * "Content-faithful, deterministically emitted (law 5)"). Each manifest fixture
 * below is read from the **shared `contract/` corpus** — the one golden set both
 * implementations test against (`specs/architecture/50-distribution.md`,
 * acquisition Decision). The Rust suite (`tests/contract_fixtures.rs`) is the sole
 * producer of those goldens: it emits them through `write_manifest_members`
 * (`src/compose.rs`, the same path `temper emit` covers) and byte-matches them.
 * A `serializeManifestMember` that diverges by one byte fails here — key order,
 * table-vs-header choice, string escaping (basic / literal / multiline), blank-
 * line placement, and array spelling all pinned by the shared file.
 *
 * The goldens carry raw backslashes (a TOML literal string like `'path\to'`) and
 * a trailing newline (`toml_edit` terminates the last value line); reading them
 * from disk keeps them byte-exact with no `String.raw` transcription to drift.
 */

import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

import {
  decision,
  defineHarness,
  emit,
  emitManifestMembers,
  fromFile,
  lockRow,
  md,
  memory,
  projectBytes,
  projectionPath,
  projectMember,
  rule,
  serializeManifestMember,
  sha256Hex,
  skill,
  stampLock,
  toManifestMember,
  writeEmit,
} from "../src/index.js";
import type { ManifestMember } from "../src/index.js";

/**
 * The shared byte-parity goldens under `contract/manifest/` — resolved from this
 * test file's location so the read is CWD-independent. The Rust suite produces
 * them; this suite consumes the identical bytes (`specs/architecture/50-distribution.md`,
 * "both implementations tested against one golden set"). The compiled test runs from
 * `sdk/dist/test/`, so `contract/` at the repo root is three segments up.
 */
const CONTRACT_MANIFEST = new URL("../../../contract/manifest/", import.meta.url);
function golden(name: string): string {
  return readFileSync(new URL(name, CONTRACT_MANIFEST), "utf8");
}

// ---------------------------------------------------------------------------
// The shared fixture corpus — one hand-built ManifestMember per fixture, paired
// with the byte-exact Rust emitter output for that same member.
// ---------------------------------------------------------------------------

/** A rule: a string-list field, a fill edge, headings, and a multiline body. */
const RULE_MEMBER: ManifestMember = {
  kind: "claude-code.rule",
  name: "rust",
  line_count: 3,
  headings: ["Rust conventions"],
  satisfies: ["engineering-standards"],
  fields: { paths: ["src/**/*.rs"] },
  sections: [
    {
      heading: "Rust conventions",
      body: "# Rust conventions\n\nErrors via miette/thiserror; clippy clean under -D warnings.",
    },
  ],
  genres: [],
  published: [],
};

const RULE_TOML = golden("rule.toml");

/** A skill: a field whose value forces a multiline-basic string, plus a demand. */
const SKILL_MEMBER: ManifestMember = {
  kind: "claude-code.skill",
  name: "coordinate",
  line_count: 2,
  headings: ["Coordinate"],
  satisfies: [],
  fields: { description: 'Use when "coordinating" agents.\nSecond line.' },
  sections: [{ heading: "Coordinate", body: "Drive the team." }],
  genres: [],
  published: [{ name: "playbook", means: "a shared playbook exists", kind: "skill", required: true }],
};

const SKILL_TOML = golden("skill.toml");

/** A decision genre value: flat leaves and a keyed sibling collection. */
const DECISION_MEMBER: ManifestMember = {
  kind: "decision",
  name: "05-surface-authority",
  line_count: 1,
  headings: [],
  satisfies: [],
  fields: {},
  sections: [],
  genres: [
    {
      genre: "decision",
      key: "surface-authority",
      leaves: {
        because: "law 7 needs an authored surface",
        chosen: "the composition surface is canonical",
      },
      collections: {
        rejected: { "baked-projection": { because: "a stamping projector breaks law 5" } },
      },
    },
  ],
  published: [],
};

const DECISION_TOML = golden("decision.toml");

/** A memory member: a boolean field, a backslash string (Rust picks a literal
 * string), and a genre with no collections. */
const MEMORY_MEMBER: ManifestMember = {
  kind: "claude-code.memory",
  name: "root",
  line_count: 5,
  headings: [],
  satisfies: [],
  fields: { "disable-model-invocation": true, note: "path\\to" },
  sections: [],
  genres: [{ genre: "bound", key: "reachability", leaves: { claim: "the world is a node" }, collections: {} }],
  published: [],
};

const MEMORY_TOML = golden("memory.toml");

/** A decision whose `rejected` collection is declared but empty — the parent and
 * the collection header still emit, childless (the Rust `Table::new()` is not
 * implicit). */
const EMPTY_REJECTED_MEMBER: ManifestMember = {
  kind: "decision",
  name: "06-sole",
  line_count: 1,
  headings: [],
  satisfies: [],
  fields: {},
  sections: [],
  genres: [
    {
      genre: "decision",
      key: "sole",
      leaves: { chosen: "one option, no alternatives" },
      collections: { rejected: {} },
    },
  ],
  published: [],
};

const EMPTY_REJECTED_TOML = golden("empty-rejected.toml");

const CORPUS: readonly (readonly [string, ManifestMember, string])[] = [
  ["rule", RULE_MEMBER, RULE_TOML],
  ["skill", SKILL_MEMBER, SKILL_TOML],
  ["decision", DECISION_MEMBER, DECISION_TOML],
  ["memory", MEMORY_MEMBER, MEMORY_TOML],
  ["empty rejected collection", EMPTY_REJECTED_MEMBER, EMPTY_REJECTED_TOML],
];

for (const [label, member, expected] of CORPUS) {
  test(`byte-parity: ${label} serializes byte-identical to the Rust emitter`, () => {
    assert.equal(serializeManifestMember(member), expected);
  });

  test(`double-emit: ${label} is byte-stable`, () => {
    assert.equal(serializeManifestMember(member), serializeManifestMember(member));
  });
}

// The corpus, kind-then-name sorted exactly as `write_manifest_members` received
// it — byte-identical to the multi-member manifest root. The blank-line-before-
// every-header-but-the-first rule, verified across member boundaries.
const CORPUS_TOML = golden("corpus.toml");

test("byte-parity: the whole corpus matches the Rust multi-member manifest root", () => {
  // A member's block, standalone, has no leading blank; the document inserts one
  // blank line before every block but the first — so join-with-blank reconstructs
  // exactly what `write_manifest_members` emits over the sorted slice.
  const sorted = CORPUS.map(([, member]) => member).sort(
    (a, b) =>
      (a.kind < b.kind ? -1 : a.kind > b.kind ? 1 : 0) ||
      (a.name < b.name ? -1 : a.name > b.name ? 1 : 0),
  );
  assert.equal(sorted.map(serializeManifestMember).join("\n"), CORPUS_TOML);
});

// ---------------------------------------------------------------------------
// The authoring face end to end — emit sorts, maps every member through
// `toManifestMember`, and lands on the byte-parity serializer.
// ---------------------------------------------------------------------------

const surfaceAuthority = decision({
  key: "surface-authority",
  chosen: "the assembly declares the posture; enforcement reads it",
  rejected: {
    "baked-projection": { because: "bakes a stance law 2 says the author owns" },
  },
});

function harness() {
  return defineHarness({
    // The requirement the rust rule below fills — declared so its `satisfies`
    // resolves (emit refuses a dangling join, `refuseBrokenSource`).
    requirements: {
      "engineering-standards": {
        means: "the repo carries a rule fixing the engineering bar",
        kind: "claude-code.rule",
      },
    },
    members: [
      rule({
        name: "rust",
        fields: { paths: ["src/**/*.rs"] },
        satisfies: {
          "engineering-standards": { rationale: "carries the Rust conventions" },
        },
        body: md`
          # Rust conventions

          Errors via miette/thiserror; clippy clean under -D warnings.
        `,
        genres: [surfaceAuthority],
      }),
    ],
  });
}

test("emit lands every member on the byte-parity serializer", () => {
  const toml = emitManifestMembers(harness());
  // The entry point equals the serializer over the same sorted, mapped members —
  // so the byte-parity the fixtures pin holds through `emit`, not just the seam.
  const sorted = [...harness().members]
    .sort(
      (a, b) =>
        (a.kind < b.kind ? -1 : a.kind > b.kind ? 1 : 0) ||
        (a.name < b.name ? -1 : a.name > b.name ? 1 : 0),
    )
    .map((member) => toManifestMember(member));
  assert.equal(toml, sorted.map(serializeManifestMember).join("\n"));
  // And it carries the expected schema.
  assert.match(toml, /^\[\[member\]\]$/m);
  assert.match(toml, /kind = "claude-code\.rule"/);
  assert.match(toml, /satisfies = \["engineering-standards"\]/);
  assert.match(toml, /^\[\[member\.section\]\]$/m);
  assert.match(toml, /^\[member\.genre\.collections\.rejected\.baked-projection\]$/m);
  assert.doesNotMatch(toml, /rejected\.0/);
});

test("double-emit over a harness is byte-stable", () => {
  assert.equal(emitManifestMembers(harness()), emitManifestMembers(harness()));
});

test("the leaf address rides structure: member + genre key + field path", () => {
  const manifest = toManifestMember(harness().members[0]);
  const value = manifest.genres[0];
  assert.equal(value.key, "surface-authority");
  assert.equal(value.collections.rejected["baked-projection"].because.length > 0, true);
});

// ---------------------------------------------------------------------------
// Body resolution — `fromFile` assets read in, mentions resolution-checked and
// rendered by the display rule (`specs/architecture/20-surface.md`, "Mentions").
// Resolution happens at emit against the whole harness's declared values.
// ---------------------------------------------------------------------------

test("a fromFile body resolves, emits, and stays byte-parity", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-fromfile-"));
  try {
    // A trailing newline forces the same multiline-basic spelling the Rust
    // emitter picks for an asset read from disk — the parity the fixture pins.
    const content = "# Long rule\n\nBody line one.\nBody line two.\n";
    writeFileSync(join(dir, "long.md"), content);
    const member = rule({ name: "long", body: fromFile("./long.md") });
    const harness = defineHarness({ members: [member] });

    const toml = emitManifestMembers(harness, { baseDir: dir });
    // The asset is the section body byte-for-byte — same bytes an inline member
    // carrying that content would land on the serializer.
    const expected: ManifestMember = {
      kind: "claude-code.rule",
      name: "long",
      line_count: content.split("\n").length,
      headings: ["Long rule"],
      satisfies: [],
      fields: {},
      sections: [{ heading: "Long rule", body: content }],
      genres: [],
      published: [],
    };
    assert.equal(toml, serializeManifestMember(expected));
    // Byte-deterministic: a second emit over the unchanged asset is identical.
    assert.equal(toml, emitManifestMembers(harness, { baseDir: dir }));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("a missing fromFile asset is a loud emit error", () => {
  const harness = defineHarness({
    members: [rule({ name: "long", body: fromFile("./absent.md") })],
  });
  assert.throws(() => emitManifestMembers(harness, { baseDir: tmpdir() }), /did not resolve/);
});

test("a resolved mention renders to its declared value's display form", () => {
  const target = rule({ name: "rust", body: md`# Rust` });
  const citer = rule({
    name: "citations",
    body: md`A ${{ address: "claude-code.rule:rust", display: "rust" }} is declared.`,
  });
  const harness = defineHarness({ members: [target, citer] });

  const toml = emitManifestMembers(harness);
  // The display rule substituted the target's form; the surrounding words and
  // spacing are the author's, and no interpolation marker survives (law 5).
  assert.match(toml, /body = "A rust is declared\."/);
  assert.doesNotMatch(toml, /\u0000/);

  // The rendered member is byte-identical to the same words authored inline.
  const rendered = serializeManifestMember(
    toManifestMember(citer, { mentionable: new Set(["claude-code.rule:rust"]) }),
  );
  const inline: ManifestMember = {
    kind: "claude-code.rule",
    name: "citations",
    line_count: 1,
    headings: [],
    satisfies: [],
    fields: {},
    sections: [{ heading: "citations", body: "A rust is declared." }],
    genres: [],
    published: [],
  };
  assert.equal(rendered, serializeManifestMember(inline));
  // Byte-deterministic across a second emit.
  assert.equal(toml, emitManifestMembers(harness));
});

test("an unresolved mention is a loud emit error", () => {
  const citer = rule({
    name: "citations",
    body: md`A ${{ address: "claude-code.rule:ghost", display: "ghost" }} is declared.`,
  });
  assert.throws(
    () => emitManifestMembers(defineHarness({ members: [citer] })),
    /a mention cannot dangle/,
  );
});

// ---------------------------------------------------------------------------
// Projection byte-parity — the SDK projector compiles a member to its `.claude/**`
// harness file byte-identical to the Rust emit projector (`src/drift.rs`
// `project_bytes`). Each fixture below is **known-good Rust output**, captured by
// running `temper import` + `temper emit` over the identical member and reading
// the projected file byte-for-byte (`specs/architecture/20-surface.md`, law 5).
// The paired `emit_hash` is the fingerprint that same Rust `emit` stamped into the
// lock — `sha256` of the projected bytes — so the SDK's stamped fingerprint must
// equal it for the lock to agree cross-tool.
// ---------------------------------------------------------------------------

/** A rule with a `paths` field: a `---`-frontmatter block over the body. */
const RULE_PROJECTION_MEMBER: ManifestMember = {
  kind: "claude-code.rule",
  name: "rust",
  line_count: 4,
  headings: ["Rust conventions"],
  satisfies: [],
  fields: { paths: ["src/**/*.rs"] },
  sections: [
    {
      heading: "Rust conventions",
      body: "# Rust conventions\n\nErrors via miette/thiserror; clippy clean under -D warnings.\n",
    },
  ],
  genres: [],
  published: [],
};

const RULE_PROJECTION = String.raw`---
paths: ["src/**/*.rs"]
---
# Rust conventions

Errors via miette/thiserror; clippy clean under -D warnings.
`;
const RULE_EMIT_HASH = "f3c0876a42c602e8d6b65775cb7346312a6f963ea48a904d7d4b1b8181586b86";

/** A skill: frontmatter carries `name` then `description`, in that declared order. */
const SKILL_PROJECTION_MEMBER: ManifestMember = {
  kind: "claude-code.skill",
  name: "coordinate",
  line_count: 3,
  headings: ["Coordinate"],
  satisfies: [],
  fields: {
    name: "coordinate",
    description: "Use when driving a complex task across a team of agents.",
  },
  sections: [{ heading: "Coordinate", body: "# Coordinate\n\nDrive the team.\n" }],
  genres: [],
  published: [],
};

const SKILL_PROJECTION = String.raw`---
name: "coordinate"
description: "Use when driving a complex task across a team of agents."
---
# Coordinate

Drive the team.
`;
const SKILL_EMIT_HASH = "8d9d761634c1fc9f3705e5095ed8c97a5bcb64b20bbfb08c05d65c7f75374d6a";

/** A fieldless rule projects to its body alone — no frontmatter block. */
const PLAIN_PROJECTION_MEMBER: ManifestMember = {
  kind: "claude-code.rule",
  name: "plain",
  line_count: 3,
  headings: ["Plain rule"],
  satisfies: [],
  fields: {},
  sections: [{ heading: "Plain rule", body: "# Plain rule\n\nNo frontmatter here.\n" }],
  genres: [],
  published: [],
};

const PLAIN_PROJECTION = "# Plain rule\n\nNo frontmatter here.\n";
const PLAIN_EMIT_HASH = "7a2c466d149cadddec9ca7a2669a659877a7eac785eecb4a80bc643c12346e33";

const PROJECTION_CORPUS: readonly (readonly [string, ManifestMember, string, string, string])[] = [
  ["rule", RULE_PROJECTION_MEMBER, RULE_PROJECTION, RULE_EMIT_HASH, ".claude/rules/rust.md"],
  [
    "skill",
    SKILL_PROJECTION_MEMBER,
    SKILL_PROJECTION,
    SKILL_EMIT_HASH,
    ".claude/skills/coordinate/SKILL.md",
  ],
  ["fieldless rule", PLAIN_PROJECTION_MEMBER, PLAIN_PROJECTION, PLAIN_EMIT_HASH, ".claude/rules/plain.md"],
];

for (const [label, member, expected, emitHash, expectedPath] of PROJECTION_CORPUS) {
  test(`projection byte-parity: ${label} projects byte-identical to the Rust projector`, () => {
    const projection = projectMember(member);
    assert.equal(projection.path, expectedPath);
    assert.equal(projection.bytes, expected);
  });

  test(`lock fingerprint agreement: ${label}'s SDK emit_hash matches the Rust emitter`, () => {
    const projection = projectMember(member);
    // The Rust `emit` stamped `sha256(projection)` as the lock's emit_hash; the SDK
    // hashes the same bytes and must land on the same fingerprint.
    assert.equal(sha256Hex(projection.bytes), emitHash);

    // A module-carried member's projection is its own state-of-record, so a fresh
    // stamp records source_hash == emit_hash == sha256(projection) — the baseline a
    // Rust import-then-emit lands on for a byte-identical projection.
    const row = lockRow(member.kind, projection);
    assert.equal(row.emitHash, emitHash);
    assert.equal(row.sourceHash, emitHash);
    assert.equal(row.sourcePath, expectedPath);
  });
}

test("projectionPath maps the built-in projected kinds; others are a loud error", () => {
  assert.equal(projectionPath("claude-code.rule", "rust"), ".claude/rules/rust.md");
  assert.equal(projectionPath("claude-code.skill", "coordinate"), ".claude/skills/coordinate/SKILL.md");
  // A memory projects a root `<name>.md` — `CLAUDE.md`, `AGENTS.md`.
  assert.equal(projectionPath("claude-code.memory", "CLAUDE"), "CLAUDE.md");
  assert.equal(projectionPath("agents-md.memory", "AGENTS"), "AGENTS.md");
  // A bare kind name resolves the same way — the locus keys on the last dotted segment.
  assert.equal(projectionPath("rule", "x"), ".claude/rules/x.md");
  // A custom kind carries no projection — a loud error, never a guessed path.
  assert.throws(() => projectionPath("acme.widget", "gadget"), /no projection/);
});

test("projectBytes: fieldless body-only, and null fields drop from the frontmatter", () => {
  // A dropped-null field with no surviving field is body-only, no `---` block.
  assert.equal(projectBytes([["paths", null]], "# Body\n"), "# Body\n");
  // A surviving field forces the block; the null one is omitted.
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
// Lock stamping — the `[[<kind>]]` roll-up (`src/import.rs` `write_rollup`): kinds
// sorted, rows name-sorted, four columns in fixed order, one blank line before
// every table header but the first (the `toml_edit` layout the manifest shares).
// ---------------------------------------------------------------------------

test("stampLock lays out `[[<kind>]]` rows the toml_edit way", () => {
  const rustRow = lockRow("claude-code.rule", projectMember(RULE_PROJECTION_MEMBER));
  const plainRow = lockRow("claude-code.rule", projectMember(PLAIN_PROJECTION_MEMBER));
  const skillRow = lockRow("claude-code.skill", projectMember(SKILL_PROJECTION_MEMBER));

  // Passed skill-first, rule-out-of-order — stampLock sorts kinds (rule < skill) and
  // rows within a kind (plain < rust), the deterministic order the Rust lock carries.
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
// The full emit — manifest + projection + lock in one deterministic pass over the
// authoring face (`specs/architecture/20-surface.md`, "Topology": the three
// provenance classes emit produces).
// ---------------------------------------------------------------------------

function projectedHarness() {
  return defineHarness({
    members: [
      rule({
        name: "rust",
        fields: { paths: ["src/**/*.rs"] },
        body: md`
          # Rust conventions

          Errors via miette/thiserror; clippy clean under -D warnings.
        `,
      }),
      skill({
        name: "coordinate",
        fields: { description: "Use when driving a complex task across a team of agents." },
        body: md`
          # Coordinate

          Drive the team.
        `,
      }),
    ],
  });
}

test("emit compiles manifest + projection + lock in one pass", () => {
  const result = emit(projectedHarness());

  // The manifest equals the standalone manifest emitter over the same harness.
  assert.equal(result.manifest, emitManifestMembers(projectedHarness()));

  // One projection per projected member, at its `.claude/**` locus, each with a
  // lock row whose fingerprint is sha256 of the very bytes projected.
  const paths = result.projections.map((p) => p.path);
  assert.deepEqual(paths, [".claude/rules/rust.md", ".claude/skills/coordinate/SKILL.md"]);
  for (const projection of result.projections) {
    assert.match(result.lock, new RegExp(`emit_hash = "${sha256Hex(projection.bytes)}"`));
  }

  // The lock carries a `[[rule]]` and a `[[skill]]` row; source_hash == emit_hash.
  assert.match(result.lock, /^\[\[rule\]\]$/m);
  assert.match(result.lock, /^\[\[skill\]\]$/m);
});

test("emit is byte-stable across a double pass", () => {
  const a = emit(projectedHarness());
  const b = emit(projectedHarness());
  assert.equal(a.manifest, b.manifest);
  assert.equal(a.lock, b.lock);
  assert.deepEqual(
    a.projections.map((p) => `${p.path} ${p.bytes}`),
    b.projections.map((p) => `${p.path} ${p.bytes}`),
  );
});

test("writeEmit lands the manifest, lock, and projection on disk", () => {
  const dir = mkdtempSync(join(tmpdir(), "temper-emit-"));
  try {
    const result = writeEmit(projectedHarness(), dir);

    assert.equal(readFileSync(join(dir, "temper.toml"), "utf8"), result.manifest);
    assert.equal(readFileSync(join(dir, "lock.toml"), "utf8"), result.lock);
    for (const projection of result.projections) {
      assert.equal(readFileSync(join(dir, projection.path), "utf8"), projection.bytes);
    }
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

// ---------------------------------------------------------------------------
// Memory projection — a module-carried memory (`CLAUDE.md` / `AGENTS.md`) has no
// frontmatter (`kinds/claude-code/memory/KIND.md`, "There is **no frontmatter**"),
// so the whole file is the body and it projects body-alone to the root `<name>.md`
// with a lock row. SDK-ahead under TS-primary: the Rust projector carries no memory
// locus, so this pins the body-alone contract, not a cross-tool byte-parity fixture.
// ---------------------------------------------------------------------------

test("a module-carried memory projects its frontmatterless body to CLAUDE.md", () => {
  const claudeBody = "# Project\n\nThe house rules Claude reads each session.\n";
  const harness = defineHarness({
    members: [memory({ name: "CLAUDE", body: md`
      # Project

      The house rules Claude reads each session.
    ` })],
  });

  const result = emit(harness);

  // One projection, at the root `CLAUDE.md` locus, whose bytes are the body alone —
  // no `---` frontmatter block, the memory kind declaring none.
  assert.deepEqual(result.projections.map((p) => p.path), ["CLAUDE.md"]);
  const projection = result.projections[0];
  assert.equal(projection.bytes, claudeBody);
  assert.doesNotMatch(projection.bytes, /^---$/m);

  // A lock row keyed on that path, both fingerprints sha256 of the body-alone bytes.
  const hash = sha256Hex(claudeBody);
  assert.match(result.lock, /^\[\[memory\]\]$/m);
  assert.match(result.lock, /source_path = "CLAUDE\.md"/);
  assert.match(result.lock, new RegExp(`source_hash = "${hash}"`));
  assert.match(result.lock, new RegExp(`emit_hash = "${hash}"`));

  // The seam agrees: projectMember over the manifest member lands the same file.
  const manifestMember = toManifestMember(harness.members[0]);
  const direct = projectMember(manifestMember);
  assert.equal(direct.path, "CLAUDE.md");
  assert.equal(direct.bytes, claudeBody);
  assert.equal(lockRow(manifestMember.kind, direct).sourcePath, "CLAUDE.md");
});

test("emit stays double-emit stable with a memory alongside rule and skill", () => {
  function mixedHarness() {
    return defineHarness({
      members: [
        rule({
          name: "rust",
          fields: { paths: ["src/**/*.rs"] },
          body: md`
            # Rust conventions

            Errors via miette/thiserror; clippy clean under -D warnings.
          `,
        }),
        skill({
          name: "coordinate",
          fields: { description: "Use when driving a complex task across a team of agents." },
          body: md`
            # Coordinate

            Drive the team.
          `,
        }),
        memory({ name: "CLAUDE", body: md`
          # Project

          The house rules Claude reads each session.
        ` }),
      ],
    });
  }

  const a = emit(mixedHarness());
  const b = emit(mixedHarness());
  assert.equal(a.manifest, b.manifest);
  assert.equal(a.lock, b.lock);
  assert.deepEqual(
    a.projections.map((p) => `${p.path} ${p.bytes}`),
    b.projections.map((p) => `${p.path} ${p.bytes}`),
  );
  // The memory rides in alongside the rule/skill projections — three loci, one each.
  assert.deepEqual(a.projections.map((p) => p.path).sort(), [
    ".claude/rules/rust.md",
    ".claude/skills/coordinate/SKILL.md",
    "CLAUDE.md",
  ]);
});
