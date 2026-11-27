/**
 * The member-address grammar — every writer's output read back by the one parser, and
 * the one qualified nested spelling three independent producers must agree on. Twenty-one
 * inline template literals could only assert that agreement by inspection; a single writer
 * makes it structural, and this file pins that it stayed so.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { blocks, embeddedMemberValue, harness, kind, text, tool } from "../src/index.js";
import { emit } from "../src/emit.js";
import { gateHook, memory, rule, skill, supportingDoc } from "../src/claude-code.js";
import { declaredAddresses } from "../src/declarations.js";
import { extend, isRootHosted } from "../src/kind.js";
import type { ToolFields } from "../src/builtins.js";
import {
  type AddressedMember,
  bareLookupKey,
  edgeLookupKey,
  hostAddress,
  leafAddress,
  memberAddress,
  nestedAddress,
  parseHostAddress,
  parseLeafAddress,
  parseNestedAddress,
  isNameQualifier,
  isOneSegment,
} from "../src/member-address.js";

/** A grammar-only member, judged by no member model: never root-hosted. */
const bare = (member: AddressedMember): string => memberAddress(member, () => false);

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

test("one segment is non-empty and carries no separator", () => {
  for (const spelling of ["on-enter", "rejected.baked.because", "a:b", "x"]) {
    assert.equal(isOneSegment(spelling), true, `\`${spelling}\` is one segment`);
  }
  for (const spelling of ["", "authority/rejected", "/", "a/"]) {
    assert.equal(isOneSegment(spelling), false, `\`${spelling}\` is not one segment`);
  }

  // Why the predicate exists, stated against the writer and the reader it sits between: a
  // key that is not one segment does not come back out of the reader as the member it was
  // written for. It reads at *leaf* grain instead — as the `rejected` leaf of the sibling
  // keyed `authority`, the same address one grain down.
  const shifted = nestedAddress(hostAddress("spec", "alpha"), "decision", "authority/rejected");
  assert.equal(parseNestedAddress(shifted), undefined);
  assert.deepEqual(parseLeafAddress(shifted), {
    member: "spec:alpha",
    kind: "decision",
    key: "authority",
    childPath: "rejected",
  });
});

