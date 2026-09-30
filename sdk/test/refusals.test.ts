/**
 * Declare-side emit refusals — a broken source yields no output, never silent
 * bytes.
 * The compile catches a `satisfies` claim that names no declared requirement (a
 * dangling join) before it writes a byte. It does **not** gate fill: whether
 * every `required` requirement has a satisfier is the engine's requirement
 * clause, which sees a layout host's edge-slot fills the SDK never reads — a
 * required requirement with no composed satisfier must not refuse SDK-side, or a
 * layout-fill corpus would refuse spuriously. A clean harness emits without
 * throwing.
 *
 * Mention refusals live in emit.test.ts ("an unresolved mention is a loud emit
 * error"); this file owns only the declare-side cases.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import type { ResolvedEmbeddedMemberValue } from "../src/index.js";
import { blocks, embeddedMemberValue, emit, file, harness, kind, text } from "../src/index.js";
import { hook, memory, rule, skill } from "../src/claude-code.js";

/** An embedded-locus kind named `decision` — host-free; the corpus's `admit` names its hosts. */
function decisionKind() {
  return kind<object>({
    name: "decision",
    locus: { kind: "embedded" },
    unitShape: "file",
    registration: [],
  });
}

// ---------------------------------------------------------------------------
// (1) Dangling join — a `satisfies` claim resolving to no declared requirement.
// ---------------------------------------------------------------------------

test("emit refuses a satisfies claim naming no declared requirement", () => {
  const h = harness({
    members: [
      // No requirement — assembly-level or member-published — carries this name.
      rule({ name: "rust", prose: text`# Rust`, satisfies: ["ghost-requirement"] }),
    ],
  });
  assert.throws(() => emit(h), /a dangling join/);
});

test("a satisfies claim filling a member-published requirement is a live join", () => {
  const h = harness({
    members: [
      skill({
        name: "operate-the-gate",
        description: "Use when operating the gate.",
        prose: text`# Operate the gate`,
        requires: { playbook: { prose: "a shared gate playbook exists", kind: rule } },
      }),
      rule({ name: "gate-playbook", prose: text`# Gate playbook`, satisfies: ["playbook"] }),
    ],
  });
  // The far end is a member-published requirement — still a declared requirement,
  // so the join resolves and emit produces output.
  assert.doesNotThrow(() => emit(h));
});

test("a satisfies claim filling a requirement typed to a required-field kind is a live join", () => {
  // `skill` (unlike `rule`) declares required fields — a requirement typed to it
  // exercises `KindDefinition<never>`'s contravariant assignability, not just the
  // no-required-fields case `rule` happens to cover.
  const h = harness({
    members: [
      rule({
        name: "gate-playbook",
        prose: text`# Gate playbook`,
        requires: { runner: { prose: "a skill runs the gate playbook", kind: skill } },
      }),
      skill({
        name: "operate-the-gate",
        description: "Use when operating the gate.",
        prose: text`# Operate the gate`,
        satisfies: ["runner"],
      }),
    ],
  });
  assert.doesNotThrow(() => emit(h));
});

test("an expect binding keyed to a required-field kind emits without throwing", () => {
  // `ExpectBinding.kind` exercises the same contravariant assignability as
  // `Requirement.kind` above — `skill` declares required fields, so binding
  // `expect` to it (rather than a no-required-fields kind like `rule`) is the
  // case `KindDefinition<never>` must accept.
  const h = harness({
    members: [
      skill({
        name: "operate-the-gate",
        description: "Use when operating the gate.",
        prose: text`# Operate the gate`,
      }),
    ],
    expect: [{ kind: skill, clauses: [] }],
  });
  assert.doesNotThrow(() => emit(h));
});

// ---------------------------------------------------------------------------
// (2) Unfilled required requirement — deferred to the engine, never SDK-side.
//     The SDK sees only composed `satisfies`, never a layout host's edge-slot
//     fills, so a fill pre-flight here would refuse a layout-fill corpus that
//     the engine (reading both) accepts. Emit must produce output; the engine's
//     requirement clause is the fill gate.
// ---------------------------------------------------------------------------

