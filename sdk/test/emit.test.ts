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
  embeddedMemberValue,
  emit,
  file,
  forbiddenKeys,
  harness,
  include,
  kind,
  maxLen,
  maxLines,
  renderText,
  required,
  text,
} from "../src/index.js";
import * as sdk from "../src/index.js";
import type { ResolvedEmbeddedMemberValue } from "../src/index.js";
import { compileDeclarations } from "../src/declarations.js";
import { agent, hook, mcpServer, memory, rule, skill } from "../src/claude-code.js";

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

test("a skill and an agent project the grown frontmatter fields, identity first then authored order", () => {
  // The generic fold already carries any typed field; this pins the newly-typed
  // Skill/Agent surface — a sample of the documented optional keys projects in the
  // author's declared order, byte-faithfully, behind the identity field.
  const h = harness({
    members: [
      skill({
        name: "deploy",
        description: "Deploy the service.",
        when_to_use: "when shipping a release",
        "allowed-tools": ["Bash(git *)"],
        model: "inherit",
        effort: "high",
        context: "fork",
        prose: text`# Deploy`,
      }),
      agent({
        name: "reviewer",
        description: "Review a diff.",
        tools: ["Read", "Grep"],
        model: "haiku",
        permissionMode: "plan",
        maxTurns: 12,
        color: "blue",
        prose: text`# Reviewer`,
      }),
    ],
  });

  const result = emit(h);

  assert.deepEqual(result.members.find((m) => m.name === "deploy")!.fields, [
    ["name", "deploy"],
    ["description", "Deploy the service."],
    ["when_to_use", "when shipping a release"],
    ["allowed-tools", ["Bash(git *)"]],
    ["model", "inherit"],
    ["effort", "high"],
    ["context", "fork"],
  ]);
  assert.deepEqual(result.members.find((m) => m.name === "reviewer")!.fields, [
    ["name", "reviewer"],
    ["description", "Review a diff."],
    ["tools", ["Read", "Grep"]],
    ["model", "haiku"],
    ["permissionMode", "plan"],
    ["maxTurns", 12],
    ["color", "blue"],
  ]);
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
        prose: "the harness maintains development standards",
        kind: rule,
 required: true,
        verifiedBy: "tests/dev-standards.test.ts",
 },
 },
 });
}

test("compileDeclarations produces all eight families, satisfies and mentions included", () => {
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
      content: undefined,
      shape: undefined,
      collection_address: undefined,
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
      range: undefined,
      section: undefined,
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
      range: undefined,
      section: undefined,
 },
  ]);
  assert.deepEqual(declarations.requirements, [
 {
      name: "dev-standards",
      kind: "rule",
 required: true,
      clauses: [],
      verified_by: "tests/dev-standards.test.ts",
      prose: "the harness maintains development standards",
 },
  ]);
  assert.deepEqual(declarations.assembly, [{ fact: "mode", value: "warn" }]);
  assert.deepEqual(declarations.satisfies, [{ member: "rule:rust", requirement: "dev-standards" }]);
  assert.deepEqual(declarations.mentions, [{ member: "rule:rust", target: "dev-standards" }]);
  // No member declares a composed-prose include in this harness.
  assert.deepEqual(declarations.includes, []);
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

test("the JSON pipe carries the declaration rows under `declarations` and the pinned version", () => {
  const seam = JSON.parse(emit(fullHarness()).seam);
  assert.equal(seam.version, 2);
  assert.deepEqual(seam.declarations.satisfies, [{ member: "rule:rust", requirement: "dev-standards" }]);
  assert.deepEqual(seam.declarations.mentions, [{ member: "rule:rust", target: "dev-standards" }]);
});

test("a member with no satisfies claim contributes no row", () => {
  const h = harness({ members: [rule({ name: "rust", prose: text`# Rust` })] });
  assert.deepEqual(compileDeclarations(h).satisfies, []);
});

