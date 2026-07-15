/**
 * Mention targets — the resolution set a citation edge names, and the adapter
 * that spells a member as one. A mention may target a top-level member
 * (`kind:name`) or an embedded member (the host-scoped
 * `<host-kind>:<host-name>/<kind>/<key>` address); both must resolve, or the
 * emit refuses the mention as dangling.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { blocks, embeddedMemberValue, emit, harness, kind, mentionOf, text } from "../src/index.js";
import { memory, rule } from "../src/claude-code.js";

/** The `decision` embedded kind the host below nests, bound in via `expect` so it is in play. */
const memoryDecision = kind<Record<never, never>>({
  name: "decision",
  locus: { kind: "embedded", withinHosts: ["memory"] },
  unitShape: "file",
  registration: [{ via: "always" }],
});

/** A host carrying one embedded `decision` member keyed `done-is-exact`. */
function hostWithEmbeddedMember() {
  return memory({
    name: "CLAUDE",
    prose: blocks(
      embeddedMemberValue({
        kind: memoryDecision,
        key: "done-is-exact",
        leaves: { chosen: "the scanner reports every finding" },
      }),
    ),
  });
}

test("mentionOf spells a member's kind:name mention target", () => {
  const member = rule({ name: "rust", prose: text`# Rust` });
  assert.deepEqual(mentionOf(member), { address: "rule:rust", display: "rust" });
});

test("a mention targeting an embedded member's host-scoped address resolves without a dangling refusal", () => {
  const citer = rule({
    name: "citations",
    prose: text`It upholds ${{ address: "memory:CLAUDE/decision/done-is-exact", display: "done-is-exact" }}.`,
  });
  const h = harness({
    members: [hostWithEmbeddedMember(), citer],
    expect: [{ kind: memoryDecision, clauses: [] }],
  });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "citations")!;
  assert.match(member.body, /It upholds done-is-exact\./);
});

test("the embedded address is host-scoped, never a flat kind:key — a flat mention still dangles", () => {
  const citer = rule({
    name: "citations",
    prose: text`It upholds ${{ address: "decision:done-is-exact", display: "done-is-exact" }}.`,
  });
  const h = harness({
    members: [hostWithEmbeddedMember(), citer],
    expect: [{ kind: memoryDecision, clauses: [] }],
  });

  assert.throws(() => emit(h), /a mention cannot dangle/);
});