test("emit does not refuse an assembly requirement marked required with no composed satisfier", () => {
  const h = harness({
    require: {
      "engineering-standards": {
        prose: "the repo carries a rule fixing the engineering bar",
        kind: rule,
        required: true,
      },
    },
    // No member's composed `satisfies` fills the requirement — a layout-content
    // kind's edge slot could, and the SDK cannot see it, so it defers the gate.
    members: [rule({ name: "rust", prose: text`# Rust` })],
  });
  assert.doesNotThrow(() => emit(h));
});

test("emit does not refuse a member-published requirement marked required with no composed satisfier", () => {
  const h = harness({
    members: [
      skill({
        name: "operate-the-gate",
        description: "Use when operating the gate.",
        prose: text`# Operate the gate`,
        requires: { playbook: { prose: "a shared gate playbook exists", kind: rule, required: true } },
      }),
    ],
  });
  assert.doesNotThrow(() => emit(h));
});

// ---------------------------------------------------------------------------
// (3) Unadmitted nesting — a `blocks()` value of a kind the corpus never admitted
//     over the host kind.
// ---------------------------------------------------------------------------

test("emit refuses a blocks() value whose kind the corpus never admitted over the host kind", () => {
  // The corpus admits `decision` over `skill`, but the value is attached to a `memory`
  // host — an unadmitted nesting that would reach the lock as a row no `templates` admits.
  const decision = decisionKind();
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({ kind: decision, key: "surface-authority", leaves: { chosen: "x" } }),
        ),
      }),
    ],
    admit: [{ host: skill, admits: [decision] }],
  });
  assert.throws(() => emit(h), /surface-authority.*does not nest within host kind `memory`/s);
});

test("emit refuses a blocks() value whose kind the corpus admits nowhere", () => {
  // No admission at all — a bare-string value name that ties to no declared nesting
  // refuses just as a host mismatch does. Absent `admit`, a host admits nothing.
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(embeddedMemberValue({ kind: "decision", key: "orphan", leaves: { chosen: "x" } })),
      }),
    ],
  });
  assert.throws(() => emit(h), /does not nest within host kind/);
});

test("emit refuses an admission naming a kind that is not embedded", () => {
  // A file-locus kind owns a file; admitting one over a host declares a nesting no
  // locus backs.
  const h = harness({
    members: [memory({ name: "CLAUDE", prose: text`# Memory` })],
    admit: [{ host: memory, admits: [rule] }],
  });
  assert.throws(() => emit(h), /admits `rule`, which is not an embedded kind/);
});

test("a host-admitted blocks() value compiles without throwing", () => {
  // The corpus admits `decision` over `memory` and the value is attached to a `memory`
  // host — an admitted nesting, so emit produces output.
  const decision = decisionKind();
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({ kind: decision, key: "surface-authority", leaves: { chosen: "x" } }),
        ),
      }),
    ],
    admit: [{ host: memory, admits: [decision] }],
  });
  assert.doesNotThrow(() => emit(h));
});

// ---------------------------------------------------------------------------
// (4) A file() value composed inside blocks() — refused at the constructor, so the
//     runtime holds the line blocks()'s parameter type already draws.
// ---------------------------------------------------------------------------

test("blocks() refuses a file() child, naming its index and what a composed body admits", () => {
  // The parameter type excludes `File`, so only a cast caller arrives — the refusal is
  // what stands between one and a raw TypeError deep in leaf resolution.
  assert.throws(
    () => blocks(file(import.meta.url, "./long.md") as never),
    /blocks\(\): block 0 is a `file\(\)` value.*`text` span or an embedded member value/s,
  );
});

test("blocks() names the offending index, not merely the first block", () => {
  assert.throws(
    () => blocks(text`# Prologue`, file(import.meta.url, "./long.md") as never),
    /block 1 is a `file\(\)` value/,
  );
});

test("blocks() points a file-bodied author at the two homes that already exist", () => {
  // The refusal is only actionable if it routes: a whole-body `file()`, or `include()`
  // interpolated into a span when the bytes belong inside a composed body.
  assert.throws(() => blocks(file(import.meta.url, "./long.md") as never), /prose: file\(…\)/);
  assert.throws(() => blocks(file(import.meta.url, "./long.md") as never), /interpolate `include\(\)`/);
});

