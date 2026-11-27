# DOC-COUNTS-AND-REQUIREMENT-WORD

Shipped as named; the rendered reference description now closes "Its contracted
requirements: …" (sdk/src/reference.ts), so that one string is author-visible
output, not a comment — two SDK tests updated with it.

Residual "domain" for a requirement remains in the reference skill's rendered
output, outside this entry's cited lines: the `### Domains` / `## Domains`
index headings, the `# The \`<r>\` domain` topic title, and the topic file
name `domain-<r>.md` (reference.ts :5-9, :139-212, :329-347, :409-452). Per
0114 case 5 those are author-read too; renaming the topic file is a projection
path change (owes a migration note), so it wants its own entry.

package.json: "needs" was a module name, not an export; the description now
names capabilities and lists span() beside the other constructors.

Re-emitted examples/base-harness (SKILL.md + lock) this time. The repo's own
dogfood projection `.claude/skills/temper/SKILL.md` (and `.temper/lock.toml`)
now drifts on the same description string; `.claude/` is outside build's
paths, so it owes a human `chore(harness):` re-emit.
