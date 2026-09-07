/**
 * The member-address grammar — every writer's output read back by the one parser, and
 * the one qualified nested spelling three independent producers must agree on. Twenty-one
 * inline template literals could only assert that agreement by inspection; a single writer
 * makes it structural, and this file pins that it stayed so.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { blocks, embeddedMemberValue, emit, harness, kind } from "../src/index.js";
import { memory, rule } from "../src/claude-code.js";
import { declaredAddresses } from "../src/declarations.js";
import {
  bareLookupKey,
  edgeLookupKey,
  hostAddress,
  leafAddress,
  nestedAddress,
  parseHostAddress,
  parseLeafAddress,
  parseNestedAddress,
} from "../src/member-address.js";

test("a host address round-trips through its own reader", () => {
  assert.equal(hostAddress("rule", "collaboration"), "rule:collaboration");
  assert.deepEqual(parseHostAddress(hostAddress("rule", "collaboration")), {
    kind: "rule",
    name: "collaboration",
  });

  // The split is at the *first* colon, so a name carrying one stays whole.
  assert.deepEqual(parseHostAddress("rule:a:b"), { kind: "rule", name: "a:b" });

  // A segment-shaped hole names nothing — and neither does a segmented spelling, whose
  // `/` is this grammar's own separator and whose readers are the two below.
  for (const address of ["collaboration", ":collaboration", "rule:", "skill:x/hook/on-enter"]) {
    assert.equal(parseHostAddress(address), undefined, `\`${address}\` is no host address`);
  }
});

test("a nested-member address round-trips, and stops at member grain", () => {
  const written = nestedAddress(hostAddress("skill", "use-when-x"), "hook", "on-enter");
  assert.equal(written, "skill:use-when-x/hook/on-enter");
  assert.deepEqual(parseNestedAddress(written), {
    host: "skill:use-when-x",
    kind: "hook",
    key: "on-enter",
  });

  // The host segment is itself a member address, so a bare head names no nested member.
  assert.equal(parseNestedAddress("note/requirement/my-req"), undefined);
  assert.equal(parseLeafAddress(written), undefined, "member grain carries no leaf");
});

test("a leaf address is the nested address it is a tail of, plus its leaf", () => {
  const written = leafAddress(hostAddress("skill", "use-when-x"), "hook", "on-enter", "command");
  assert.equal(written, `${nestedAddress(hostAddress("skill", "use-when-x"), "hook", "on-enter")}/command`);
  assert.deepEqual(parseLeafAddress(written), {
    member: "skill:use-when-x",
    kind: "hook",
    key: "on-enter",
    childPath: "command",
  });
  assert.equal(parseNestedAddress(written), undefined, "a leaf tail is no member address");

  // The leaf path is the whole remainder after the third slash — its dots and its deeper
  // slashes intact, never re-cut into a fifth segment.
  assert.equal(parseLeafAddress("spec:20/decision/authority/rejected.baked.because")?.childPath, "rejected.baked.because");
  assert.equal(parseLeafAddress("spec:20/decision/authority/a/b")?.childPath, "a/b");

  // The bare member head this SDK's own leaf writer spells parses at leaf grain.
  assert.equal(parseLeafAddress(leafAddress("CLAUDE", "decision", "authority", "chosen"))?.member, "CLAUDE");
});

test("a segment-shaped hole names nothing at either grain", () => {
  for (const address of ["/hook/on-enter", "skill:x//on-enter", "skill:x/hook/", "skill:x/hook"]) {
    assert.equal(parseNestedAddress(address), undefined, `\`${address}\` is no nested-member address`);
  }
  for (const address of ["/hook/on-enter/command", "skill:x//on-enter/command", "skill:x/hook//command", "skill:x/hook/on-enter/"]) {
    assert.equal(parseLeafAddress(address), undefined, `\`${address}\` is no leaf address`);
  }
});

test("the bare lookup key is a key, never an address", () => {
  // A one-element `to` set lifts an unqualified reference into the lookup key; anything
  // the author already qualified — a host address or a whole nested address — stands.
  assert.equal(edgeLookupKey("surface-authority", ["decision"]), bareLookupKey("decision", "surface-authority"));
  assert.equal(edgeLookupKey("decision:surface-authority", ["decision"]), "decision:surface-authority");
  assert.equal(edgeLookupKey("rule:sdk/decision/surface-authority", ["decision"]), "rule:sdk/decision/surface-authority");
  // A multi-kind `to` reads the kind-qualified spelling the author wrote, lifting nothing.
  assert.equal(edgeLookupKey("surface-authority", ["decision", "hook"]), "surface-authority");
});

/** An embedded-locus kind named `decision` — the nested member the address below names. */
function decisionKind() {
  return kind<object>({ name: "decision", locus: { kind: "embedded" }, unitShape: "file", registration: [] });
}

/** An embedded kind whose `source` edge renders the target facts' own address verbatim. */
function citationKind() {
  return kind<object>(
    {
      name: "citation",
      locus: { kind: "embedded" },
      unitShape: "file",
      registration: [],
      edgeFields: [{ field: "source", to: ["decision"] }],
    },
    { render: (value) => `See \`${value.targets.source.address}\`.` },
  );
}

test("the member table, declaredAddresses and an edge's target facts spell one nested address", () => {
  const address = nestedAddress(hostAddress("rule", "sdk"), "decision", "surface-authority");
  const decision = decisionKind();
  const citation = citationKind();
  const h = harness({
    members: [
      rule({
        name: "sdk",
        paths: ["sdk/**/*.ts"],
        prose: blocks(embeddedMemberValue({ kind: decision, key: "surface-authority", leaves: {} })),
      }),
      memory({
        name: "CLAUDE",
        prose: blocks(embeddedMemberValue({ kind: citation, key: "the-standard", leaves: { source: address } })),
      }),
    ],
    admit: [
      { host: rule, admits: [decision] },
      { host: memory, admits: [citation] },
    ],
  });

  // The mention-resolution set spells it.
  assert.ok(declaredAddresses(h).has(address), "declaredAddresses spells the nested address");

  // The member table spells it: the edge above resolves only because the table keyed the
  // embedded value under this exact string, and the rendered body is the target facts'
  // own `address` verbatim — three producers, one spelling, compared as bytes.
  const body = emit(h).members.find((member) => member.name === "CLAUDE")?.body;
  assert.equal(body, `See \`${address}\`.\n`);
});