test("a member with no mentions in its prose contributes no mention row", () => {
  const h = harness({ members: [rule({ name: "rust", prose: text`# Rust` })] });
  assert.deepEqual(compileDeclarations(h).mentions, []);
});

test("a composed-prose include contributes an include row with the module-resolved target path", () => {
  const moduleUrl = pathToFileURL("/repo/.temper/rules/rust.ts").href;
  const h = harness({
    members: [
      rule({ name: "rust", prose: text`Intro.\n${include(moduleUrl, "./fragment.md")}\nOutro.` }),
    ],
  });
  assert.deepEqual(compileDeclarations(h).includes, [
    { member: "rule:rust", source_path: "/repo/.temper/rules/fragment.md" },
  ]);
});

test("an include leaves its slot in the rendered body for the engine to splice, mentions still resolving", () => {
  const prose = text`Intro. ${{ address: "rule:a", display: "a" }} ${include("file:///m.ts", "./f.md")} tail.`;
  // The mention becomes its display; the include stays a U+0001 slot the engine fills.
  assert.equal(renderText(prose), "Intro. a " + "\u0001" + " tail.");
  assert.equal(prose.includes.length, 1);
  assert.equal(prose.mentions.length, 1);
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
      x: { prose: "x" },
      y: { prose: "y" },
      z: { prose: "z" },
 },
 });
  assert.deepEqual(compileDeclarations(h).satisfies, [
    { member: "rule:a", requirement: "z" },
    { member: "rule:b", requirement: "x" },
    { member: "rule:b", requirement: "y" },
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

/** An embedded-locus kind, built via `kind()` directly — host-free, as every embedded kind is. */
function embeddedKind<T extends object>(name: string) {
  return kind<T>({
    name,
    locus: { kind: "embedded" },
    unitShape: "file",
    registration: [{ via: "always" }],
  });
}

/**
 * The `decision` embedded kind the `blocks()` cases nest under a `memory` host, plus the
 * admission each harness declares to let it compose one (`emit` refuses a value the
 * corpus never admitted over the host kind).
 */
const memoryDecision = embeddedKind<Record<never, never>>("decision");
const admitDecision = { host: memory, admits: [memoryDecision] };

test("an embedded member neither projects nor takes a kind-fact row", () => {
  const decisionBlock = embeddedKind<Record<never, never>>("decision-block");
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

test("a host kind's fact row carries the embedded kinds the corpus admits over it as templates", () => {
  const decisionBlock = embeddedKind<Record<never, never>>("decision");
  const mixed = harness({
    members: [rule({ name: "rust", prose: text`# Rust` }), decisionBlock({ name: "surface-authority" })],
    admit: [{ host: rule, admits: [decisionBlock] }],
 });
  const declarations = compileDeclarations(mixed);
  const ruleRow = declarations.kinds.find((k) => k.name === "rule")!;
  assert.deepEqual(ruleRow.templates, [{ kind: "decision" }]);
  // The same kind, admitted nowhere else, leaves every other host's column absent.
  assert.equal(declarations.kinds.find((k) => k.name === "memory")?.templates, undefined);
});

/**
 * A nested-file `supporting-doc` kind — the child a host templates at a path pattern. It
 * governs no glob of its own: the pattern is its host's declared fact, so it can never
 * contend with the host's own locus for a document's position.
 */
const supportingDoc = kind<Record<never, never>>({
  name: "supporting-doc",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [{ via: "always" }],
});

/** A `guide` host declaring one file-child layer: `supporting-doc` units at `*.md`,
 * relative to the host's own unit. */
const guide = kind<Record<never, never>>({
  name: "guide",
  locus: { kind: "at", root: ".claude/guides", glob: "GUIDE.md" },
  format: "yaml-frontmatter",
  unitShape: "directory",
  identityField: "name",
  registration: [{ via: "always" }],
  templates: [{ kind: supportingDoc, path: "*.md" }],
});

test("a host's file child emits at its host's unit joined with the template's pattern", () => {
  const gate = guide({ name: "operate-the-gate" });
  const result = emit(
    harness({ members: [gate, supportingDoc({ name: "checklist", host: gate, prose: text`# Checklist` })] }),
  );

  // The child owns a file — its path composed from the host's unit and the `*.md` pattern
  // the host declares, never a glob of its own.
  const child = result.members.find((m) => m.kind === "supporting-doc")!;
  assert.equal(child.host, "guide:operate-the-gate");
  assert.equal(child.body, "# Checklist");

  // The child kind takes a fact row (the engine places its file off one) carrying no
  // governs pair, and no locus it could share with its `guide` host.
  const childRow = result.declarations.kinds.find((k) => k.name === "supporting-doc")!;
  assert.equal(childRow.governs_root, undefined);
  assert.equal(childRow.governs_glob, undefined);
  assert.deepEqual(
    result.declarations.kinds.find((k) => k.name === "guide")!.templates,
    [{ kind: "supporting-doc", path: "*.md" }],
  );
});

test("a nested file child names its host, and every other locus names none", () => {
  const gate = guide({ name: "operate-the-gate" });
  assert.throws(
    () => supportingDoc({ name: "checklist" }),
    /composes from its host's unit, so it names the `host` member/,
  );
  assert.throws(() => rule({ name: "rust", host: gate, prose: text`# Rust` }), /its path composes from nobody/);
});

test("a kind's declared file-child template reaches its fact row with no admission in play", () => {
  const h = harness({ members: [guide({ name: "operate-the-gate" })] });
  const guideRow = compileDeclarations(h).kinds.find((k) => k.name === "guide")!;

  // The template is a kind-side fact: no `admit` declaration exists, and a file child
  // could not be admitted anyway — admission is over an embedded body.
  assert.deepEqual(guideRow.templates, [{ kind: "supporting-doc", path: "*.md" }]);
});

test("an admission over a host overrides the child kind its own template declares", () => {
  const richDoc = embeddedKind<Record<never, never>>("rich-doc");
  const h = harness({
    members: [guide({ name: "operate-the-gate" })],
    admit: [{ host: guide, admits: [richDoc] }],
 });
  const guideRow = compileDeclarations(h).kinds.find((k) => k.name === "guide")!;

  // The adopting corpus wants richer typing than the declared child kind, so its
  // admission wins. The admitted kind is embedded by construction, so the overriding
  // layer carries no path — the declared `*.md` pattern goes with the kind it addressed.
  assert.deepEqual(guideRow.templates, [{ kind: "rich-doc" }]);
});

test("a built-in kind's composed body admits a corpus-declared embedded kind", () => {
  // The built-in `skill` names no child kind — admission is the adopting corpus's
  // declaration, so a locally-declared embedded kind composes a shipped kind's body.
  const rubric = embeddedKind<Record<never, never>>("rubric");
  const h = harness({
    members: [
      skill({
        name: "operate-the-gate",
        description: "Use when operating the gate.",
        prose: blocks(
          embeddedMemberValue({ kind: rubric, key: "green-bar", leaves: { check: "every gate passes" } }),
        ),
      }),
    ],
    admit: [{ host: skill, admits: [rubric] }],
 });
  const result = emit(h);
  const member = result.members.find((m) => m.name === "operate-the-gate")!;
  assert.equal(member.body, '```member.rubric green-bar\ncheck = "every gate passes"\n```\n');
  assert.deepEqual(compileDeclarations(h).kinds.find((k) => k.name === "skill")!.templates, [{ kind: "rubric" }]);
  assert.deepEqual(result.declarations.nested_members, [
    { host: "skill:operate-the-gate", kind: "rubric", key: "green-bar", leaves: { check: "every gate passes" }, collections: {} },
  ]);
});

test("a kind's declared layout lowers into its content row; a file-content kind omits it", () => {
  // A `layout`-content kind exercising all three primitives: an importing prose region,
  // a field section filling a named slot, and a member collection of a named kind.
  const specDoc = kind<Record<never, never>>({
    name: "spec",
    locus: { kind: "at", root: "specs", glob: "*.md" },
    unitShape: "file",
    registration: [{ via: "always" }],
    content: {
      regions: [
        { region: "prose", import: "specs/intent.md" },
        { region: "field", slot: "intent" },
        { region: "collection", memberKind: "invariant" },
      ],
    },
  });
  const h = harness({
    members: [
      specDoc({ name: "representation", prose: text`# Representation` }),
      rule({ name: "rust", prose: text`# Rust` }),
    ],
  });
  const declarations = compileDeclarations(h);

  // The layout lowers verbatim, `memberKind` spelled as the wire's `member_kind`.
  const specRow = declarations.kinds.find((k) => k.name === "spec")!;
  assert.deepEqual(specRow.content, {
    regions: [
      { region: "prose", import: "specs/intent.md" },
      { region: "field", slot: "intent" },
      { region: "collection", member_kind: "invariant", key: undefined },
    ],
  });

  // The file-content rule declares no layout, so its row omits the column entirely.
  const ruleRow = declarations.kinds.find((k) => k.name === "rule")!;
  assert.equal(ruleRow.content, undefined);
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

test("the names the root must not carry are absent from the SDK's exports", () => {
  const exports = sdk as Record<string, unknown>;
  // The prescribed child-kind constructors, plus the seam's value half: the payload is
  // internal, so its compiler and its version reach a caller only through `emit()`.
  for (const removed of ["decision", "law", "bound", "genre", "genreValue", "compileDeclarations", "SEAM_VERSION"]) {
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

test("an unresolved mention naming no declared kind is a loud emit error", () => {
  // `widget` is nowhere declared in this harness, so the mention names no discovery
  // locus to defer to — a dangling refusal at emit. (A mention naming a *declared* kind
  // with no composed member defers instead; see `mention.test.ts`.)
  const citer = rule({
    name: "citations",
    prose: text`A ${{ address: "widget:ghost", display: "ghost" }} is declared.`,
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
    admit: [admitDecision],
  });
  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(
    member.body,
    '```member.decision surface-authority\nchosen = "the composition surface is canonical"\n```\n',
  );
});

test("a kind()'s render hook projects fence-free in place of the default TOML view; a kind() without one keeps its member fence byte-identical", () => {
  const embeddedFacts = {
    locus: { kind: "embedded" as const },
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
    admit: [{ host: memory, admits: [decisionWithRender, decisionWithoutRender] }],
  });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  // The render-hook value projects its hook markdown bare — no `member.decision`
  // fence around it — while the hook-less value keeps its fenced TOML view.
  assert.equal(
    member.body,
    "surface-authority chose: the composition surface is canonical\n" +
      "\n" +
      '```member.decision second\nchosen = "unchanged"\n```\n',
  );
});

test("a kind()'s render hook refuses on a dangling embedded-kind leaf mention, the same as the hook-less default TOML view", () => {
  const embeddedFacts = {
    locus: { kind: "embedded" as const },
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
    admit: [{ host: memory, admits: [decisionWithRender] }],
  });

  assert.throws(() => emit(h), /a mention cannot dangle/);
});

test("a kind()'s render hook receives a resolvable leaf mention already rendered to a plain string, not a Text object", () => {
  const embeddedFacts = {
    locus: { kind: "embedded" as const },
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
    admit: [{ host: memory, admits: [decisionWithRender] }],
  });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "CLAUDE")!;
  assert.equal(member.body, "surface-authority chose: See rust for the standard.\n");
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
    admit: [admitDecision],
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
    admit: [admitDecision],
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
    admit: [admitDecision],
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
    admit: [admitDecision],
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
    admit: [admitDecision],
  });
  assert.deepEqual(compileDeclarations(h).mentions, []);
});

test("a blocks()-declared embedded member surfaces a matching nested_member row alongside its unchanged rendered fence", () => {
  // The composed value feeds both the rendered fence and, additively, a
  // `nested_member` declaration row carrying the identical facts.
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
    admit: [admitDecision],
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

/**
 * An embedded kind whose `source` field is an edge to a `rule`, and whose render hook
 * spells the reference off the derived target facts alone — the instance authors an
 * address and never a word of the rendering.
 */
function citationKind() {
  return kind<object>(
    {
      name: "citation",
      locus: { kind: "embedded" },
      unitShape: "file",
      registration: [],
      edgeFields: [{ field: "source", to: "rule" }],
    },
    {
      render: (value) => {
        const target = value.targets.source;
        return `See [${target.name}](${target.path}) — the ${target.kind} at \`${target.address}\`.`;
      },
    },
  );
}

test("an embedded format spells an edge target off the derived facts — name, address, kind, and a path relative to each host's own projection", () => {
  const citation = citationKind();
  // One authored value, two hosts whose projections land at different depths: the
  // rendered path differs per host, so it is derived from the host's own locus rather
  // than baked into the instance.
  const cite = () =>
    embeddedMemberValue({ kind: citation, key: "the-standard", leaves: { source: "rule:rust" } });

  const h = harness({
    members: [
      rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust conventions` }),
      memory({ name: "CLAUDE", prose: blocks(cite()) }),
      skill({
        name: "coordinate",
        description: "Use when driving a complex task across a team of agents.",
        prose: blocks(cite()),
      }),
    ],
    admit: [
      { host: memory, admits: [citation] },
      { host: skill, admits: [citation] },
    ],
  });

  const result = emit(h);
  // `CLAUDE.md` sits at the root, so the target's projection is the path itself.
  assert.equal(
    result.members.find((m) => m.name === "CLAUDE")!.body,
    "See [rust](.claude/rules/rust.md) — the rule at `rule:rust`.\n",
  );
  // `.claude/skills/coordinate/SKILL.md` sits two directories down from `.claude/`.
  assert.equal(
    result.members.find((m) => m.name === "coordinate")!.body,
    "See [rust](../../rules/rust.md) — the rule at `rule:rust`.\n",
  );
});

test("an edge field's leaf still rides the nested_member row as the authored address, never the rendered reference", () => {
  const citation = citationKind();
  const h = harness({
    members: [
      rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust conventions` }),
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({ kind: citation, key: "the-standard", leaves: { source: "rule:rust" } }),
        ),
      }),
    ],
    admit: [{ host: memory, admits: [citation] }],
  });

  assert.deepEqual(compileDeclarations(h).nested_members, [
    {
      host: "memory:CLAUDE",
      kind: "citation",
      key: "the-standard",
      leaves: { source: "rule:rust" },
      collections: {},
    },
  ]);
});

