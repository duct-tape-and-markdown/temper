# temper

_A type system for the documents that program agents._

`temper` is a CLI that checks a Claude Code harness (the skills, rules,
agents, hooks, and `CLAUDE.md` that program the agent) against a declared
contract. It fills a gap that code linters and portability tools don't cover:
a malformed harness fails silently at runtime, as a skill that never triggers,
a rule Claude Code silently ignores, or a hook command that resolves to
nothing, and you learn about it from the agent's behavior, not from an error
message. temper moves that failure to author-time, the way a type checker
does. Think `tsc`, not ESLint.

A harness is a codebase, and its primary author is increasingly the agent
itself. The contract catches the structural failures agents most commonly
commit, and every finding arrives with its guidance attached, delivered at
the moment of failure to the author who needs it most.

## Usage

Point it at any repo with a `.claude/`. No config, no project file, nothing
installed:

```sh
npx @dtmd/temper check --harness .
```

It validates every skill, rule, and agent against the documented Anthropic
schemas and best practices, then reports what is malformed, what Claude Code
silently ignores, and what a requirement you declared would strand:

![temper check rendering a real diagnostic: a rule authored with Cursor's `globs` key that Claude Code silently ignores](examples/demo/demo.svg)

To wire the gate into a project:

```sh
npx @dtmd/temper install
```

`install` opens with a report of what it finds in your harness, then asks one
question: represent it as a temper program? Answering no wires the advisory
session-start report alone: one settings entry, no runtime needed after
install. Answering yes converts each discovered artifact into a typed member
module, your prose byte-for-byte intact, and runs the first emit.

## Authoring the harness as a program

The same package is the SDK ([`@dtmd/temper`](https://www.npmjs.com/package/@dtmd/temper)).
A harness is a small typed program: members are typed values, composition is
ordinary imports, and requirements are declared next to the members that fill
them:

```ts
import { emit, harness } from "@dtmd/temper";
import { memory_CLAUDE } from "./memory/CLAUDE.ts";
import { rule_collaboration } from "./rules/collaboration.ts";
import { skill_captureFriction } from "./skills/capture-friction.ts";

const program = harness({
  require: {
    "friction-capture-procedure": {
      prose: "an agent that hits harness friction needs a procedure for filing the capture",
      required: true,
    },
  },
  members: [memory_CLAUDE, rule_collaboration, skill_captureFriction],
});

process.stdout.write(emit(program).seam);
```

`emit` compiles the program into the projected `.claude/` files and a lock,
byte-for-byte reproducible; it verifies itself by emitting twice. `check`
gates against the lock, so it can answer what fills each requirement and what
would strand it, coverage a file-by-file linter cannot see. A hand edit to a
generated file surfaces as drift routed to its authored source, never merged
around.

A kind may also declare the layout of a document's body, so a document's
sections and the members inside them are typed, addressable, and under the
same contract as any file.

## Commands

| Command | Description |
| --- | --- |
| `temper check` | Gate the harness against the active contract |
| `temper explain <name>` | Narrate one member or requirement: what fills it, what references it, its blast radius |
| `temper emit` | Compile the authored surface into the projected files and the lock |
| `temper schema` | Emit the active contract as an editor JSON Schema |
| `temper guard` | `PreToolUse` hook body that protects projected files from hand edits |
| `temper install` | Wire the gate into a project |
| `temper bundle` | Compose the surface into a Claude Code plugin plus a `marketplace.json` |

See [`docs/cli.md`](docs/cli.md) for the full reference,
[`docs/how-it-works.md`](docs/how-it-works.md) for the model in plain words,
and [`specs/intent.md`](specs/intent.md) for why the project exists.

## CI and editors

`temper check --reporter github` emits GitHub Actions annotations, inline on
the pull request diff; `--reporter sarif` lands the same verdict in code
scanning. `--deny-advisories` fails the run on advisory findings too, the
strict CI posture. `temper schema` gives your editor the same contract, so
the clauses that gate in CI also complain at keystroke time.

## Install

Prebuilt binaries for Linux and Windows (x64) ship on npm, macOS next; the
binary needs no runtime of any kind once it is on disk. On other platforms,
build from source with a Rust 1.96+ toolchain (`cargo install --path .` from
a clone).

## Status

`temper` is pre-1.0: versions stay `0.x` until the surface stabilizes, so
expect breaking changes. The codebase is largely agent-built under
human-authored specs; see [CONTRIBUTING](.github/CONTRIBUTING.md) for the
two-sided AI-authorship policy.

## License

Dual-licensed under **MIT OR Apache-2.0**, the Rust ecosystem's standard
grant. You may use `temper` under the terms of either.
