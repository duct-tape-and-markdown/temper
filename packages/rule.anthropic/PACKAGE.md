+++
# rule.anthropic — the built-in package for the `rule` kind (`.claude/rules/*.md`).
# PRODUCT SOURCE, human-curated; the build embeds this, never writes it.
# Renamed from the bare `rule` per specs/10-contracts.md ("named for its source"):
# these clauses are Anthropic-sourced exactly as skill.anthropic's are.
# All sources retrieved 2026-07-01.

[[clause]]
severity = "advisory"
predicate = "optional"
field = "paths"
guidance = "`paths` is the one documented frontmatter key for rules: glob patterns (brace expansion supported) that scope the rule to matching files. Rules without it load at launch with the same priority as CLAUDE.md; path-scoped rules load when Claude reads a matching file. Note skills now take a `paths` key too — the two schemas are separate."
source = "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "forbidden_keys"
keys = ["description", "globs", "alwaysApply"]
guidance = "Cursor `.mdc` keys. Claude Code's documented rules schema is `paths`-only; a rule authored with Cursor frontmatter is configuration another tool's semantics silently fail to honor — the rule loads, the scoping you meant does not. (That Claude Code ignores unknown keys is observed behavior, not documented contract — the documented schema is the citation.)"
source = "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-01)"

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 200
guidance = "Unconditional rules are always-on context, paid every session: the docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' (Distinct from the hard 200-line/25KB cutoff, which applies only to auto-memory MEMORY.md; rules load in full regardless of length.) For each line ask: would removing it cause Claude to make mistakes? If not, cut it."
source = "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-01)"
+++

# rule.anthropic

Anthropic's documented contract for a Claude Code rules file, as decidable
clauses — sourced from the memory docs (`.claude/rules/` landed in v2.0.64).
Adopt, extend, fork, or ignore: data, never a hardcoded check.

What the clauses cannot carry, as guidance: keep a rule to facts Claude should
hold whenever the rule is in scope — concrete enough to verify ("use 2-space
indentation", not "format code properly"). If an entry is a multi-step procedure
or only matters occasionally, it belongs in a skill (on-demand) rather than a
rule (always-on). Prefer path-scoped rules when one convention governs scattered
paths; prefer per-directory CLAUDE.md when directory owners maintain their own.
Treat rules like code: prune them when behavior drifts, and test a change by
watching whether Claude's behavior actually shifts.
