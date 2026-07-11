# @dtmd/temper

_A type system for the documents that program agents._

One package, two faces: the `temper` CLI (a prebuilt binary that needs no
runtime once it is on disk) and the typed SDK for authoring a Claude Code
harness as a program. The full story, the CLI reference, and the spec corpus
live in the [repository](https://github.com/duct-tape-and-markdown/temper).

## Check a harness

Point it at any repo with a `.claude/`. No config, no project file, nothing
installed:

```sh
npx @dtmd/temper check --harness .
```

It validates every skill, rule, and agent against the documented Anthropic
schemas and best practices, then reports what is malformed, what Claude Code
silently ignores, and what a requirement you declared would strand. Every
finding arrives with its guidance attached.

## Wire the gate into a project

```sh
npx @dtmd/temper install
```

`install` opens with a report of what it finds in your harness, then asks one
question: represent it as a temper program? Answering no wires the advisory
session-start report alone, one settings entry. Answering yes converts each
discovered artifact into a typed member module, your prose byte-for-byte
intact, and runs the first emit.

## Author the harness as a program

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

`temper emit` compiles the program into the projected `.claude/**` files and
a lock, byte-for-byte reproducible; it verifies itself by emitting twice.
`temper check` gates against the lock, so it can answer what fills each
requirement and what would strand it. A hand edit to a generated file
surfaces as drift routed to its authored source, never merged around. The SDK
implements no semantics: every type erases at the seam, and the engine
consumes only declared data, offline, no Node.

## Documentation

- [CLI reference](https://github.com/duct-tape-and-markdown/temper/blob/main/docs/cli.md),
  the seven verbs
- [How it works](https://github.com/duct-tape-and-markdown/temper/blob/main/docs/how-it-works.md),
  the model in plain words
- [Why it exists](https://github.com/duct-tape-and-markdown/temper/blob/main/specs/intent.md)

## Platforms

Prebuilt binaries ship for Linux and Windows (x64), macOS next. On other
platforms, build from source with a Rust 1.96+ toolchain (`cargo install
--path .` from a clone of the repository).

## License

Dual-licensed under **MIT OR Apache-2.0**. You may use `temper` under the
terms of either.