/**
 * The host for one citation-shaped value, whose render hook is the test's variable —
 * what the format does or does not place is the whole subject.
 */
function citationHarness(
  render: (value: ResolvedEmbeddedMemberValue) => string,
  leaves: Record<string, string> = { source: "rule:rust", note: "the bar" },
) {
  const citation = kind<object>(
    {
      name: "citation",
      locus: { kind: "embedded" },
      unitShape: "file",
      registration: [],
      edgeFields: [{ field: "source", to: "rule" }],
    },
    { render },
  );
  return harness({
    members: [
      rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust conventions` }),
      memory({
        name: "CLAUDE",
        prose: blocks(embeddedMemberValue({ kind: citation, key: "the-standard", leaves })),
      }),
    ],
    admit: [{ host: memory, admits: [citation] }],
  });
}

test("an embedded kind's declared edge field writes its assembly `edge` fact, though the kind itself takes no kind-fact row", () => {
  const { declarations } = emit(citationHarness((value) => `See \`${value.leaves.source}\`.`));

  // The two halves one kind list once conflated: an embedded kind is filtered out of the
  // kind-fact rows (it reaches the lock through its host's `templates` column alone) and
  // must NOT be filtered out of the assembly edge facts (an edge is a declared
  // relationship at any grain). Pinned apart so a future filter cannot re-merge them and
  // silently strand every embedded edge at zero.
  assert.deepEqual(
    declarations.assembly.filter((fact) => fact.fact === "edge"),
    [{ fact: "edge", from: "citation", field: "source", to: "rule" }],
  );
  assert.ok(
    !declarations.kinds.some((row) => row.name === "citation"),
    "an embedded kind still takes no kind-fact row",
  );
});

