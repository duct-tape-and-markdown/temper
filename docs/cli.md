# The command line

temper ships one binary with eight verbs. One of them gates, one of them
reads, and the rest set up or compile the surface those two work over. The
surface is pre-1.0 and still moving; `temper <verb> --help` is always current
for your installed version, and the operational definition of each verb lives
in [`specs/architecture/20-surface.md`](../specs/architecture/20-surface.md).

## check

The gate. Judges the harness against the active contract and exits nonzero
when a required clause fails. Runs offline, over committed files, with no
language runtime.

The zero-config form points at any repo with a `.claude/` and needs no
workspace, no project file, no import step:

```sh
temper check --harness .
```

With a workspace argument it gates a temper surface instead. `--reporter`
switches the output format for machines (GitHub Actions annotations, SARIF)
without changing the verdict, and `--deny-advisories` fails on advisory
findings too, the strict CI posture.

## explain

The read verb. `check` already computes a graph of the harness: members,
the requirements they fill, the references between them. `explain` narrates
one corner of it:

```sh
temper explain review-rules
```

For a member you get its forward references and what breaks if it goes away.
For a requirement you get what fills it and how covered it is. A name that
matches more than one thing is an error listing the qualified spellings
(`member:review-rules`, `requirement:review-rules`), and a qualified name is
always accepted directly. It reads, never gates: exit zero on every input.

## init

The on-ramp. Scans an existing harness and writes a config skeleton over its
members in place: no file moves, no copied tree, your prose untouched.
`--lift <member>` later migrates a single member into a richer authoring
carriage, one at a time, at your pace.

## emit

The compiler. Re-emits every projected artifact whole from the authored
surface, byte-deterministically, and records what it wrote in the lock.
Emitting twice produces identical bytes or fails loudly; nondeterminism is
never silent churn. `--dry-run` reports every projection without writing,
and `--frozen` is the CI posture.

## schema

Emits the active contract as an editor JSON Schema, so the same clauses that
gate in CI also complain at keystroke time, with the guidance as hover text.
`--kind skill` narrows to one kind.

## guard

A `PreToolUse` hook body. Reads the tool-call payload on stdin, and when the
write targets a generated `.claude/` file, either informs and routes to the
authored source (the shared posture, advisory) or blocks (the surface
posture). The posture is yours to declare; temper never escalates on its own.
You rarely run this by hand: `install` wires it.

## install

Wires temper's own gate into the harness: the session-start report hook, the
guard at the write boundary, and the managed header lines in each artifact.
Idempotent, so re-running repairs anything that was deleted. CI wiring is
deliberately not a placement; the recommended job is two lines you author
yourself.

## bundle

Composes the surface into an installable Claude Code plugin plus a
`marketplace.json`, for distributing the gate through the native plugin
rails. Deterministic: re-running reproduces an identical tree.