test("blocks() admits an embedded value of a child kind named `file`", () => {
  // `EmbeddedMemberValue.kind` is a free-form kind name, so the `file` tag alone is
  // ambiguous — a corpus that declares a child kind by that name must not be refused.
  const fileKind = kind<object>({
    name: "file",
    locus: { kind: "embedded" },
    unitShape: "file",
    registration: [],
  });
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(embeddedMemberValue({ kind: fileKind, key: "the-doc", leaves: { chosen: "x" } })),
      }),
    ],
    admit: [{ host: memory, admits: [fileKind] }],
  });
  assert.doesNotThrow(() => emit(h));
});

test("blocks() admits a bare prose span", () => {
  // The guard bounds `blocks()`'s children and nothing else — a `text` span composes as
  // it always did.
  assert.doesNotThrow(() => blocks(text`# Prologue`));
});

// ---------------------------------------------------------------------------
// (5) Dangling edge target — an embedded value's edge field naming a member the
//     program does not resolve. Refusal reaches exactly as far as the program's own
//     universe, the same boundary a bare mention draws: an address naming a declared
//     `at`-locus kind may name a member discovered on disk, so it defers to `check`;
//     an address naming no declared kind refuses before a byte is written.
// ---------------------------------------------------------------------------

/** An embedded kind whose `source` field is a declared edge to a `rule`, optionally rendered. */
function citationKind(render?: (value: ResolvedEmbeddedMemberValue) => string) {
  return kind<object>(
    {
      name: "citation",
      locus: { kind: "embedded" },
      unitShape: "file",
      registration: [],
      edgeFields: [{ field: "source", to: ["rule"] }],
    },
    render === undefined ? {} : { render },
  );
}

/** A `memory` host carrying one `citation` value whose `source` leaf reads `address`. */
function citingHarness(citation: ReturnType<typeof citationKind>, address: string) {
  return harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(embeddedMemberValue({ kind: citation, key: "the-standard", leaves: { source: address } })),
      }),
    ],
    admit: [{ host: memory, admits: [citation] }],
  });
}

/**
 * A program that declares `rule` (it composes `rust`) but not `ghost`, cited by a
 * `citation` whose `source` edge names that uncomposed member — the deferring case.
 */
