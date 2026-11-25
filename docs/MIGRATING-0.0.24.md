# Migrating to 0.0.24

**This note covers `0.0.23` to `0.0.24` and nothing earlier.** For the
upgrade before it, read [`MIGRATING-0.0.23.md`](MIGRATING-0.0.23.md).

Every harness needs steps 1 and 2. The rest apply only if you use what they
name. Run `tsc` over your `.temper/` program after upgrading: steps 4 and 6
are compile errors, so the compiler lists every site for you.

## 1. Re-emit and commit the lock

Three things move in the lock.

```text
before (0.0.23)  engine = "0.0.23"
after  (0.0.24)  engine = "0.0.24"

after  (0.0.24)  [[declaration.module]]
                 member = "root"
                 source_path = ".temper/harness.ts"
                 import_hash = "…"

before (0.0.23)  collection_address = { manifest = "settings.json", key_path = "hooks.<Event>", … }
after  (0.0.24)  collection_address = { key_path = "hooks.<Event>", … }
```

The new `[[declaration.module]]` rows fingerprint every module your program
loads that the repository tracks, so editing an imported module now reads
stale on `check` without an `input()`. Gitignored files and packages under
`node_modules` get no row.

**What to do.** Run `temper emit` and commit the lock. A CI step running
`temper emit --frozen` then `git diff --exit-code` fails until you do. An
`input()` you added only to make an imported module count can go.

## 2. Use Node 22.15 or later for `emit`

`emit` records the program's modules through Node's `module.registerHooks`.

```text
before (0.0.23)  emit runs on any supported Node
after  (0.0.24)  temper emit needs Node's module.registerHooks (Node >= 22.15) to fingerprint the program's modules
```

**What to do.** Upgrade Node where you run `emit`, CI included. `check` and
`guard` still need no Node.

## 3. Mark advisory requirements `required: false`

A requirement that omits `required` now inherits the shipped default:
required, at error severity.

```ts
// before (0.0.23): unfilled and silent
"standards": { prose: "…" },

// after (0.0.24): unfilled fails check; to keep it advisory
"standards": { prose: "…", required: false },
```

**What to do.** For each requirement nothing may fill, add
`required: false`. A `required: true` you wrote only to get the floor now
restates the default and can go.

## 4. Rewrite gate hooks with `gateHook()`

`GUARD_COMMAND` and `SESSION_START_COMMAND` are no longer exported, and
`install.gate-installed` judges the gate hook member, not one command
string.

```ts
// before (0.0.23)
import { GUARD_COMMAND, SESSION_START_COMMAND, hook } from "@dtmd/temper/claude-code";
export const hook_sessionStart = hook({
  name: "SessionStart",
  hooks: [{ type: "command", command: SESSION_START_COMMAND }],
});
export const hook_guard = hook({
  name: "PreToolUse",
  matcher: "Write|Edit|MultiEdit",
  hooks: [{ type: "command", command: GUARD_COMMAND }],
});

// after (0.0.24)
import { gateHook } from "@dtmd/temper/claude-code";
export const hook_sessionStart = gateHook({ name: "SessionStart" });
export const hook_guard = gateHook({ name: "PreToolUse", matcher: "Write|Edit|MultiEdit" });
```

With no `binary`, the rendered command is the same bytes as before, so
`settings.json` does not change. If temper lives in a workspace package and
not on your PATH, state it once:

```ts
gateHook({ name: "PreToolUse", matcher: "Write|Edit|MultiEdit", binary: "./tools/node_modules/.bin/temper" })
```

**What to do.** Replace each hook that used a command constant, including
the `PostToolUse` hook on `Bash`. A hook you wrote with a path in its command
string, because the constant exited 127 off the PATH, becomes a `gateHook`
with `binary`.

## 5. Make locus globs placement templates

A file locus glob is now literal segments with at most one `*`, optionally
under a leading `**/`. Anything else is refused when the lock loads, where
it used to match nothing or place members at the wrong path.