test("emit records which declared edges an embedded format placed — the fact the engine cannot observe for itself", () => {
  const placing = emit(citationHarness((value) => `See [${value.targets.source.name}](${value.targets.source.path}).`));
  assert.deepEqual(placing.declarations.nested_members[0].placed_edges, ["source"]);

  // The same edge placed off its address leaf rather than the derived facts still
  // represents the reference, so it is still placed.
  const viaLeaf = emit(citationHarness((value) => `See \`${value.leaves.source}\`.`));
  assert.deepEqual(viaLeaf.declarations.nested_members[0].placed_edges, ["source"]);

  // A format that renders only the value's prose never names the edge at all — the case
  // the clause exists for. `[]`, never absent: a format ran and placed nothing.
  const omitting = emit(citationHarness((value) => value.leaves.note));
  assert.deepEqual(omitting.declarations.nested_members[0].placed_edges, []);
});

test("a value leaving a declared edge field unfilled records no placement — an absent edge is nothing to omit", () => {
  const unfilled = emit(citationHarness((value) => value.leaves.note, { note: "the bar" }));
  assert.deepEqual(unfilled.declarations.nested_members[0].placed_edges, undefined);

  // The obligation ranges over what the value fills, so the same format placing nothing
  // does record an omission once the edge is actually carried.
  const filled = emit(citationHarness((value) => value.leaves.note));
  assert.deepEqual(filled.declarations.nested_members[0].placed_edges, []);
});

