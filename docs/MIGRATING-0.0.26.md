# Migrating to 0.0.26

**This note covers `0.0.25` to `0.0.26` and nothing earlier.** For the
upgrade before it, read [`MIGRATING-0.0.25.md`](MIGRATING-0.0.25.md).

Every harness with a `.temper/` program needs step 1, and step 2 for
whatever the compiler then reports. The rest apply only if `check` or
`emit` reports what they name.

## 1. Bring the program to the 0.0.26 shape and re-emit

Three changes land together, and `emit` writes nothing until all three
hold:

- `.temper/package.json` declares its package manager. Without it, `check`
  refuses with `temper::tool::no_package_manager`. npm and pnpm are read.
- The program declares the typecheck emit runs before it compiles the
  payload. Without one, emit refuses with
  `temper::drift::typecheck_undeclared`.
- The program default-exports its harness. `emit` moved to the
  `@dtmd/temper/pipe` subpath, which `temper emit` drives, so a 0.0.25 tail
  fails with "does not provide an export named 'emit'".

```ts
// before (0.0.25), .temper/harness.ts
import { emit, harness } from "@dtmd/temper";

const program = harness({
  members: [/* … */],
});

process.stdout.write(emit(program).seam);
```

```ts
// after (0.0.26), .temper/harness.ts
import { harness, input, tool } from "@dtmd/temper";

const tsc = tool({ package: "typescript", bin: "tsc" });

const program = harness({
  typecheck: { tool: tsc, project: input(import.meta.url, "./tsconfig.json") },
  members: [tsc, /* … */],
});

export default program;
```

`.temper/package.json` gains two fields:

```text
"devDependencies": { "@types/node": "^22.20.1", "typescript": "^5.6.0" },
"packageManager": "npm@11.6.2"
```

And `.temper/tsconfig.json` is new:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "types": ["node"],
    "strict": true,
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "skipLibCheck": true
  },
  "include": ["**/*.ts"],
  "exclude": ["node_modules"]
}
```

The typecheck's tool must be one of the program's members. In place of
declaring it by hand, a program can spread `defaultPatterns()` from
`@dtmd/temper/patterns`, which composes the typecheck, its `tsc` tool, the
three gate hooks, and the harness reference skill at
`.claude/skills/temper/`. Drop your own gate hooks if you do, and rename
any skill of yours named `temper`.

**What to do.** Add the two fields and the `tsconfig.json`, write the
version your package manager reports into `packageManager`, and run its
install in `.temper/` so the lockfile records `typescript` (otherwise
`check` reports `tool.unlocked`). Then run the compiler once yourself:

```sh
.temper/node_modules/.bin/tsc --noEmit -p .temper/tsconfig.json
```

A 0.0.25 lock records no typecheck, so the first emit runs the program
before the compiler, and a 0.0.25 `hook({ name })` refuses at runtime,
naming the move to `event`, before `tsc` runs. Fix what it reports (step 2),
then run `temper emit` and commit the lock, `.claude/settings.json`, and the
new `.claude/rules/temper-sdk.md` and `temper-guidance-<kind>.md` rules emit
now writes. Until that emit, `check` reports `root.engine-matches`: the
0.0.25 lock still reads, its snake-case predicates and old built-in rows
included.

## 2. Apply the renames the compiler names

Each 0.0.25 spelling below fails `tsc`. The hook renames, `binary` and
`statusLine` in `residue` fail with a remedy that names the new spelling;
the rest are ordinary type errors.

```text
before (0.0.25)                               after (0.0.26)
hook({ name: "PostToolUse", … })              hook({ event: "PostToolUse", … })
gateHook({ name: "PreToolUse", matcher })     gateHook({ event: "PreToolUse", matcher })
gateHook({ …, binary: "scripts/temper" })     gateHook({ …, tool: tool({ file: "scripts/temper" }) })
relocate(k, { governs: { root, glob } })      relocate(k, { locus: { root, glob } })
memoryAnthropicDefaultContract                memoryDefaultContract
when(guard, body)                             when(guard, body, { severity: "required" })
edgeFields: [{ field, to: ["source"] }]       edgeFields: [{ field, to: [source] }]
requirement({ kind: "skill", … })             requirement({ kind: skill, … })
settings({ residue: { statusLine } })         settings({ statusLine })
permissionUnion(needs)  // string[]           permissionUnion(needs).allow  // { allow, ask }
```

A `tool(...)` a gate hook names is a member like the typecheck's: compose
it in `members`. An edge to a kind declared later in the module, or to
the kind itself, names it through a getter:

```ts
edgeFields: [{ field: "superseded-by", get to() { return [decision] as const; } }],
```

An embedded kind declared with `registration` now fails `tsc` too
("an embedded locus registers nothing"); delete the field.

**What to do.** Apply each rename, re-run the compiler until it is clean,
then `temper emit`.

## 3. Expect new gate commands

A gate hook or tap that names no tool now runs the program's own `temper`
bin, anchored at the project root, where a 0.0.25 hook with no `binary`
ran `temper` on the PATH.

```text
before (0.0.25)  cd "${CLAUDE_PROJECT_DIR:-.}" || exit 1; command -v temper >/dev/null 2>&1 || { …; exit 127; } && temper guard .
after  (0.0.26)  cd "${CLAUDE_PROJECT_DIR:-.}" || exit 1; command -v ./.temper/node_modules/.bin/temper >/dev/null 2>&1 || { …; exit 127; } && ./.temper/node_modules/.bin/temper guard .
```

**What to do.** The step 1 emit rewrites `.claude/settings.json`; commit
it. Until then, `install.gate-installed` reports the old commands. A
0.0.25 `binary: ".temper/node_modules/.bin/temper"` is now the default,
so drop it. To keep `temper` on the PATH, compose
`tool({ system: "temper" })` and pass it as `tool`. An unrepresented
harness's install still writes `temper` on the PATH.

## 4. Rename dial entries and matched finding ids

A dial entry keys its clause as `id`, and every predicate segment of a
clause id is kebab case. A `label`-keyed entry is refused by the dial's
closed key set.

```toml
# before (0.0.25), .temper/dial.toml
[[clause]]
label = "skill.max_len.description"
severity = "advisory"