function ghostCitingHarness(citation: ReturnType<typeof citationKind>) {
  return harness({
    members: [
      rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust` }),
      ...citingHarness(citation, "rule:ghost").members,
    ],
    admit: [{ host: memory, admits: [citation] }],
  });
}

test("emit defers an edge field naming a declared at-locus kind's uncomposed member", () => {
  // The address is inside the kind's universe and outside the program's, so emit derives
  // no facts and `check` owns the route verdict — the same deferral a *mention* of
  // `rule:ghost` takes. The case is true absence of the member: an embedded target the
  // program *did* compose resolves off the member table's nested spellings (emit.test.ts,
  // "either spelling"), never here.
  const result = emit(ghostCitingHarness(citationKind()));
  // The authored address rides the lock's `nested_member` row as written — the leaf the
  // engine lifts into the member's fields and route-resolves against the discovered
  // corpus (tests/it/graph.rs, the `graph.route` twin).
  assert.deepEqual(result.declarations.nested_members[0].leaves, { source: "rule:ghost" });
  // A deferred field carries no facts, so it is no `format-places-edges` obligation:
  // there is nothing to place, and the route verdict is the gate's.
  assert.deepEqual(result.declarations.nested_members[0].placed_edges, undefined);
});

test("a render reading a deferred edge field's facts refuses by name", () => {
  // The deferral derives no facts, so there is nothing for a hook to spell a reference
  // off — and an absent key would surface as a `TypeError` in the author's own template,
  // loud but unnamed and pointing at the wrong file. The read itself is the refusal, and
  // it names the member, the embedded value, the edge field and the authored address
  // (invariant 6, "Loud or nothing").
  const citation = citationKind((value) => `See [${value.targets.source.name}](${value.targets.source.path}).`);
  let thrown: unknown;
  try {
    emit(ghostCitingHarness(citation));
  } catch (error) {
    thrown = error;
  }
  assert.ok(thrown instanceof Error, "a deferred read refuses, and never with a raw TypeError");
  assert.ok(!(thrown instanceof TypeError), `refused with a TypeError: ${thrown.message}`);
  assert.match(thrown.message, /member `CLAUDE`/);
  assert.match(thrown.message, /embedded value `the-standard` of kind `citation`/);
  assert.match(thrown.message, /edge field `source` names `rule:ghost`/);
  assert.match(thrown.message, /defers to `check`/);
});

test("a render that never names a deferred edge field emits clean", () => {
  // The other half of the same ruling: deferral is not a render-selection rule. A format
  // that does not reach for the absent facts writes its bytes, and the authored address
  // still rides the `nested_member` row for `check` to route.
  const citation = citationKind((value) => `The standard: ${value.leaves.source ?? "unstated"}.`);
  const result = emit(ghostCitingHarness(citation));
  assert.deepEqual(result.declarations.nested_members[0].leaves, { source: "rule:ghost" });
  const claude = result.members.find((member) => member.name === "CLAUDE");
  assert.ok(claude !== undefined);
  assert.match(claude.body, /The standard: rule:ghost\./);
  // Reading the *leaf* is a placement, but the deferred field derives no facts and so
  // carries no obligation: `placed_edges` stays absent either way.
  assert.deepEqual(result.declarations.nested_members[0].placed_edges, undefined);
});

test("emit refuses an edge field whose address names no declared kind", () => {
  // The universe rule's other side. No `journal` kind is in play, so no member of that
  // kind can be discovered on disk either: nothing downstream could ever resolve the
  // address, and the fault is the program's to answer now.
  assert.throws(() => emit(citingHarness(citationKind(), "journal:ghost")), /resolves to no composed member/);
});

test("emit refuses an edge field naming a target that owns no projection", () => {
  const citation = citationKind();
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({ kind: citation, key: "the-standard", leaves: { source: "hook:PreToolUse" } }),
        ),
      }),
      hook({ name: "PreToolUse", hooks: [{ type: "command", command: "temper guard" }] }),
    ],
    admit: [{ host: memory, admits: [citation] }],
  });
  // A registration member owns no artifact of its own, so there is no path to point at.
  // An *embedded* target is the resolvable case beside it: it owns no file either, yet
  // its host does, and the rendered path is that host's projection.
  assert.throws(() => emit(h), /owns no projection to reference/);
});

test("emit refuses a bare nested key several hosts carry — an ambiguous address names nothing", () => {
  const citation = kind<object>(
    {
      name: "citation",
      locus: { kind: "embedded" },
      unitShape: "file",
      registration: [],
      edgeFields: [{ field: "source", to: ["decision"] }],
    },
    { render: (value) => `See \`${value.targets.source.address}\`.` },
  );
  const decision = decisionKind();
  const build = (source: string) =>
    emit(
      harness({
        members: [
          rule({
            name: "rust",
            paths: ["src/**/*.rs"],
            prose: blocks(embeddedMemberValue({ kind: decision, key: "surface-authority", leaves: {} })),
          }),
          rule({
            name: "sdk",
            paths: ["sdk/**/*.ts"],
            prose: blocks(embeddedMemberValue({ kind: decision, key: "surface-authority", leaves: {} })),
          }),
          memory({
            name: "CLAUDE",
            prose: blocks(embeddedMemberValue({ kind: citation, key: "the-standard", leaves: { source } })),
          }),
        ],
        admit: [
          { host: rule, admits: [decision] },
          { host: memory, admits: [citation] },
        ],
      }),
    );

  // Uniqueness is the resolver's bar, not the corpus's (decision 0049): two hosts keying
  // one nested member alike compose fine, and only the bare citation of them refuses —
  // naming every carrier, never silently picking one.
  assert.throws(() => build("surface-authority"), /a bare key 2 hosts carry \(`rule:rust`, `rule:sdk`\)/);
  // The full spelling tells them apart, so the same harness emits when the leaf composes
  // the address through its host.
  assert.equal(
    build("rule:sdk/decision/surface-authority").members.find((m) => m.name === "CLAUDE")!.body,
    "See `rule:sdk/decision/surface-authority`.\n",
  );
});

test("an unfilled edge field is no edge — it emits, deriving no target facts", () => {
  const result = emit(citingHarness(citationKind(), ""));
  // Requiredness is the kind's own field schema, which fails in the author's program at
  // compose time; refusal here reaches only a reference filled yet unresolvable.
  assert.deepEqual(result.declarations.nested_members[0].placed_edges, undefined);
});

