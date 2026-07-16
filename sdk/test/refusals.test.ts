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

import { blocks, embeddedMemberValue, emit, harness, kind, text } from "../src/index.js";
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
// (4) Dangling edge target — an embedded value's edge field naming a member the
//     program does not resolve. Unlike a bare mention, an edge target never defers
//     to the gate: its facts are rendered into the projection now, so an
//     unresolved one has nothing true to place.
// ---------------------------------------------------------------------------

/** An embedded kind whose `source` field is a declared edge to a `rule`. */
function citationKind() {
  return kind<object>({
    name: "citation",
    locus: { kind: "embedded" },
    unitShape: "file",
    registration: [],
    edgeFields: [{ field: "source", to: "rule" }],
  });
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

test("emit refuses an edge field whose target the program does not resolve", () => {
  // `rule` is a declared at-locus kind, so a *mention* of `rule:ghost` would defer to
  // check — an edge target cannot: the reference is written now.
  assert.throws(() => emit(citingHarness(citationKind(), "rule:ghost")), /resolves to no composed member/);
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
      hook({ name: "PreToolUse", command: "temper guard" }),
    ],
    admit: [{ host: memory, admits: [citation] }],
  });
  assert.throws(() => emit(h), /owns no projection to reference/);
});

test("emit refuses an edge field whose leaf names no target at all", () => {
  assert.throws(() => emit(citingHarness(citationKind(), "")), /names no target/);
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
