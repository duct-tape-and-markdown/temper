---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: [".temper/**"]
---
# Authoring the temper program

This directory is the harness's source: a typed program over `@dtmd/temper`. The
harness files it describes (`CLAUDE.md`, `.claude/**`, any governed document) are
projections of it.

## The loop

- Change the module or the prose file that owns a member, then run `temper emit`.
  Never edit a projection: a file carrying the `temper: managed projection` header is
  written by `emit`, and a direct edit is drift that `check` and `guard` report.
- Run `temper check` after an emit; read its findings and their guidance before
  deciding a finding is noise. `temper explain <kind>` prints what a kind expects.
- A refusal from `emit` or a type error from `tsc` names the broken input. Fix the
  source; never route around a refusal by loosening a type.

## Members

- Build members with the kind constructors (`rule`, `skill`, `memory`, `hook`,
  `agent`, ... from `@dtmd/temper/claude-code`, or your own `kind()`), and list every
  one in `harness({ members })`. A member the program does not list is not governed.
- Keep long prose in a sibling Markdown file and reference it with
  `file(import.meta.url, "./name.md")`; use `text` for short inline prose, `span`
  only for a string the program computes. temper copies prose verbatim.
- Cite another member as `text` with `${mentionOf(member)}` interpolated, not a
  hand-typed name or path: the mention resolves at emit and refuses when its target
  is gone.
- A temper gate placement is `gateHook({ event, matcher })`, never `hook()` with a
  hand-written `temper` command: the engine renders the command, so it stays correct
  across installs and upgrades.

## Requirements and contracts

- A `require` entry names a role the harness must fill; members fill it through
  `satisfies`. Unset, `required` means at least one satisfier at error severity, so
  an empty slot fails `check`. A slot you declare ahead of its content, or one that
  is aspirational, says `required: false`.
- Bind clauses to a kind with `expect`; start from the shipped default contract
  (`skillDefaultContract`, ...) and add to it rather than restating it.
- A check enters only as a decidable clause. Behavior no predicate can decide belongs
  to a wired verifier or to your own review — never a clause that approximates it. A
  clause that can false-fire (`mentionReachable`'s glob containment) is advisory.

## Kinds

- Declare a custom kind's intent as `guidance` and its fields' as `fieldGuidance`,
  never only in a comment: emit delivers guidance as a rule scoped to the kind's
  documents, where the next author reads it.
- To move or retune a built-in, `relocate` it; do not redeclare it under a new name.
- A subkind is `extend<Base, Sub>(base, delta)` with `interface Sub extends Base`:
  `tsc` refuses a surface that drops or retypes a base field. A field interface,
  clause set, or guidance several subkinds share is an ordinary TypeScript value —
  extend the interface, and spread the value into each `extend` delta.
- Narrow a kind's members with a clause, never a richer locus glob. A `render` hook
  treats every `value.targets` entry as possibly absent.
- The rule names `temper-sdk` and `temper-guidance-<kind>` are temper's own; an
  authored rule at either refuses.