```text
kind locus  root "spec"  glob "domains/[a-z0-9-]*.md"
before (0.0.23)  member "example" placed at domains/[a-z0-9-]example.md
after  (0.0.24)  check fails: governs_glob is not a placement template
```

**What to do.** Keep the template and move each other job to its own place:

```ts
locus: {
  kind: "at",
  root: "spec",
  glob: "domains/*.md",
  // a file or a directory beside the members that the kind does not govern
  exclude: ["spec/domains/README.md"],
},
// narrow member names with a clause
clause(allowedChars("name", charset), { severity: "required" }),
```

An `exclude` entry is a literal path the template would match, or a
directory (no trailing `/`) whose contents the kind leaves alone. A pattern
in `exclude` is refused.

## 6. Delete restated kind facts

Three fields that repeated something the kind already said are gone.

```ts
// before (0.0.23)
kind({ name: "invariant", locus: { kind: "embedded" }, registration: [], /* … */ });
kind({ name: "hook", locus: { kind: "at", root: ".claude", glob: "settings.json" },
       collectionAddress: { manifest: "settings.json", keyPath: "hooks.<Event>", entryShape: "…" } });

// after (0.0.24)
kind({ name: "invariant", locus: { kind: "embedded" }, /* … */ });
kind({ name: "hook", locus: { kind: "at", root: ".claude", glob: "settings.json" },
       collectionAddress: { keyPath: "hooks.<Event>", entryShape: "…" } });
```

- `registration` is typed `never` on embedded and nested-file kinds; their
  locus already says they register nothing.
- `collectionAddress.manifest` is gone; the manifest is the kind's starless
  `at` locus.
- A layout `field` region's `slot` must be a key of the kind's typed
  surface, so a slot the member does not carry fails `tsc`.

**What to do.** Delete the fields `tsc` names. Give a layout kind a type
parameter that covers its field slots.

## 7. Check layout kinds

Three layout behaviors changed.

- **Frontmatter now reads.** A `yaml-frontmatter` kind with a body layout
  used to drop its frontmatter fields; `required("package")` over a
  frontmatter field now passes when the field is there.
- **Identity follows the unit shape.** A `named-field` or `directory`
  layout kind keys its member the way its non-layout twin would, where it
  used to use the file stem. Re-emitting rewrites the rows.
- **A layout over `json-document` or `toml-document` is refused** at load.
  A layout types a markdown body.

**What to do.** Remove any workaround for the frontmatter gap, such as a
separate check that read the field. If you made a layout kind `local` only
to quiet the guard (step 8), make it committed again.

## 8. Expect the guard to agree with `check`

The guard now judges with `check`'s site classification, walk and
freshness judges, at both edges. A finding it raises is one `check` raises
on the same tree.

```text
declared layout document spec/domains/example.md, after a Bash call
before (0.0.23)  root.locus-declared: … the lock declares no member for it
after  (0.0.24)  (no finding)
```

It also reports what it used to miss: a moved import, include or input
after a shell call, an undeclared nested layout child, and documents at
`.`-rooted loci. Its walk honors your ignore rules, including a linked
worktree's common `info/exclude`.

**What to do.** Nothing, unless you dialed or relocated something to quiet
a false guard finding. Undo that.

## 9. Rules in subdirectories are governed

Claude Code discovers rules recursively under `.claude/rules/`, and the
built-in `rule` kind now does too. A `CLAUDE.md` under `.claude/rules/`,
`.claude/agents/`, `.claude/commands/` or `.claude/skills/` is no longer a
`memory` member, because Claude Code loads it as that folder's kind.

```text
.claude/rules/frontend/react.md
before (0.0.23)  not governed
after  (0.0.24)  rule member "react", judged like any rule
```

**What to do.** Re-emit, then read `check`'s findings for the newly governed
rules.
