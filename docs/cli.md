# The command line

temper ships one binary with seven verbs. One of them gates, one of them
reads, and the rest set up or compile the surface those two work over. The
surface is pre-1.0 and still moving; `temper <verb> --help` is always current
for your installed version, and the operational definition of each verb lives
in the spec corpus: [`specs/model/pipeline.md`](../specs/model/pipeline.md)
for the pipeline verbs and
[`specs/model/contract.md`](../specs/model/contract.md) for the gate and the
read side.

## check

The gate. Judges the harness against the active contract and exits nonzero
when a required clause fails. Runs offline, over committed files, with no
language runtime.

The zero-config form points at any repo with a `.claude/` and needs no
workspace, no project file, no import step:

```sh
temper check --harness .
```

With a workspace argument (default `./.temper`) it gates a temper surface
instead. `--reporter` switches the output format for machines (GitHub Actions
annotations, SARIF) without changing the verdict, and `--deny-advisories`
fails on advisory findings too, the strict CI posture.

One reporter is special: `--reporter session-start` is the advisory form a
Claude Code `SessionStart` hook runs. It reads the path as a harness root,
always exits zero, and surfaces a failing contract to the session for
approval instead of blocking it.

## explain

The read verb. `check` already computes a graph of the harness: members, the
requirements they fill, the references between them. `explain` narrates one
corner of it:

```sh
temper explain review-rules
```

For a member you get its forward references, its blast radius, and its
neighborhood. For a requirement you get what fills it and how covered it is.
For a leaf address you get its citations. A name that matches more than one
thing is an error listing the qualified spellings (`member:review-rules`,
`requirement:review-rules`), and a qualified name is always accepted
directly. It reads, never gates: exit zero on every input.

## emit

The compiler. Re-emits every projected artifact whole from the authored
surface, byte-deterministically, and records what it wrote in the lock.
Emitting twice produces identical bytes or fails loudly; nondeterminism is
never silent churn. `--dry-run` reports every projection without writing,
and `--frozen` is the CI posture.

## schema

Emits the active contract as an editor JSON Schema, so the same clauses that
gate in CI also complain at keystroke time, with the guidance as hover text.
`--kind skill` narrows to one kind; omitted, you get every modeled kind keyed
by name.

## guard

A `PreToolUse` hook body. Reads the tool-call payload on stdin, and when the
write targets a file the lock names as a projection, acts per the declared
enforcement mode: `note` allows and defers, `warn` allows and surfaces
in-band, `block` denies. The mode is read live from the harness's lock;
temper never escalates on its own, and a harness with no lock reads the
default `warn`. You rarely run this by hand: `install` wires it.

## install

The one on-ramp. It opens with a discovery report (what the walk finds,
members by kind, what the built-in contract says about them), then asks one
question: represent this project as a temper program?

Answering no (`--no-represent`) wires the `SessionStart` reporter alone, a
footprint of one settings entry, Node-free. Answering yes (`--yes`) scaffolds
the SDK program: it ensures the `@dtmd/temper` dependency, converts each
discovered artifact into a typed member module with your prose byte-faithful,
runs the first emit, and places the guard and the schema wiring the fresh
lock justifies. `--dry-run` reports every placement without writing.
Re-running converges; CI wiring is deliberately not a placement, the
recommended job is two lines you author yourself.

## bundle

Composes the surface into an installable Claude Code plugin plus a
`marketplace.json`, for distributing the gate through the native plugin
rails. Deterministic: re-running reproduces an identical tree. `--out` names
the plugin directory (default `./plugin`).
