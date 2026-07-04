/**
 * Declare-side emit refusals — a broken source yields no output, never silent
 * bytes (`specs/architecture/20-surface.md`, "Emit refuses before it writes").
 * Two cases the compiler must catch before it writes a byte: a `satisfies` claim
 * that names no declared requirement (a dangling join), and a `required`
 * requirement no member fills (an unfilled required requirement). Both `emit` and
 * `emitManifestMembers` run the check; a clean harness passes through untouched.
 *
 * Mention refusals live in emit.test.ts ("an unresolved mention is a loud emit
 * error") — this file owns only the two declare-side cases, so CONTRACT-DIR's
 * later fixture move stays disjoint from the emit byte-parity corpus.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { defineHarness, emit, emitManifestMembers, md, rule, skill } from "../src/index.js";

// ---------------------------------------------------------------------------
// (1) Dangling join — a `satisfies` claim resolving to no declared requirement.
// ---------------------------------------------------------------------------

test("emit refuses a satisfies claim naming no declared requirement", () => {
  const harness = defineHarness({
    members: [
      rule({
        name: "rust",
        body: md`# Rust`,
        // No requirement — assembly-level or member-published — carries this name.
        satisfies: { "ghost-requirement": { rationale: "fills nothing that exists" } },
      }),
    ],
  });
  assert.throws(() => emitManifestMembers(harness), /a dangling join/);
  assert.throws(() => emit(harness), /a dangling join/);
});

test("a satisfies claim filling a member-published requirement is a live join", () => {
  const harness = defineHarness({
    members: [
      skill({
        name: "operate-the-gate",
        body: md`# Operate the gate`,
        requirements: {
          playbook: { means: "a shared gate playbook exists", kind: "claude-code.rule" },
        },
      }),
      rule({
        name: "gate-playbook",
        body: md`# Gate playbook`,
        satisfies: { playbook: { rationale: "carries the playbook the skill demands" } },
      }),
    ],
  });
  // The far end is a member-published requirement, not an assembly one — still a
  // declared requirement, so the join resolves and emit produces output.
  assert.doesNotThrow(() => emitManifestMembers(harness));
});

// ---------------------------------------------------------------------------
// (2) Unfilled required requirement — a `required` demand no member satisfies.
// ---------------------------------------------------------------------------

test("emit refuses an assembly requirement marked required that no member fills", () => {
  const harness = defineHarness({
    requirements: {
      "engineering-standards": {
        means: "the repo carries a rule fixing the engineering bar",
        kind: "claude-code.rule",
        required: true,
      },
    },
    // A member, but none satisfies `engineering-standards`.
    members: [rule({ name: "rust", body: md`# Rust` })],
  });
  assert.throws(() => emitManifestMembers(harness), /an unfilled required requirement/);
  assert.throws(() => emit(harness), /an unfilled required requirement/);
});

test("emit refuses a member-published requirement marked required that no member fills", () => {
  const harness = defineHarness({
    members: [
      skill({
        name: "operate-the-gate",
        body: md`# Operate the gate`,
        requirements: {
          playbook: { means: "a shared gate playbook exists", kind: "claude-code.rule", required: true },
        },
      }),
    ],
  });
  assert.throws(() => emitManifestMembers(harness), /an unfilled required requirement/);
});

// ---------------------------------------------------------------------------
// A clean harness — every join resolves, every required requirement filled —
// emits without throwing.
// ---------------------------------------------------------------------------

test("a clean harness emits without throwing", () => {
  const harness = defineHarness({
    requirements: {
      "engineering-standards": {
        means: "the repo carries a rule fixing the engineering bar",
        kind: "claude-code.rule",
        required: true,
      },
    },
    members: [
      rule({
        name: "rust",
        body: md`# Rust`,
        satisfies: { "engineering-standards": { rationale: "carries the Rust conventions" } },
      }),
    ],
  });
  assert.doesNotThrow(() => emitManifestMembers(harness));
  assert.doesNotThrow(() => emit(harness));
});