# after (0.0.26)
[[clause]]
id = "skill.max-len.description"
severity = "advisory"
```

Five finding ids are renamed. Anything that matches findings by id (a
dial entry, a CI filter, a SARIF rule mapping) moves with them.

| 0.0.25 | 0.0.26 |
| --- | --- |
| `graph.route` | `root.resolves` |
| `graph.directive-unbacked` | `root.import-backed` |
| `requirement.unfilled` | `requirement.count` |
| `requirement.dangling` | `requirement.resolves` |
| `kind.governs-collision` | `kind.locus-collision` |

**What to do.** Rename `label` to `id` in each `[[clause]]` entry, respell
`max_len`, `min_len`, `forbidden_keys`, `allowed_chars`,
`require_sections`, `must_define` and `section_contains` with hyphens, and
update any id from the table.

## 5. Pass emit's workspace positionally

`temper emit` takes its workspace the way `check` and `bundle` take
theirs. `--into` is removed.

```text
before (0.0.25)  temper emit --into examples/demo/.temper
after  (0.0.26)  temper emit examples/demo/.temper
```

**What to do.** Update scripts and CI jobs that pass `--into`.

## 6. Answer the members the built-in kinds now claim

The built-in kinds read the tree the way Claude Code loads it. `check` now
discovers:

- skills, rules and output styles under any subdirectory's own `.claude/`
  folder, such as `apps/web/.claude/skills/deploy/SKILL.md`
  (`skill:apps:web:deploy`);
- commands in subfolders of `.claude/commands/`
  (`command:frontend:component`);
- every `AGENTS.md` and `.claude/AGENTS.md`, as memory behind the runtime's
  absence gate;
- every `CLAUDE.local.md`, as the `memory-local` kind, judged in place and
  never emitted.

A tree that holds harness files as test data, such as a fixtures folder,
now draws findings for them: a required finding where a fixture breaks a
default contract, and `root.locus-declared` on a represented harness.

**What to do.** Declare a real member in the program and re-emit. To leave
a fixture tree ungoverned, exclude it through the kind's relocation and
build that kind's members from the relocated value:

```ts
import { relocate } from "@dtmd/temper";
import { skill as builtinSkill, type Skill } from "@dtmd/temper/claude-code";

export const skill = relocate<Skill>(builtinSkill, { exclude: ["tests/fixtures"] });
```