test("a value whose kind declares no edge field records no placement — an ordinary nested_member row is unchanged", () => {
  const passage = kind<object>(
    { name: "passage", locus: { kind: "embedded" }, unitShape: "file", registration: [] },
    { render: (value) => value.leaves.body },
  );
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(embeddedMemberValue({ kind: passage, key: "intro", leaves: { body: "Words." } })),
      }),
    ],
    admit: [{ host: memory, admits: [passage] }],
  });

  assert.deepEqual(emit(h).declarations.nested_members, [
    { host: "memory:CLAUDE", kind: "passage", key: "intro", leaves: { body: "Words." }, collections: {} },
  ]);
});

test("a composed body interleaves prose spans and embedded values in authored order, byte-identical to a wrapper-kind narrative", () => {
  // A passage-style wrapper: an embedded kind whose render hook projects a leaf
  // verbatim, fence-free — the exhibit the native interleave retires.
  const passage = kind<object>(
    { name: "passage", locus: { kind: "embedded" }, unitShape: "file", registration: [] },
    { render: (value) => value.leaves.body },
  );
  const decisionValue = embeddedMemberValue({
    kind: "decision",
    key: "surface-authority",
    leaves: { chosen: "the composition surface is canonical" },
  });

  // The narrative rides natively — text spans interleaved with the embedded value.
  const native = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          text`Intro narrative before the record.`,
          decisionValue,
          text`Closing narrative after the record.`,
        ),
      }),
    ],
    admit: [admitDecision],
  });

  // The same narrative carried by wrapper passages minted to hold it.
  const wrapped = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({ kind: passage, key: "intro", leaves: { body: "Intro narrative before the record." } }),
          decisionValue,
          embeddedMemberValue({ kind: passage, key: "outro", leaves: { body: "Closing narrative after the record." } }),
        ),
      }),
    ],
    admit: [{ host: memory, admits: [memoryDecision, passage] }],
  });

  const nativeBody = emit(native).members.find((m) => m.name === "CLAUDE")!.body;
  const wrappedBody = emit(wrapped).members.find((m) => m.name === "CLAUDE")!.body;

  assert.equal(
    nativeBody,
    "Intro narrative before the record.\n" +
      "\n" +
      '```member.decision surface-authority\nchosen = "the composition surface is canonical"\n```\n' +
      "\n" +
      "Closing narrative after the record.\n",
  );
  // The narrative needs no wrapper kind to ride in a composed body.
  assert.equal(nativeBody, wrappedBody);

  // A prose span mints no nested member — only the embedded value does. The wrapper
  // variant, by contrast, mints one nested member per passage plus the decision.
  assert.deepEqual(
    emit(native).declarations.nested_members.map((row) => `${row.kind}:${row.key}`),
    ["decision:surface-authority"],
  );
  assert.deepEqual(
    emit(wrapped).declarations.nested_members.map((row) => `${row.kind}:${row.key}`),
    ["decision:surface-authority", "passage:intro", "passage:outro"],
  );
});

