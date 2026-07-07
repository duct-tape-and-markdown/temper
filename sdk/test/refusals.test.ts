/**
 * Declare-side emit refusals — a broken source yields no output, never silent
 * bytes (`specs/model/pipeline.md`, "Emit").
 * Two cases the compile must catch before it writes a byte: a `satisfies` claim
 * that names no declared requirement (a dangling join), and a `required`
 * requirement no member fills (an unfilled required requirement). A requirement
 * may be published by the assembly's `require` or a member's own `requires` — one
 * namespace, one fill (`specs/model/contract.md`, "requirement"). A clean harness emits without throwing.
 *
 * Mention refusals live in emit.test.ts ("an unresolved mention is a loud emit
 * error"); this file owns only the two declare-side cases.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { emit, harness, text } from "../src/index.js";
import { rule, skill } from "../src/claude-code.js";

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
        requires: { playbook: { means: "a shared gate playbook exists", kind: rule } },
      }),
      rule({ name: "gate-playbook", prose: text`# Gate playbook`, satisfies: ["playbook"] }),
    ],
  });
  // The far end is a member-published requirement — still a declared requirement,
  // so the join resolves and emit produces output.
  assert.doesNotThrow(() => emit(h));
});

// ---------------------------------------------------------------------------
// (2) Unfilled required requirement — a `required` demand no member satisfies.
// ---------------------------------------------------------------------------

test("emit refuses an assembly requirement marked required that no member fills", () => {
  const h = harness({
    require: {
      "engineering-standards": {
        means: "the repo carries a rule fixing the engineering bar",
        kind: rule,
        required: true,
      },
    },
    members: [rule({ name: "rust", prose: text`# Rust` })],
  });
  assert.throws(() => emit(h), /an unfilled required requirement/);
});

test("emit refuses a member-published requirement marked required that no member fills", () => {
  const h = harness({
    members: [
      skill({
        name: "operate-the-gate",
        description: "Use when operating the gate.",
        prose: text`# Operate the gate`,
        requires: { playbook: { means: "a shared gate playbook exists", kind: rule, required: true } },
      }),
    ],
  });
  assert.throws(() => emit(h), /an unfilled required requirement/);
});

// ---------------------------------------------------------------------------
// A clean harness — every join resolves, every required requirement filled.
// ---------------------------------------------------------------------------

test("a clean harness emits without throwing", () => {
  const h = harness({
    require: {
      "engineering-standards": {
        means: "the repo carries a rule fixing the engineering bar",
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
