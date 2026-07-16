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

/**
 * A `source`-shaped `at`-locus kind whose members live on disk under `src/`, never
 * composed in the program — the discovery-locus a deferred mention names.
 */
const sourceKind = kind<Record<never, never>>({
  name: "source",
  locus: { kind: "at", root: "src", glob: "*.js" },
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

test("a dangling mention in a composed-body prose span refuses at emit, like a member-level text body", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(text`It rests on ${{ address: "rule:ghost", display: "ghost" }}.`),
      }),
    ],
  });

  assert.throws(() => emit(h), /a mention cannot dangle/);
});

test("a mention naming a declared at-locus kind with no composed member defers to check, riding the lock", () => {
  // `source` is declared (bound via `expect`) but has no composed member: its members
  // are discovered on disk. `source:main` names that discovery locus, so emit defers
  // rather than refusing — the row rides the lock for `check` to resolve.
  const citer = rule({
    name: "citations",
    prose: text`It rests on ${{ address: "source:main", display: "main" }}.`,
  });
  const h = harness({
    members: [citer],
    expect: [{ kind: sourceKind, clauses: [] }],
  });

  const result = emit(h);
  const member = result.members.find((m) => m.name === "citations")!;
  assert.match(member.body, /It rests on main\./);
  assert.deepEqual(result.declarations.mentions, [{ member: "rule:citations", target: "source:main" }]);
});

test("a mention naming no declared kind still refuses at emit — no discovery locus to defer to", () => {
  // `source` is nowhere declared here (no member of it, no `expect` binding), so the
  // mention names an undeclared kind: dangling with no discovery locus, refused at emit.
  const citer = rule({
    name: "citations",
    prose: text`It rests on ${{ address: "source:main", display: "main" }}.`,
  });
  const h = harness({ members: [citer] });

  assert.throws(() => emit(h), /a mention cannot dangle/);
});

test("a composed-body prose span's mention keys to the host kind:name and mints no nested member", () => {
  const h = harness({
    members: [
      rule({ name: "rust", prose: text`# Rust` }),
      memory({
        name: "CLAUDE",
        prose: blocks(
          text`Follow ${{ address: "rule:rust", display: "rust" }} for the standard.`,
          embeddedMemberValue({
            kind: memoryDecision,
            key: "done-is-exact",
            leaves: { chosen: "the scanner reports every finding" },
          }),
        ),
      }),
    ],
    expect: [{ kind: memoryDecision, clauses: [] }],
  });

  const result = emit(h);
  // The span's mention keys to the host member's own kind:name — the same row a
  // member-level text body yields — never a leaf-scoped `<host>/<kind>/<key>` address.
  assert.deepEqual(result.declarations.mentions, [{ member: "memory:CLAUDE", target: "rule:rust" }]);
  // The span mints no nested-member row; only the interleaved embedded value does.
  assert.deepEqual(
    result.declarations.nested_members.map((row) => `${row.kind}:${row.key}`),
    ["decision:done-is-exact"],
  );
});
