/**
 * Mention targets — the resolution set a citation edge names, and the adapter
 * that spells a member as one. A mention may target a top-level member
 * (`kind:name`) or an embedded member (the host-scoped
 * `<host-kind>:<host-name>/<kind>/<key>` address); both must resolve, or the
 * emit refuses the mention as dangling.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import { blocks, embeddedMemberValue, emit, harness, kind, mentionOf, renderText, text } from "../src/index.js";
import { memory, rule, skill, supportingDoc } from "../src/claude-code.js";

/** The `decision` embedded kind the host below nests — host-free, admitted over `memory` per harness. */
const memoryDecision = kind<Record<never, never>>({
  name: "decision",
  locus: { kind: "embedded" },
  unitShape: "file",
});

/** The admission letting a `memory` host compose a `decision` — every `blocks()` harness below declares it. */
const admitDecision = { host: memory, admits: [memoryDecision] };

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

test("a mention of a nested skill displays its invocation spelling while its address stays the identity", () => {
  const nested = skill({ name: "apps:web:deploy", description: "Deploy the web app.", prose: text`# Deploy` });
  assert.deepEqual(mentionOf(nested), { address: "skill:apps:web:deploy", display: "/apps/web:deploy" });
  // A root skill's identity is its whole name, and its display keeps it.
  const root = skill({ name: "deploy", description: "Deploy.", prose: text`# Deploy` });
  assert.deepEqual(mentionOf(root), { address: "skill:deploy", display: "deploy" });
});

test("a mention targeting an embedded member's host-scoped address resolves without a dangling refusal", () => {
  const citer = rule({
    name: "citations",
    prose: text`It upholds ${{ address: "memory:CLAUDE/decision/done-is-exact", display: "done-is-exact" }}.`,
  });
  const h = harness({
    members: [hostWithEmbeddedMember(), citer],
    admit: [admitDecision],
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
    admit: [admitDecision],
  });

  assert.throws(() => emit(h), /a mention cannot dangle/);
});

