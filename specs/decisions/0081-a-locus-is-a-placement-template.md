# 0081 — a locus is a placement template

- **Date:** 2026-11-24 · **Status:** accepted

## Context

GH #64. A kind's `glob` does two jobs:

- **Discovery** matches it with globset, so character classes and braces
  work.
- **Placement** substitutes the member name for its one `*` as text
  (`drift.rs`, `splice_name`). So `domains/[a-z0-9-]*.md` places a member
  at `domains/[a-z0-9-]example.md`, a path its own glob cannot find.

Two further gaps:

- An extglob (`!(README).md`) fails to compile, and every caller treats a
  failed compile as no match, so the kind silently governs nothing.
- A kind has no way to leave a sibling README out of its directory.

The 0078 audit also found a registration kind spelling its manifest twice,
in `locus.glob` and in `collectionAddress.manifest`. Ruled by John
2026-11-24 on the session's recommendation.

## Decision

**A file locus is a placement template** (`representation.md`, "locus"):

- Literal segments with at most one `*`, optionally under a leading
  `**/` for any depth.
- Less the paths the kind declares **excluded**.
- Any other pattern syntax refuses at load.
- Narrowing names is a clause (`allowed_chars`), never a pattern.

Matching is derived from the template, so placement and discovery read
one spelling. A registration kind's starless locus names the manifest it
surfaces in and governs no document, so the collection address carries
only the key path.

## Rejected

- **A full glob with smarter placement.** The glob keeps both jobs, and
  placement has to reason about arbitrary patterns to substitute a name
  and then prove the result matches.
- **A sibling as its own kind.** Declaring the README as a member of a
  second, narrower kind needs a rule for overlapping loci, and "two kinds
  never govern one path" exists to keep that rule out.

## Consequences

Load refuses a non-template glob; the built-in globs (`*/SKILL.md`,
`**/*.md`, `**/CLAUDE.md`, fixed manifest paths) are templates already.
The SDK gains `exclude` on a file locus, and `collectionAddress.manifest`
retires in favor of the locus.
