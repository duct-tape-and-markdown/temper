+++
# skill.anthropic — the built-in package for the `skill` kind.
# PRODUCT SOURCE, human-curated (specs/architecture/10-contracts.md: "product source, not the
# dogfood"); the build embeds this as the shipped std-lib default, never writes it.
# Checking posture: Anthropic's documented contract — the Agent Skills spec
# (agentskills.io) plus Anthropic's platform validation. Claude Code's runtime is
# deliberately laxer; divergences are noted per clause in guidance, never used to
# weaken a portability fact. All sources retrieved 2026-07-01.

[[clause]]
severity = "required"
predicate = "required"
field = "name"
guidance = "Every skill declares a `name` — the slug the harness binds to. Claude Code alone would default it from the directory name, but a nameless skill is not portable: the spec and Anthropic's upload validation both require it."
source = "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "min_len"
field = "name"
min = 1
guidance = "A present-but-empty name fails the spec's 1-64 character bound."
source = "https://agentskills.io/specification#name-field (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "allowed_chars"
field = "name"
ranges = ["a-z", "0-9"]
chars = "-"
guidance = "Lowercase letters, digits, and hyphens only — `PDF-Processing` is the spec's own counter-example. The charset also keeps XML out of the name, which Anthropic's upload validation separately forbids."
source = "https://agentskills.io/specification#name-field (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64
guidance = "Keep the name short and slug-like; it becomes a directory and an id."
source = "https://agentskills.io/specification#name-field (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "deny"
field = "name"
values = ["anthropic", "claude"]
guidance = "Reserved words, enforced by Anthropic's platform upload validation (not by the open spec, and not by Claude Code's runtime — which itself ships a `claude-api` skill). Keep them out if the skill will ever travel through the API or claude.ai."
source = "https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview#skill-structure (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "name-matches-dir"
guidance = "The spec requires the name to match its parent directory. Claude Code decouples the two (the frontmatter name is a display label; the directory names the slash command, except for a plugin-root SKILL.md) — but a mismatch is a portability trap and a reader trap even where it loads."
source = "https://agentskills.io/specification#name-field (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "required"
field = "description"
guidance = "The description is how the model chooses this skill from potentially 100+ available — it is the skill's API. Claude Code would fall back to the body's first paragraph; the spec and upload validation require it declared."
source = "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "min_len"
field = "description"
min = 1
guidance = "Say both what the skill does and when to use it, with the keywords a user would naturally say. Write in third person — the text is injected into the system prompt, and inconsistent point-of-view causes discovery problems."
source = "https://agentskills.io/specification#description-field (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "max_len"
field = "description"
max = 1024
guidance = "The spec's cap. Claude Code additionally truncates the skill listing at 1,536 combined characters (description + when_to_use) — truncation, not rejection, but text past the fold cannot help the model choose."
source = "https://agentskills.io/specification#description-field (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "max_len"
field = "compatibility"
max = 500
guidance = "Optional field; when present the spec caps it at 500 characters. Most skills do not need it."
source = "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)"

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 500
guidance = "Progressive disclosure: keep SKILL.md under 500 lines and move detailed reference material to separate files, one level deep. Once a skill loads, its body stays in context across turns — every line is a recurring token cost. The context window is a public good."
source = "https://agentskills.io/specification#progressive-disclosure (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "forbidden_keys"
keys = ["globs", "alwaysApply"]
guidance = "Cursor `.mdc` keys. Nothing in the Agent Skills spec or Claude Code's documented frontmatter accepts them — a skill authored with them is carrying dead configuration that another tool's semantics silently fail to apply."
source = "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)"
+++

# skill.anthropic

Anthropic's documented contract for a skill, as decidable clauses — sourced from
the Agent Skills open standard (agentskills.io), Anthropic's platform validation,
and Claude Code's docs. Adopt it, extend it, fork it, or ignore it: it is data,
never a hardcoded check.

**Enforcement profiles differ, and this package checks the strictest documented
one.** The spec and Anthropic's upload validation are hard; Claude Code's runtime
is deliberately forgiving ("All fields are optional"). The clauses hold the
portable contract — a skill that passes here loads everywhere the format is
honored, not merely on the machine it was written on. Where the runtime diverges,
the clause's guidance says so.

**Deliberately absent — undecidable, so never gate clauses** (they live here as
guidance instead): whether the description *actually* triggers well or reads
third-person (semantic); vagueness / no-op detection (semantic); gerund naming
(judgment). Two *decidable* spec rules are also absent, pending a vocabulary
addition (a narrow shape predicate — `specs/architecture/10-contracts.md` governs additions):
the name must not start/end with a hyphen and must not contain consecutive
hyphens; likewise the platform's "no XML tags in the description."

Authoring notes the clauses cannot carry: prefer gerund or noun-phrase names
(`processing-pdfs`, `pdf-processing`) over vague ones (`helper`, `utils`);
`disable-model-invocation: true` for side-effectful workflows you want to time
yourself; `user-invocable: false` for background knowledge that is not a command;
`metadata` is the sanctioned home for versioning — there is no top-level
`version` field.
