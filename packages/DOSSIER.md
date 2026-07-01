# Built-in package citation dossier — authoring-session working material

Input to authoring `packages/{skill.anthropic,rule.anthropic}/PACKAGE.md`
(`specs/10-contracts.md`, "named for its source, and cited to it"). Each entry
becomes a per-clause `source` stamp; the guidance quotes seed the guidance
channel. All sources retrieved **2026-07-01**. This file is consumed by the
session, then kept as the audit-trail companion or deleted — session's call.

Canonical URL note: `docs.claude.com/en/docs/agents-and-tools/...` now
302-redirects to `platform.claude.com/docs/...`; the old
`anthropic.com/engineering/claude-code-best-practices` post 308-redirects to
`code.claude.com/docs/en/best-practices` (no separately maintained blog text
remains). Cite the targets, not the redirects.

## The headline finding: skill constraints are enforcement-profile-dependent

Three surfaces disagree, and any clause must say which it checks against:

- **SPEC** — https://agentskills.io/specification — the Agent Skills open
  standard, now the normative home ("Claude Code skills follow the Agent
  Skills open standard"). Hard validation rules live here.
- **API** — https://platform.claude.com/docs/en/build-with-claude/skills-guide
  — upload validation; adds rules the spec lacks (reserved words, no XML tags).
- **CC runtime** — https://code.claude.com/docs/en/skills — enforces almost
  none of it: "All fields are optional. Only `description` is recommended."
  `name` defaults from the directory; missing `description` falls back to the
  body's first paragraph.

**Session decision:** what posture `skill.anthropic` checks — spec-conformance
(portability), CC-runtime (does-it-load), or clauses fact-marked per profile.
Related: the spec's home is now the open standard, so the source suffix itself
is a question (`skill.anthropic` vs `skill.agentskills` — or the suffix stays
`anthropic` as the curating source of the *package*, citing the spec per
clause). Connects to `45-governance.md`'s harness-version/profile loose end.

## skill.anthropic — clause-by-clause

| clause (today) | verdict | source |
|---|---|---|
| `name` required | SPEC/API hard; **CC: optional** (defaults to dir name) | https://agentskills.io/specification#frontmatter — "name … Required: Yes"; CC #frontmatter-reference: "Required: No … Defaults to the directory name" |
| `name` min_len 1 / max_len 64 | SPEC hard | https://agentskills.io/specification#name-field — "Must be 1-64 characters" |
| `name` charset `[a-z0-9-]` | SPEC hard | #name-field — "unicode lowercase alphanumeric characters (`a-z`, `0-9`) and hyphens (`-`)" |
| `name` deny "anthropic"/"claude" | **API-only** — not in spec, not enforced by CC (CC ships a `claude-api` skill) | platform overview #skill-structure — "Cannot contain reserved words: \"anthropic\", \"claude\"" |
| `name-matches-dir` | SPEC hard; **CC explicitly decouples** (display label vs command name; exception: plugin-root SKILL.md) | spec #name-field — "Must match the parent directory name"; CC #how-a-skill-gets-its-command-name |
| `description` required | SPEC/API hard; **CC: recommended-with-fallback** | spec frontmatter table; CC: "If omitted, uses the first paragraph" |
| `description` min 1 / max 1024 | SPEC hard | spec #description-field — "Must be 1-1024 characters" (CC truncates listing at 1,536 combined chars — truncation, not rejection) |
| body max_lines 500 | best-practice in all three sources | spec #progressive-disclosure — "Keep your main SKILL.md under 500 lines"; BP #token-budgets; CC #add-supporting-files |
| forbidden_keys globs/alwaysApply | no official "ignored keys" statement anywhere; the valid-key sets are documented (below) | justification = absence from both documented schemas |

**Missing clauses to consider adding (spec-hard, cheap):**
- `name` no leading/trailing hyphen — spec #name-field: "Must not start or end with a hyphen"
- `name` no consecutive hyphens — spec: "Must not contain consecutive hyphens (`--`)"
- `name`/`description` no XML tags — API-only (overview/BP/skills-guide): "Cannot contain XML tags"
- `compatibility` max 500 chars if present — spec: "Must be 1-500 characters if provided"

**Documented key sets** (for `forbidden_keys`/schema clauses):
- Spec closed set: `name, description, license, compatibility, metadata, allowed-tools` (experimental).
- CC extensions: `when_to_use, argument-hint, arguments, disable-model-invocation, user-invocable, allowed-tools, disallowed-tools, model, effort, context, agent, hooks, paths, shell`.
- Upstream inconsistency on record: the plugin validator rejects CC extensions
  (anthropics/claude-code#25380) while the runtime accepts them.
- No top-level `version` field exists; the sanctioned home is `metadata`
  (spec example: `metadata: {author: …, version: "1.0"}`).

**Conformance oracle:** `skills-ref validate ./my-skill` —
https://github.com/agentskills/agentskills/tree/main/skills-ref — candidate
test fixture source for spec-profile clauses.

**Guidance channel raw material (skill):**
- Third person: "Always write in third person. The description is injected
  into the system prompt" (BP #writing-effective-descriptions; good/avoid
  examples there).
- Triggers: "Include both what the Skill does and specific triggers/contexts
  for when to use it"; "Claude uses it to choose the right Skill from
  potentially 100+ available Skills."
- Naming: gerund-form recommendation + "Avoid: Vague names: `helper`,
  `utils`, `tools`" (BP #naming-conventions).
- Conciseness: "The context window is a public good"; "Does this paragraph
  justify its token cost?" (BP #concise-is-key); "Keep references one level
  deep from SKILL.md."
- Invocation: `disable-model-invocation: true` for side-effectful workflows
  ("You don't want Claude deciding to deploy"); `user-invocable: false` for
  background knowledge; menu-visibility vs tool-access distinction
  (CC #control-who-invokes-a-skill).

## rule.anthropic — clause-by-clause

Primary source: https://code.claude.com/docs/en/memory. Feature landed
v2.0.64 (2025-12-10, changelog).

| clause (today) | verdict | source |
|---|---|---|
| `optional` paths | hard fact — `paths` is the **only documented** rules frontmatter key | #path-specific-rules — "Rules can be scoped … using YAML frontmatter with the `paths` field"; glob table + brace expansion documented |
| forbidden_keys description/globs/alwaysApply | **NO OFFICIAL SOURCE** — zero mentions of `.mdc`/`globs`/`alwaysApply` in docs or changelog; ignoring is observed behavior | justification = the documented schema is `paths`-only; community observation as secondary. Docs never affirm "unknown keys ignored" either |
| max_lines 200 (advisory) | best-practice with a real docs source | #write-effective-instructions — "Size: target under 200 lines per CLAUDE.md file. Longer files consume more context and reduce adherence" |

**Fact-marking caution:** the motivating clause (forbidden Cursor keys) is
temper's flagship and it is *folklore-sourced*. Honest stamping: source = the
documented `paths`-only schema + explicit "ignoring is observed, not
documented." Consider filing an upstream docs issue asking for the unknown-key
contract — a citation we create.

**Also documented (extraction/kind facts, not clauses):**
- Unconditional rules "loaded at launch with the same priority as
  `.claude/CLAUDE.md`"; path-scoped rules "trigger when Claude reads files
  matching the pattern" (#organize-rules, #path-specific-rules).
- Recursive discovery ("All `.md` files are discovered recursively"),
  subdirectories, symlinks (resolved; cycles handled), user-level
  `~/.claude/rules/` loaded before project rules.
- The 200-line **hard** cutoff is `MEMORY.md` only ("first 200 lines … or the
  first 25KB"); CLAUDE.md/rules load in full regardless — keep the advisory
  honest about which is which.
- `@`-imports documented for CLAUDE.md; **undocumented for rules files** (gap).
- Skills also take a `paths` key — two schemas, don't conflate.
- Debugging: the `InstructionsLoaded` hook (v2.1.69) logs which instruction
  files load and why — relevant to `50-distribution.md` placements someday.

**Guidance channel raw material (rule):**
- What belongs where: "Keep it to facts Claude should hold in every session …
  If an entry is a multi-step procedure or only matters for one part of the
  codebase, move it to a skill or a path-scoped rule instead" (#when-to-add).
- Specificity: "write instructions that are concrete enough to verify. 'Use
  2-space indentation' instead of 'Format code properly'."
- Conciseness: "For each line, ask: 'Would removing this cause Claude to make
  mistakes?' If not, cut it. Bloated CLAUDE.md files cause Claude to ignore
  your actual instructions!" (best-practices).
- Treat it like code: "review it when things go wrong, prune it regularly,
  and test changes by observing whether Claude's behavior actually shifts."
- Rules vs per-directory CLAUDE.md: rules "when you want all conventions in
  one place, or the same rule applies to many scattered paths"
  (large-codebases #choose-between).

## Session decision items (pre-gathered)

1. `skill.anthropic`'s checking profile: spec / CC-runtime / fact-marked per
   profile — and whether the suffix stays `anthropic` now that the spec's
   normative home is the open standard.
2. Adopt the four missing spec-hard name/field clauses?
3. How to stamp the folklore-sourced Cursor-keys clause (+ file the upstream
   docs issue?).
4. Split the 200-line advisory's citation from the MEMORY.md hard cutoff so
   the guidance never conflates them.