// ---------------------------------------------------------------------------
// Registration members — a fields-only hook/mcp-server erases into a manifest
// write fact, never a standalone projection.
// ---------------------------------------------------------------------------

test("a hook and an mcp-server member each erase into a registration write fact — name-keyed at their collection address, fields folded", () => {
  const h = harness({
    members: [
      hook({ name: "SessionStart", type: "command", command: "temper reporter", timeout: 5 }),
      mcpServer({ name: "gmail", type: "stdio", command: "npx", args: ["gmail-mcp"] }),
    ],
  });

  const result = emit(h);

  // Neither surfaces as a standalone projection — a fields-only registration member
  // owns no artifact of its own, so it never rides the projected-member payload.
  assert.deepEqual(result.members, []);

  // Each erases into a write fact carrying its name (key), collection address, and
  // folded fields — the shape a manifest write face reads back. Kind-then-key sorted,
  // so `hook` precedes `mcp-server`.
  assert.deepEqual(result.registrations, [
    {
      kind: "hook",
      key: "SessionStart",
      collectionAddress: { manifest: "settings.json", keyPath: "hooks.<Event>" },
      fields: [
        ["type", "command"],
        ["command", "temper reporter"],
        ["timeout", 5],
      ],
    },
    {
      kind: "mcp-server",
      key: "gmail",
      collectionAddress: { manifest: ".mcp.json", keyPath: "mcpServers.*" },
      fields: [
        ["type", "stdio"],
        ["command", "npx"],
        ["args", ["gmail-mcp"]],
      ],
    },
  ]);
});

