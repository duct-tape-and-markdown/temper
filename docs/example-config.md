# Example config: this repository's own harness

The best worked example of a temper surface is the live one in this
repository. The harness you are reading about is authored at
[`.temper/`](../.temper/) and gated by `temper check .` on every
session start. Every path on this page is real. The excerpts are copies
made for reading flow, and nothing gates this page against the tree, so if
a quote and its file ever disagree, the file is right and this page has a
bug.

## The shape

```
.temper/
  harness.ts          # the program: members, requirements, the emit call
  lock.toml           # tool-written: provenance, fingerprints, declarations
  memory/CLAUDE.ts    # the CLAUDE.md member, prose in CLAUDE.md beside it
  rules/*.ts          # one module per rule, prose in a .md file beside each
  skills/*.ts         # same pattern for skills
```

Each member is a typed value in its own module. A rule looks like this
([`.temper/rules/rust.ts`](../.temper/rules/rust.ts)):

```ts
import { file, rule } from "@dtmd/temper/claude-code";

export const rule_rust = rule({
  name: "rust",
  paths: ["src/**/*.rs", "tests/**/*.rs", "benches/**/*.rs"],
  prose: file(import.meta.url, "./rust.md"),
});
```

The prose stays in a markdown file beside the module and lands in the
projection byte for byte. The fields are typed, so a key Claude Code would
silently ignore is a compile error at the keystroke, before the gate ever
runs.

## The program

[`.temper/harness.ts`](../.temper/harness.ts) composes the members and
declares what the harness must contain:

```ts
const program = harness({
  require: {
    "pending-entry-discipline": {
      prose: "flume's plan phase needs pending.json entry-filing constraints available as a rule scoped to .flume/plan/pending.json",
      kind: rule,
      required: true,
    },
    // ...
  },
  members: [memory_CLAUDE, rule_collaboration, /* ... */ skill_captureFriction],
});
```

A requirement carries its intent in prose and is filled when a member's
satisfies edge names it. Delete the member that fills a required requirement
and `emit` refuses before writing a byte.

## The projection and the gate

`temper emit` compiles the program into the committed artifacts (`CLAUDE.md`,
`.claude/rules/*.md`, `.claude/skills/*`) plus
[`.temper/lock.toml`](../.temper/lock.toml). From then on the projected files
are owned by the program: a hand edit is drift, detected by hash and routed
back to the owning module. The wiring in
[`.claude/settings.json`](../.claude/settings.json) closes the loop:

- a `SessionStart` hook runs `temper check . --reporter session-start`, the
  advisory report at session open;
- a `PreToolUse` hook runs `temper guard .`, which routes any agent write
  aimed at a projection back to the authored source.

To see the gate's output on a harness with real findings, run the checked-in
demo: `temper check --harness examples/demo/harness`.
