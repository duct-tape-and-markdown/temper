# Protocol — project conventions

Runtime mechanics — the phases, their prompts, the gates, the entry
extension, the judges — ship in the harness package and are configured in
`.flume/declaration.ts`. This file holds the project-side conventions
no declaration encodes.

## The chain

`specs/` → `.flume/plan/` → `src/`, `sdk/`, `tests/` → git log

`specs/` is the evergreen source of truth (`specs/process/spec-system.md`),
authored only in interactive sessions, never by a phase. It is not a release
line: plan reconciles code against the living corpus. `specs/intent.md` is
the north star; `specs/decisions/` is history, outside every `per` cite.

## Two harnesses live here

1. **The flume harness** (`.flume/`) — this pipeline. Human territory; build
   writes only its records.
2. **The Claude Code harness** (`.claude/`, `CLAUDE.md`) — the product
   domain, and the environment every tick runs inside. `CLAUDE.md` and the
   `.claude/` member files are projections of `.temper/` sources: a direct
   edit is drift the guard blocks.

The build fence excludes both, plus `.temper/`, `specs/` and `docs/`. A tick
that finds one of them wrong files a record; it never edits.

## What an entry means here

The discipline file holds what every entry carries. This project adds:

1. **A retirement's blast radius is symbol scope.** `rg` a retired
   function or type across `tests/**` as well as `src/**` and `sdk/**`: a
   shared helper under `tests/it/common/` fans the edit out to every file
   that imports it, and `files` names those up front.
2. **A fold's census is the whole class, by behavior.** Before filing "X
   folds into one home", find every copy by what it does — the literal it
   spells, the shape it builds, the call it makes — across `src/` unit
   tests, `tests/it/**` and `sdk/**`, and name each in `files`.
3. **Widening a shared enumeration names its other consumers** in `files`
   (`specs/process/engineering.md`, "A shared concept is one type").
4. **A seam change is two-sided.** A declaration-row change moves the SDK
   row builders (`sdk/src/declarations.ts`), the Rust reader
   (`src/read.rs`) and the embedded lock (`src/builtin_lock.toml`) in one
   entry.

## Tests

A `tests[]` or `pins[]` line names a Rust test by its words. Build names
the test from the line in snake_case: `a layout reads its frontmatter` is
`fn a_layout_reads_its_frontmatter`. Punctuation drops, or is spelled out
where it carries meaning (`` a `.`-rooted locus `` as `a_dot_rooted_locus`):
the line's words must appear in the name in order. The judge runs
`.flume/runner.mjs` over `cargo nextest`; integration tests live in the one
`tests/it/` binary, unit tests beside their flat `src/` module.

**A `tests[]` line's test lives under `tests/it/`, never in a `src/`
module.** The judge proves the line new by laying the test's file over the
pre-change tree; a `src/` file carries its module's fix with it, so the line
passes there by construction and is refused. A unit test pins a `pins[]`
line, or none. Behavior whose only test is an SDK one (`sdk/test/`) names
no line at all: the judge runs cargo alone, and the SDK gate holds it.

## Lanes

| Layer | Artifact | Author | Commit prefix |
| ----- | -------- | ------ | ------------- |
| spec | `specs/` | session, human in the loop | `specs:` |
| plan | `.flume/plan/` | plan slices | `plan:` |
| code | `src/`, `sdk/`, `tests/` | build | `build:` |
| harnesses | `.flume/`, `.claude/`, `.temper/`, `CLAUDE.md` | session | `chore(harness):`, `chore(flume):` |
| release | version homes, `CHANGELOG.md` | session | `chore(release):` |

## Records

A finding for plan and a note from a build tick are **records**: one file
each, drained by the inbox slice and deleted on the way out. The paths, the
title line, and the byte cap are the package's (`docs/CHAIN-AUTHORING.md`,
*Records: one file each*):

- **Inbox** — `.flume/inbox/<YYYY-MM-DD>-<slug>.md`, from whoever observes
  something in the field: an adopter's issue, a session's routing.
- **Build notes** — `.flume/plan/notes/<TAG>.md`, from the build tick
  assigned that entry: an observation for the next plan tick, structural
  debt the tick could not take among them.
- **Build parks** — `.flume/plan/notes/parked/<TAG>.md`: the entry cannot
  ship as written and stays in the queue.
- **Build continuations** — `.flume/plan/notes/continuing/<TAG>.md`: a
  green segment landed and the rest is another tick's.

**Friction** — the declared channel `.flume/friction/`, gitignored: a tick
that hit costly harness friction files one note there (the
`capture-friction` skill); the inbox slice routes it.

## Kept on purpose

Deliberate asymmetries, each a ruling with the condition that reopens it.
Nothing is filed against one; a tick that meets its condition files a
question.

- **A pack is a skill — no skill-package kind.** `skill` plus its nested
  `supporting-doc` layer owns the shape. Reopens for a consumer who cannot
  express a pack with the two.
- **Default-contract auto-adoption.** A bare harness gets the built-in
  kinds checked with no assembly declaration: the engine embeds a built-in
  lock (`specs/model/pipeline.md`, "The lock"). Data, not code.
- **Format implementations are engine code** (frontmatter, `json-document`,
  `toml-document`); the kind that selects one is data. A format grows only
  by deliberate addition, answering the exhaustive `Format` match.
- **Stale cites: intra-doc links are gated, prose rides.** Broken intra-doc
  links fail the `cargo doc` gate for every item; the
  `rustdoc::private_intra_doc_links` lint stays advisory. Prose staleness
  no linter can check rides the next entry that opens the file, never its
  own entry, tracked nowhere. Reopens when a reader cannot find a real
  broken link among the advisory ones.
- **`.flume/` is ungoverned by temper.** The pipeline that builds temper is
  outside its gate; a candidate governed corpus once a custom-kind story
  for it proves end to end (`specs/model/representation.md`, "Reach").
- **`docs/` is candidate intent, not intent** — human territory; no tick
  reads a page there as intent.

## Disk vs git log

To answer "did X ship?" or "is gate Y green?", read the disk artifact — the
queue at `.flume/plan/pending/`, the source file, `cargo` output. Git log is
orientation, not authority.

## Push policy

No phase pushes. Trunk moves on this machine by the engine's fast-forward;
the operator pushes `main`. Never force-push, amend a pushed commit, or
`--no-verify`.