test("the assembly's residual settings erase into settings.json residue rows, key-sorted, carried beside the hooks segment", () => {
  const h = harness({
    members: [hook({ name: "SessionStart", type: "command", command: "temper reporter" })],
    settings: { worktree: true, autoMemoryEnabled: false },
  });

  const result = emit(h);

  // Each authored settings key erases into a `settings.json` residue row — key-sorted, so
  // `autoMemoryEnabled` precedes `worktree` regardless of authoring order.
  assert.deepEqual(result.settings, [
    { manifest: "settings.json", key: "autoMemoryEnabled", value: false },
    { manifest: "settings.json", key: "worktree", value: true },
  ]);

  // The same rows ride the seam's `settings` declaration family — the one source the
  // `EmitResult` sibling also maps from, so they cannot disagree.
  const seam = JSON.parse(result.seam);
  assert.deepEqual(seam.declarations.settings, [
    { manifest: "settings.json", key: "autoMemoryEnabled", value: false },
    { manifest: "settings.json", key: "worktree", value: true },
  ]);

  // The hook still erases into its own registration write fact — the two families are
  // carried side by side, never one at the other's expense.
  assert.deepEqual(
    result.registrations.map((r) => r.key),
    ["SessionStart"],
  );
});

test("a harness with no residual settings carries an empty settings family", () => {
  const result = emit(projectedHarness());
  assert.deepEqual(result.settings, []);
  assert.deepEqual(JSON.parse(result.seam).declarations.settings, []);
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