test("a dangling mention in a composed-body prose span refuses at emit, like a member-level text body", () => {
  const h = harness({
    members: [
      memory({
        name: "CLAUDE",
        prose: blocks(text`It rests on ${{ address: "agent:ghost", display: "ghost" }}.`),
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

test("a name-less `kind:` refuses at emit even when the kind is deferrable — it names no member to discover", () => {
  // `source` is declared and deferrable, but `source:` names no member of it: a
  // discovery locus is not itself a mention target, so there is nothing for `check`
  // to resolve and the refusal is emit's.
  const citer = rule({
    name: "citations",
    prose: text`It rests on ${{ address: "source:", display: "the sources" }}.`,
  });
  const h = harness({
    members: [citer],
    expect: [{ kind: sourceKind, clauses: [] }],
  });

  assert.throws(() => emit(h), /a mention cannot dangle/);
});

test("an unresolved nested-shaped address refuses at emit even under a deferrable head kind", () => {
  // `source` is deferrable, but a `<host>/<kind>/<key>` address names an embedded member,
  // which is composed in the program and never discovered on disk: nothing to defer to.
  const citer = rule({
    name: "citations",
    prose: text`It rests on ${{ address: "source:main/hook/on-enter", display: "on-enter" }}.`,
  });
  const h = harness({
    members: [citer],
    expect: [{ kind: sourceKind, clauses: [] }],
  });

  assert.throws(() => emit(h), /a mention cannot dangle/);
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
    admit: [admitDecision],
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

test("a leaf mention row heads its host's own address, at whatever grain the host sits", () => {
  // The leaf branch composes the same `memberAddress` every other row site does, so a
  // top-level host heads `kind:name` and a nested-file host heads its whole address. A
  // bare name would head a string no member answers to — a nested child's identity *is*
  // its host-qualified address — and would collapse two sibling `home`s into one head.
  const alpha = skill({ name: "alpha", description: "Use when alpha." });
  const beta = skill({ name: "beta", description: "Use when beta." });
  const cited = rule({ name: "rust", prose: text`# Rust` });
  const leafMention = (key: string) =>
    embeddedMemberValue({
      kind: memoryDecision,
      key,
      leaves: { chosen: text`See ${{ address: "rule:rust", display: "rust" }}.` },
    });

  const result = emit(
    harness({
      members: [
        cited,
        alpha,
        beta,
        memory({ name: "CLAUDE", prose: blocks(leafMention("top")) }),
        supportingDoc({ name: "home", host: alpha, prose: blocks(leafMention("under-alpha")) }),
        supportingDoc({ name: "home", host: beta, prose: blocks(leafMention("under-beta")) }),
      ],
      admit: [admitDecision, { host: supportingDoc, admits: [memoryDecision] }],
    }),
  );

  assert.deepEqual(
    result.declarations.mentions.map((row) => row.member),
    [
      "memory:CLAUDE/decision/top/chosen",
      "skill:alpha/supporting-doc/home/decision/under-alpha/chosen",
      "skill:beta/supporting-doc/home/decision/under-beta/chosen",
    ],
  );

  // Two sibling children of one name head two distinct strings — the host segment is the
  // whole of what tells them apart, so neither leaf answers to the other's address.
  const heads = result.declarations.mentions
    .map((row) => row.member)
    .filter((member) => member.includes("/supporting-doc/home/"));
  assert.equal(new Set(heads).size, 2, `two hosts, two heads: ${heads.join(", ")}`);
});

test("a resolved edge target carries both host-relative and repo-rooted path facts", () => {
  const edgeKind = kind<object>(
    {
      name: "link",
      locus: { kind: "embedded" },
      unitShape: "file",
      edgeFields: [{ field: "target", to: [rule] }],
    },
    {
      render: (value) => {
        const edge = value.targets.target;
        return `relative: ${edge.path}, rooted: ${edge.repoRootedPath}`;
      },
    },
  );

  const h = harness({
    members: [
      rule({ name: "rust", paths: ["src/**/*.rs"], prose: text`# Rust` }),
      memory({ name: "CLAUDE", prose: blocks(embeddedMemberValue({ kind: edgeKind, key: "ref", leaves: { target: "rule:rust" } })) }),
    ],
    admit: [{ host: memory, admits: [edgeKind] }],
  });

  const result = emit(h);
  const body = result.members.find((m) => m.name === "CLAUDE")!.body;
  assert.equal(body, "relative: .claude/rules/rust.md, rooted: .claude/rules/rust.md\n");
});

test("a path-form mention renders the target's projection path relative to the host's", () => {
  // The skill cites its own supporting doc: the doc is placed under the skill's unit, so
  // the link the body carries is the doc's path read from SKILL.md — never hand-written.
  const placement = skill({ name: "deploy", description: "Use when deploying." });
  const doc = supportingDoc({ name: "reference", host: placement, prose: text`# Reference` });
  const host = skill({
    name: "deploy",
    description: "Use when deploying.",
    prose: text`See [the reference](${mentionOf(doc, { as: "path" })}).`,
  });
  // A doc citing a sibling skill's SKILL.md climbs out of its own unit first.
  const other = skill({ name: "release", description: "Use when releasing.", prose: text`# Release` });
  const citer = supportingDoc({
    name: "notes",
    host: placement,
    prose: text`Then [release](${mentionOf(other, { as: "path" })}).`,
  });

  const result = emit(harness({ members: [host, doc, other, citer] }));
  const body = (name: string) => result.members.find((m) => m.name === name)!.body;
  assert.match(body("deploy"), /See \[the reference\]\(reference\.md\)\./);
  assert.match(body("notes"), /Then \[release\]\(\.\.\/release\/SKILL\.md\)\./);
  // The path form changes the rendering, never the edge: the mention row is the same.
  assert.ok(
    result.declarations.mentions.some(
      (row) => row.member === "skill:deploy" && row.target === "skill:deploy/supporting-doc/reference",
    ),
  );
});

test("mentionOf's path form marks the mention while the name form stays the bare pair", () => {
  const member = rule({ name: "rust", prose: text`# Rust` });
  assert.deepEqual(mentionOf(member, { as: "path" }), {
    address: "rule:rust",
    display: "rust",
    form: "path",
  });
  assert.deepEqual(mentionOf(member, { as: "name" }), { address: "rule:rust", display: "rust" });
});

test("a path-form mention of a member that projects no file refuses at emit, before any write", () => {
  const citer = rule({
    name: "citations",
    prose: text`It upholds ${{
      address: "memory:CLAUDE/decision/done-is-exact",
      display: "done-is-exact",
      form: "path",
    }}.`,
  });
  const h = harness({ members: [hostWithEmbeddedMember(), citer], admit: [admitDecision] });
  assert.throws(
    () => emit(h),
    /member `citations`: path-form mention of `memory:CLAUDE\/decision\/done-is-exact`, which projects no file/,
  );
});

test("a path-form mention deferring to check refuses at emit — a discovered target has no derived placement", () => {
  const citer = rule({
    name: "citations",
    prose: text`It rests on ${{ address: "source:main", display: "main", form: "path" }}.`,
  });
  const h = harness({ members: [citer], expect: [{ kind: sourceKind, clauses: [] }] });
  assert.throws(() => emit(h), /path-form mention of `source:main` names no composed member/);
});

test("a path-form mention in an embedded-member leaf refuses by the leaf's name", () => {
  const cited = rule({ name: "rust", prose: text`# Rust` });
  const h = harness({
    members: [
      cited,
      memory({
        name: "CLAUDE",
        prose: blocks(
          embeddedMemberValue({
            kind: memoryDecision,
            key: "top",
            leaves: { chosen: text`See ${mentionOf(cited, { as: "path" })}.` },
          }),
        ),
      }),
    ],
    admit: [admitDecision],
  });
  assert.throws(
    () => emit(h),
    /member\.decision top: leaf `chosen`: mention of `rule:rust` renders as a path/,
  );
});

test("renderText outside emit refuses a path-form mention rather than spelling its name", () => {
  const cited = rule({ name: "rust", prose: text`# Rust` });
  assert.throws(() => renderText(text`See ${mentionOf(cited, { as: "path" })}.`), /renders as a path/);
});
