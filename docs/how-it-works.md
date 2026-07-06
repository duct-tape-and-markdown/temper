# How temper works

A Claude Code harness accumulates. Skills, rules, agents, hooks, settings,
plugin manifests. Claude Code reads all of it at session start and tells you
almost nothing about what it found: a frontmatter key it doesn't recognize is
skipped, a rule scoped with the wrong glob key never loads, a hook that
references a missing file fails quietly at runtime. The files look fine in
review. The behavior is just missing.

temper treats the harness as a typed artifact instead of a pile of files, and
checks it the way a compiler checks a program.

## The model

When temper reads a harness, every file becomes a **member** of some **kind**.
A skill is a kind, so is a rule, an agent, a hook definition. The kind knows
where its members live, what fields they carry, and how Claude Code registers
them at runtime.

Each kind carries a contract: a set of **clauses**. A clause is one decidable
statement about a member, with a severity (required or advisory), a piece of
guidance, and a citation to the documented source it came from. Guidance is
teaching prose that lives on the clause itself, the rationale a predicate
cannot encode, and it reaches you wherever the clause does: as hover
documentation in an editor wired to the emitted schema, and attached to the
diagnostic when the clause fails. The citation matters too. Claims about what Claude Code actually does
are external facts, so every built-in clause carries the URL and retrieval
date of the documentation behind it. When temper tells you a field is ignored,
it also tells you where that is written down.

Beyond per-file checks, a harness can declare **requirements**: things it must
contain. A requirement is filled when a member opts in by naming it. This is
coverage a file-by-file linter cannot see. temper can answer what fills each
requirement, and which members would strand one if they were deleted.

## The gate

`temper check` judges the harness against the active contract and exits
nonzero when a required clause fails. It runs offline, against committed
files, with no language runtime. Pointing it at any repo needs no setup:

```sh
temper check --harness .
```

The finding is the product. Each one renders with the guidance attached, so
the output reads as an explanation of what is wrong and why it matters, not a
bare rule id.

## Authoring, and where this is headed

The shipped path today is the zero-config check above: point temper at an
existing `.claude/` and read the findings.

The authoring model, which is landing now, inverts the flow. You describe the
harness as a small typed program using temper's SDK, and `temper emit`
compiles it into the actual files Claude Code reads, plus a lock file
recording exactly what was emitted. Drift detection then becomes one
comparison, disk versus lock. If someone hand-edits a generated file, temper
routes the change back to its authored source rather than merging around it.
Your prose is never rewritten; authored text lands in the output byte for
byte.

## Where the details live

This page is a summary. The operational definition of temper is its spec
corpus, which is versioned in this repository and holds the decision record
behind every behavior described above:

- [`specs/intent/00-intent.md`](../specs/intent/00-intent.md), why the project exists
- [`specs/intent/05-model.md`](../specs/intent/05-model.md), the full concept vocabulary
- [`specs/architecture/10-contracts.md`](../specs/architecture/10-contracts.md), clauses and requirements
- [`specs/architecture/20-surface.md`](../specs/architecture/20-surface.md), the CLI, emit, and the lock

If this page and a spec ever disagree, the spec is right and the page has a
bug.
