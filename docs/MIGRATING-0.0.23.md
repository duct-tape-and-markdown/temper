# Migrating to 0.0.23

**This note covers `0.0.22` to `0.0.23` and nothing earlier.** For the
upgrade before it, read [`MIGRATING-0.0.22.md`](MIGRATING-0.0.22.md).

Every harness needs step 1. The rest apply only if you use what they name.
No built-in projection moves.

## 1. Re-emit and commit the lock

The `engine` line moves.

```text
before (0.0.22)  engine = "0.0.22"
after  (0.0.23)  engine = "0.0.23"
```

**What to do.** Run `temper emit` and commit the lock. A CI step running
`temper emit --frozen` then `git diff --exit-code` fails until you do.

## 2. Undo workarounds for falsely dead path-scoped rules

`check --harness <path>` with any spelling but `.` matched `paths` globs
against root-prefixed file paths, so a rule anchored at the root read dead.

```text
check --harness /abs/repo    rule paths: ["scripts/**"]
before (0.0.22)  root.reachable: … is dead — its `paths` globs match no file
after  (0.0.23)  (no finding)
```

**What to do.** If you widened a glob to `**/scripts/**` or dialed
`root.reachable` down to silence this, revert it.

## 3. New stray advisories under `.claude/`

`coverage.unclaimed-entry` used to claim an entry for a kind when the
entry matched only the last segment of that kind's locus glob. It now
matches the whole glob, as discovery and the guard do.

```text
kind locus  root ".claude"  glob "prompts/*.md"
before (0.0.22)  .claude/notes.md   claimed (matched "*.md")
after  (0.0.23)  .claude/notes.md   coverage.unclaimed-entry
```

`.claude/CLAUDE.md`, which was reported both a `memory` member and
unclaimed, is now only the member.

**What to do.** Model the entry with a kind, or remove it. The finding is
an advisory; it blocks only under `--deny-advisories`.

## 4. Relocated built-in kinds

A kind row that relocates a built-in kind's `governs` locus now moves the
stray scan with it.

```text
relocate rule  .claude/rules  →  .claude/policies
before (0.0.22)  unclaimed-entry over .claude/policies, silent over .claude/rules
after  (0.0.23)  silent over .claude/policies, unclaimed-entry over .claude/rules
```

A relocation rooted under `.temper/` now fails `check` with
`at-locus.under-workspace`, which `emit` already refused. A kind declaring
`commitment = "local"` under `.temper/` is no longer refused by either.

**What to do.** Model or remove what remains at the root you left, and
re-root any relocation that sits under `.temper/`.

## 5. Old `./` rows in the lock bind the guard

A lock written by an older engine may carry provenance rows
spelled `./CLAUDE.md`. The guard did not match them, so it allowed a hand
write to those projections. It now refuses the write, as `check` already
reported it.

**What to do.** Edit the owning module and run `temper emit`, which also
rewrites the rows as `CLAUDE.md`.

## 6. `explain` names tap events by their documented name

The member-grain field strand of `explain` now matches the requirement
strand.

```text
before (0.0.22)  skill_invoked
after  (0.0.23)  SkillInvoked
```

**What to do.** Update any script that greps `explain` output for the
snake_case label.

## 7. A lock address row must carry `entry_shape`

A `collection_address` row without `entry_shape` is refused when the lock
is read, where 0.0.22 inferred it. Every lock `emit` writes carries it.

**What to do.** If you built a lock by hand, run `temper emit` to rewrite it.