test("an edge field resolving to a composed member emits without throwing", () => {
  const citation = citationKind();
  const h = harness({
    members: [rule({ name: "rust", prose: text`# Rust` }), ...citingHarness(citation, "rule:rust").members],
    admit: [{ host: memory, admits: [citation] }],
  });
  assert.doesNotThrow(() => emit(h));
});

// ---------------------------------------------------------------------------
// (6) Reserved leaf — a composed leaf named `prose`, the key a read member's own
//     span owns (0051). Refused at compose, where the author can still rename the
//     field, on the value's own leaves and on every collection entry's alike.
// ---------------------------------------------------------------------------

test("embeddedMemberValue refuses a top-level leaf named `prose`, naming the rename", () => {
  const decision = decisionKind();
  assert.throws(
    () =>
      embeddedMemberValue({
        kind: decision,
        key: "surface-authority",
        leaves: { prose: "the words" },
      }),
    (error: Error) => {
      assert.match(error.message, /embedded member `decision` `surface-authority`: leaf `prose` is reserved/);
      // The remedy is named, and it is the one an author of an embedded value can take:
      // this init carries leaves and collections alone, so a member-prose route would
      // point at a surface that does not exist here.
      assert.match(error.message, /rename the field$/);
      assert.doesNotMatch(error.message, /member's prose/);
      return true;
    },
  );
});

test("embeddedMemberValue refuses a collection entry's leaf named `prose`", () => {
  const decision = decisionKind();
  assert.throws(
    () =>
      embeddedMemberValue({
        kind: decision,
        key: "surface-authority",
        leaves: { chosen: "x" },
        collections: { alternatives: [{ key: "the-other-way", leaves: { prose: "the words" } }] },
      }),
    // The address the refusal names is the entry's, not the host value's — an author
    // reading it knows which of many entries to rename.
    /`surface-authority\.alternatives\.the-other-way`: leaf `prose` is reserved/,
  );
});

test("embeddedMemberValue admits every other leaf key, top-level and in a collection", () => {
  // The reservation binds exactly one name: a near-miss key composes, or the guard
  // would be refusing the authored fields it exists to protect.
  const decision = decisionKind();
  assert.doesNotThrow(() =>
    embeddedMemberValue({
      kind: decision,
      key: "surface-authority",
      leaves: { chosen: "x", prosecution: "not the reserved key" },
      collections: { alternatives: [{ key: "the-other-way", leaves: { rejected: "y" } }] },
    }),
  );
});

// ---------------------------------------------------------------------------
// (7) A key that is not one address segment — a member's identity *is* its
//     `<host>/<kind>/<key>` address, so a key carrying the grammar's own `/` spells
//     the sibling leaf address `<host>/<kind>/authority/rejected`, which every
//     reader answers at leaf grain. Refused at compose, where the author can still
//     rename, on the same posture as the reserved leaf above.
// ---------------------------------------------------------------------------

test("embeddedMemberValue refuses a key carrying the address grammar's separator", () => {
  const decision = decisionKind();
  assert.throws(
    () =>
      embeddedMemberValue({
        kind: decision,
        key: "authority/rejected",
        leaves: { chosen: "x" },
      }),
    (error: Error) => {
      // The key the author wrote is named back, and so is the separator that makes it
      // two segments — together they are the whole remedy.
      assert.match(error.message, /embedded member `decision` `authority\/rejected`:/);
      assert.match(error.message, /one address segment/);
      assert.match(error.message, /`\/`/);
      return true;
    },
  );
});

test("embeddedMemberValue refuses an empty key", () => {
  // The grammar's other hole: an empty segment names nothing at any grain, so the
  // address the member would spell resolves to no member at all.
  const decision = decisionKind();
  assert.throws(
    () => embeddedMemberValue({ kind: decision, key: "", leaves: { chosen: "x" } }),
    /embedded member `decision` ``: a member's key is one address segment/,
  );
});

test("embeddedMemberValue admits a well-formed key, dots and a collection path included", () => {
  // Non-vacuity, and the deliberate scope: the refusal binds the member's own key. A
  // *collection entry* key lands inside the leaf tail — the whole remainder after the
  // third slash — so it aliases nothing and stays legal.
  const decision = decisionKind();
  assert.doesNotThrow(() =>
    embeddedMemberValue({
      kind: decision,
      key: "surface-authority",
      leaves: { chosen: "x" },
      collections: { alternatives: [{ key: "the/other.way", leaves: { rejected: "y" } }] },
    }),
  );
});

// ---------------------------------------------------------------------------
// A projected member's own NAME, one grain up from the key above: it is the member's
//     first address segment and, at every `at` locus, the identity its projection path
//     is spliced from — a file stem, or the one directory segment the kind's glob
//     stars. A `/` in it shifts every address beneath the member by one segment AND
//     places the file where no discovery walk looks. Refused at compose, where the
//     author can still rename; the engine refuses the same name over the payload.
// ---------------------------------------------------------------------------

test("emit refuses a member name carrying the address separator", () => {
  const h = harness({
    members: [rule({ name: "lang/rust", prose: text`# Rust` })],
  });
  assert.throws(
    () => emit(h),
    (error: Error) => {
      // The name the author wrote is named back, and so is the separator that breaks it.
      assert.match(error.message, /member `lang\/rust` of kind `rule`:/);
      assert.match(error.message, /one address segment/);
      assert.match(error.message, /`\/`/);
      return true;
    },
  );
});

test("emit refuses a slashed member name on a directory-unit kind too", () => {
  // Total over the projected set, not narrowed to the splice: a directory unit composes
  // `<root>/<name>/<entry>` without splicing a glob at all, so a splice-side guard would
  // admit exactly this shape.
  const h = harness({
    members: [
      skill({
        name: "team/coordinate",
        description: "Use when driving a complex task across a team of agents.",
        prose: text`# Coordinate`,
      }),
    ],
  });
  assert.throws(() => emit(h), /member `team\/coordinate` of kind `skill`/);
});

// ---------------------------------------------------------------------------
// (9) Two members at one address — the coincidence a nested file child's identity
//     scopes to its host. A file child's address carries its host, so the refusal
//     narrows to what is actually coincident: one host carrying the same
//     `(kind, key)` twice, never two hosts each carrying the name once.
// ---------------------------------------------------------------------------

/** A nested-file child kind: it governs no glob, its path being its host's declared fact. */
const supportingDoc = kind<Record<never, never>>({
  name: "supporting-doc",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
});

/** A directory-unit host templating one `supporting-doc` file layer at `*.md`. */
const guide = kind<Record<never, never>>({
  name: "guide",
  locus: { kind: "at", root: ".claude/guides", glob: "*/GUIDE.md" },
  unitShape: "directory",
  registration: [{ via: "always" }],
  templates: [{ kind: supportingDoc, path: "*.md" }],
});

test("emit refuses two file children of one kind under one host — one address, twice", () => {
  const alpha = guide({ name: "alpha" });
  const h = harness({
    members: [
      alpha,
      supportingDoc({ name: "home", host: alpha, prose: text`# Home` }),
      supportingDoc({ name: "home", host: alpha, prose: text`# Home again` }),
    ],
  });
  // Coincident addresses are a malformed lock, and here they truly coincide: one host,
  // one key, twice — the two would project onto the very same file.
  assert.throws(() => emit(h), /duplicate identity key `guide:alpha\/supporting-doc\/home`/);
});

test("two hosts each carrying the same child name is two members, and emits", () => {
  const alpha = guide({ name: "alpha" });
  const beta = guide({ name: "beta" });
  const h = harness({
    members: [
      alpha,
      beta,
      supportingDoc({ name: "home", host: alpha, prose: text`# Alpha's home` }),
      supportingDoc({ name: "home", host: beta, prose: text`# Beta's home` }),
    ],
  });
  // The refusal a corpus-wide `supporting-doc:home` keying raised for a corpus that is
  // perfectly well-formed: the host segment tells the two apart, and both files project.
  assert.doesNotThrow(() => emit(h));
});

// ---------------------------------------------------------------------------
// A clean harness — every join resolves, every required requirement filled.
// ---------------------------------------------------------------------------

test("a clean harness emits without throwing", () => {
  const h = harness({
    require: {
      "engineering-standards": {
        prose: "the repo carries a rule fixing the engineering bar",
        kind: rule,
        required: true,
      },
    },
    members: [
      rule({ name: "rust", prose: text`# Rust`, satisfies: ["engineering-standards"] }),
    ],
  });
  assert.doesNotThrow(() => emit(h));
});