test("a name qualifier admits the empty spelling and refuses the separator", () => {
  // The qualifier bar is `isOneSegment`'s, less its non-emptiness: a hook whose `matcher`
  // is `""` is its own group on the wire, and the name it joins onto is non-empty
  // whatever the qualifier holds.
  for (const spelling of ["", "Edit|Write", "Edit, Write", "*", ".*", "a:b"]) {
    assert.equal(isNameQualifier(spelling), true, `\`${spelling}\` qualifies a name`);
  }
  assert.equal(isOneSegment(""), false, "the empty spelling is no key");
  for (const spelling of ["a/b", "/", "Edit|Write/"]) {
    assert.equal(isNameQualifier(spelling), false, `\`${spelling}\` carries the separator`);
  }

  // Why: the joined name round-trips as one name, and its handlers address beneath it.
  const joined = hostAddress("hook", "PostToolUse:Edit|Write");
  assert.deepEqual(parseHostAddress(joined), { kind: "hook", name: "PostToolUse:Edit|Write" });
  assert.deepEqual(parseNestedAddress(nestedAddress(joined, "handler", "0")), {
    host: joined,
    kind: "handler",
    key: "0",
  });

  // With a separator in it, every segment below shifts and the handler's address reads at
  // leaf grain, naming no member at all.
  const shifted = nestedAddress(hostAddress("hook", "PostToolUse:a/b"), "handler", "0");
  assert.equal(parseNestedAddress(shifted), undefined);
  assert.deepEqual(parseLeafAddress(shifted), {
    member: "hook:PostToolUse:a",
    kind: "b",
    key: "handler",
    childPath: "0",
  });
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

  // The leaf path is the address's final segment, its own dots intact — a child path
  // joins its layers with `.`, so a further `/` is a further segment and the count reads
  // the whole as a member address instead.
  assert.equal(parseLeafAddress("spec:20/decision/authority/rejected.baked.because")?.childPath, "rejected.baked.because");
  assert.equal(parseLeafAddress("spec:20/decision/authority/a/b"), undefined, "five segments is member grain");

  // The bare member head a lock committed before the writers spelled the canonical one
  // still parses at leaf grain — the reader accepts it, nothing writes it.
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
  return kind<object>({ name: "decision", locus: { kind: "embedded" }, unitShape: "file" });
}

/** An embedded kind whose `source` edge renders the target facts' own address verbatim. */
function citationKind() {
  return kind<object>(
    {
      name: "citation",
      locus: { kind: "embedded" },
      unitShape: "file",
      edgeFields: [{ field: "source", to: [decisionKind()] }],
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

test("a member's own address takes its host when it has one, and its kind alone when it does not", () => {
  // A top-level member's identity is its bare name, so its address is its kind joined on.
  assert.equal(bare({ kind: "rule", name: "collaboration" }), "rule:collaboration");

  // A nested **file** child's identity *is* its whole `<host-address>/<kind>/<key>`
  // address, exactly as an embedded member's is — the host segment is the whole of what
  // tells two same-named children under different hosts apart.
  const alpha = bare({ kind: "supporting-doc", name: "home", host: { kind: "skill", name: "alpha" } });
  const beta = bare({ kind: "supporting-doc", name: "home", host: { kind: "skill", name: "beta" } });
  assert.equal(alpha, nestedAddress(hostAddress("skill", "alpha"), "supporting-doc", "home"));
  assert.equal(alpha, "skill:alpha/supporting-doc/home", "one layer is spelled byte for byte as it was");
  assert.notEqual(alpha, beta, "two hosts carrying one name is two addresses, never one twice");

  // And what it composes is the nested grain the one parser reads back — never a fourth
  // segment, and never a bare `kind:name` a sibling host also spells.
  assert.deepEqual(parseNestedAddress(alpha), {
    host: "skill:alpha",
    kind: "supporting-doc",
    key: "home",
  });
});

test("a tool's one address is the root-hosted nested member, wherever a row names it", () => {
  // The root hosts its tools, so a tool's own address is `root/tool/<name>` — the spelling
  // `check` resolves it at — and never a second `tool:<name>` beside it.
  const lint = tool({ name: "lint", system: "eslint" });
  assert.equal(memberAddress(lint, isRootHosted), nestedAddress("root", "tool", "lint"));
  assert.equal(memberAddress(lint, isRootHosted), "root/tool/lint");
  assert.deepEqual(parseNestedAddress(memberAddress(lint, isRootHosted)), { host: "root", kind: "tool", key: "lint" });

  // The `runs` edge and the mention set spell it by that one address.
  const gate = gateHook({ event: "SessionStart", tool: lint });
  const h = harness({ members: [lint, gate] });
  assert.ok(declaredAddresses(h).has("root/tool/lint"), "the mention set carries the root-hosted spelling");
  assert.ok(!declaredAddresses(h).has("tool:lint"), "and no `tool:<name>` beside it");
  const runs = emit(h).declarations.runs.map((row) => row.tool);
  assert.ok(runs.length > 0 && runs.every((address) => address === memberAddress(lint, isRootHosted)));
});

test("a tool subkind's member is root-hosted: addressed at root/<kind>/<key> and rowed as the root's nested member", () => {
  // Root-hosting closes over the base chain, as the engine's `root_hosted_kinds` reads the
  // kind rows' `base` column — so a subkind of a subkind still answers for `tool`.
  const linter = extend<ToolFields, ToolFields>(tool, { name: "linter" });
  const strict = extend<ToolFields, ToolFields>(linter, { name: "strict-linter" });
  const lint = linter({ name: "lint", system: "eslint" });
  const deep = strict({ name: "deep", system: "eslint" });
  assert.equal(memberAddress(lint, isRootHosted), "root/linter/lint");
  assert.equal(memberAddress(deep, isRootHosted), "root/strict-linter/deep");

  const gate = gateHook({ event: "SessionStart", tool: lint });
  const h = harness({ members: [lint, deep, gate] });
  const declared = declaredAddresses(h);
  assert.ok(declared.has("root/linter/lint") && !declared.has("linter:lint"));
  const { declarations } = emit(h);
  const rows = declarations.nested_members.filter((row) => row.host === "root");
  assert.ok(rows.some((row) => row.kind === "linter" && row.key === "lint"), "the subkind's member is the root's row");
  assert.ok(rows.some((row) => row.kind === "strict-linter" && row.key === "deep"));
  assert.deepEqual(
    declarations.runs.map((row) => row.tool),
    ["root/linter/lint"],
    "a gate hook takes a tool subkind's member, and its runs edge names the root-hosted address",
  );
});

test("a member's own address composes its host's, to whatever depth the model nests", () => {
  // The host contributes its *own whole address*, so a two-layer child carries both hosts
  // and the grandparent survives. Composed members satisfy the shape recursively, which is
  // the only reason a depth the SDK's own factories already build can be addressed at all.
  const demo = skill({ name: "demo", description: "Use when demonstrating." });
  const outer = supportingDoc({ name: "outer", host: demo, prose: text`# Outer` });
  const inner = supportingDoc({ name: "inner", host: outer, prose: text`# Inner` });

  assert.equal(memberAddress(outer, isRootHosted), "skill:demo/supporting-doc/outer");
  assert.equal(memberAddress(inner, isRootHosted), "skill:demo/supporting-doc/outer/supporting-doc/inner");

  // Flattening the host to `<kind>:<name>` lost the grandparent, and keyed two cousins —
  // one name under each of two outers — as one member.
  const sibling = supportingDoc({ name: "other", host: demo, prose: text`# Other` });
  const cousin = supportingDoc({ name: "inner", host: sibling, prose: text`# Cousin` });
  assert.notEqual(memberAddress(cousin, isRootHosted), memberAddress(inner, isRootHosted), "two hosts, two addresses");

  // The declaration rows key by this very address: the mention-resolution set is where the
  // engine's reader meets it, and it spells both layers.
  const addresses = declaredAddresses(harness({ members: [demo, outer, inner, sibling, cousin] }));
  for (const member of [demo, outer, inner, sibling, cousin]) {
    assert.ok(addresses.has(memberAddress(member, isRootHosted)), `declaredAddresses spells \`${memberAddress(member, isRootHosted)}\``);
  }
});

test("the segment count decides the grain at any depth, host carried verbatim", () => {
  // Odd is member grain, even is leaf grain — the one discrimination, and the two grains
  // are disjoint by construction rather than by a reader's precedence.
  const grains = [
    ["skill:a/hook/on-enter", "member"],
    ["skill:a/hook/on-enter/command", "leaf"],
    ["area:a/page/b/leaf/k", "member"],
    ["area:a/page/b/leaf/k/body", "leaf"],
    ["area:a/page/b/leaf/k/deep/d", "member"],
  ] as const;
  for (const [address, grain] of grains) {
    assert.equal(parseNestedAddress(address) !== undefined, grain === "member", `\`${address}\` is ${grain} grain`);
    assert.equal(parseLeafAddress(address) !== undefined, grain === "leaf", `\`${address}\` is ${grain} grain`);
  }

  // Two layers down, each grain hands back the host whole: never re-split at a first
  // colon, which would take `area` / `a/page/b` out of `area:a/page/b`.
  assert.deepEqual(parseNestedAddress("area:a/page/b/leaf/k"), {
    host: "area:a/page/b",
    kind: "leaf",
    key: "k",
  });
  assert.deepEqual(parseLeafAddress("area:a/page/b/leaf/k/body"), {
    member: "area:a/page/b",
    kind: "leaf",
    key: "k",
    childPath: "body",
  });

  // The host is itself a member address all the way down, so a deep spelling whose
  // innermost head is no host address names nothing at either grain.
  assert.equal(parseNestedAddress("a/b/c/d/e"), undefined);
  assert.equal(parseLeafAddress("a/b/c/d/e"), undefined);
});
