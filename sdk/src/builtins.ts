/**
 * The built-in Claude Code kinds — the face nouns a harness author imports.
 * Each is an
 * ordinary `kind<T>()` value built with the same constructor every provider uses
 * (ownership not privilege). Their five facts are external facts about the Claude
 * Code harness, cited at the point of claim.
 *
 * These are the SDK's own provider-face exports, surfaced through the
 * `@dtmd/temper/claude-code` subpath
 * — never from the root.
 */

import { kind } from "./kind.js";
import type { KindDefinition } from "./kind.js";
import type { Prose } from "./prose.js";
import {
  allowedChars,
  clause,
  deny,
  forbiddenKeys,
  maxLen,
  maxLines,
  minLen,
  nameMatchesDir,
  required,
} from "./contract.js";
import type { Clause } from "./contract.js";

/** A Claude Code skill — a directory whose entry file is `SKILL.md` with YAML frontmatter. */
export interface Skill {
 /**
   * The description trigger — always in context; the body loads on invocation
   * (code.claude.com/docs/en/skills, retrieved 2026-07-02).
 */
  readonly description: string;
  /** The optional license field the skill spec carries (agentskills.io/specification). */
  readonly license?: string;
  /**
   * Set `true` to prevent Claude from automatically loading this skill — only
   * the user-invoked channel stays live (code.claude.com/docs/en/skills,
   * "Control who invokes a skill", retrieved 2026-07-07). Default: `false`.
   */
  readonly "disable-model-invocation"?: boolean;
  /**
   * Set `false` to hide the skill from the `/` menu — only the
   * description-trigger channel stays live (code.claude.com/docs/en/skills,
   * "Control who invokes a skill", retrieved 2026-07-07). Default: `true`.
   */
  readonly "user-invocable"?: boolean;
  readonly prose?: Prose;
}

/**
 * `skill` — `.claude/skills/<name>/SKILL.md`, a directory unit, YAML frontmatter
 * carrying `name` then `description`; registers on both documented invocation
 * channels — user-invoked (`/name`) and description-trigger — modulated per
 * member by the `disable-model-invocation`/`user-invocable` fields
 * (code.claude.com/docs/en/skills, agentskills.io/specification, retrieved
 * 2026-07-07).
 */
export const skill: KindDefinition<Skill> = kind<Skill>({
  name: "skill",
  locus: { kind: "at", root: ".claude/skills", glob: "*/SKILL.md" },
  format: "yaml-frontmatter",
  unitShape: "directory",
  registration: [{ via: "user-invoked" }, { via: "description-trigger", field: "description" }],
  identityField: "name",
});

/** A Claude Code rule — a flat markdown file with an optional `paths` scope. */
export interface Rule {
 /**
   * The path scope — a present list matching zero files is a dead edge; an absent
   * one loads unconditionally (code.claude.com/docs/en/memory, retrieved 2026-07-02).
 */
  readonly paths?: readonly string[];
  readonly prose?: Prose;
}

/**
 * `rule` — `.claude/rules/<name>.md`, a lone file (identity from the stem), YAML
 * frontmatter; registers a path scope (code.claude.com/docs/en/memory, retrieved
 * 2026-07-02).
 */
export const rule: KindDefinition<Rule> = kind<Rule>({
  name: "rule",
  locus: { kind: "at", root: ".claude/rules", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "paths-match", field: "paths" }],
});

/** A Claude Code memory file — `CLAUDE.md`, loaded in full at launch, no frontmatter. */
export interface Memory {
  readonly prose?: Prose;
}

/**
 * `memory` — a root `<name>.md` (`CLAUDE.md`, `AGENTS.md`), a lone file loaded
 * unconditionally, with **no frontmatter** (code.claude.com/docs/en/memory,
 * retrieved 2026-07-02): the whole file is the body, so the kind declares no
 * `format`. Its discovery locus is any-depth (a `CLAUDE.md` at any directory);
 * a module-carried memory projects the root file.
 */
export const memory: KindDefinition<Memory> = kind<Memory>({
  name: "memory",
  locus: { kind: "at", root: ".", glob: "**/CLAUDE.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
});

/**
 * The floor for `skill` — Anthropic's documented skill contract: the Agent
 * Skills open standard (agentskills.io), Anthropic's platform upload
 * validation, and Claude Code's own docs.
 * All sources retrieved 2026-07-01.
 *
 * Checks the strictest documented profile: the spec and upload validation are
 * hard, Claude Code's runtime is deliberately forgiving ("All fields are
 * optional") — a skill that passes here loads everywhere the format is
 * honored, not merely on the machine it was written on; where the runtime
 * diverges, the clause's own guidance says so.
 *
 * Deliberately absent — undecidable, so never gate clauses: whether the
 * description actually triggers well or reads third-person (semantic);
 * vagueness/no-op detection (semantic); gerund naming (judgment). Two
 * decidable spec rules are also absent, pending a vocabulary addition (a
 * narrow shape predicategoverns additions): the name
 * must not start/end with a hyphen or contain consecutive hyphens; likewise
 * the platform's "no XML tags in the description."
 *
 * Authoring notes the clauses cannot carry: prefer gerund or noun-phrase
 * names (`processing-pdfs`, `pdf-processing`) over vague ones (`helper`,
 * `utils`); `disable-model-invocation: true` for side-effectful workflows you
 * want to time yourself; `user-invocable: false` for background knowledge
 * that is not a command; `metadata` is the sanctioned home for versioning —
 * there is no top-level `version` field.
 */
export const skillFloor: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "Every skill declares a `name` — the slug the harness binds to. Claude Code alone would default it from the directory name, but a nameless skill is not portable: the spec and Anthropic's upload validation both require it.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
  }),
  clause(minLen("name", 1), {
    severity: "required",
    guidance: "A present-but-empty name fails the spec's 1-64 character bound.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
  }),
  clause(allowedChars("name", { ranges: ["a-z", "0-9"], chars: "-" }), {
    severity: "required",
    guidance:
      "Lowercase letters, digits, and hyphens only — `PDF-Processing` is the spec's own counter-example. The charset also keeps XML out of the name, which Anthropic's upload validation separately forbids.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
  }),
  clause(maxLen("name", 64), {
    severity: "required",
    guidance: "Keep the name short and slug-like; it becomes a directory and an id.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
  }),
  clause(deny("name", ["anthropic", "claude"]), {
    severity: "required",
    guidance:
      "Reserved words, enforced by Anthropic's platform upload validation (not by the open spec, and not by Claude Code's runtime — which itself ships a `claude-api` skill). Keep them out if the skill will ever travel through the API or claude.ai.",
    cite: "https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview#skill-structure (retrieved 2026-07-01)",
  }),
  clause(nameMatchesDir(), {
    severity: "required",
    guidance:
      "The spec requires the name to match its parent directory. Claude Code decouples the two (the frontmatter name is a display label; the directory names the slash command, except for a plugin-root SKILL.md) — but a mismatch is a portability trap and a reader trap even where it loads.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-01)",
  }),
  clause(required("description"), {
    severity: "required",
    guidance:
      "The description is how the model chooses this skill from potentially 100+ available — it is the skill's API. Claude Code would fall back to the body's first paragraph; the spec and upload validation require it declared.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
  }),
  clause(minLen("description", 1), {
    severity: "required",
    guidance:
      "Say both what the skill does and when to use it, with the keywords a user would naturally say. Write in third person — the text is injected into the system prompt, and inconsistent point-of-view causes discovery problems.",
    cite: "https://agentskills.io/specification#description-field (retrieved 2026-07-01)",
  }),
  clause(maxLen("description", 1024), {
    severity: "required",
    guidance:
      "The spec's cap. Claude Code additionally truncates the skill listing at 1,536 combined characters (description + when_to_use) — truncation, not rejection, but text past the fold cannot help the model choose.",
    cite: "https://agentskills.io/specification#description-field (retrieved 2026-07-01)",
  }),
  clause(maxLen("compatibility", 500), {
    severity: "required",
    guidance: "Optional field; when present the spec caps it at 500 characters. Most skills do not need it.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
  }),
  clause(maxLines(500), {
    severity: "advisory",
    guidance:
      "Progressive disclosure: keep SKILL.md under 500 lines and move detailed reference material to separate files, one level deep. Once a skill loads, its body stays in context across turns — every line is a recurring token cost. The context window is a public good.",
    cite: "https://agentskills.io/specification#progressive-disclosure (retrieved 2026-07-01)",
  }),
  clause(forbiddenKeys(["globs", "alwaysApply"]), {
    severity: "required",
    guidance:
      "Cursor `.mdc` keys. Nothing in the Agent Skills spec or Claude Code's documented frontmatter accepts them — a skill authored with them is carrying dead configuration that another tool's semantics silently fail to apply.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-01)",
  }),
];

/**
 * The floor for `rule` — Anthropic's documented contract for a Claude Code
 * rules file, sourced from the memory docs (`.claude/rules/` landed in
 * v2.0.64; `packages/rule.anthropic/PACKAGE.md`, the curated authoring
 * reference this migrates verbatim). All sources retrieved 2026-07-01.
 *
 * `paths` is the one documented frontmatter key for rules: glob patterns
 * (brace expansion supported) that scope the rule to matching files. Rules
 * without it load at launch with the same priority as CLAUDE.md; path-scoped
 * rules load when Claude reads a matching file. Note skills now take a
 * `paths` key too — the two schemas are separate. (Guidance only: an
 * optional field asserts nothing decidable, so it carries no clause of its
 * own: `required` is the one
 * presence predicate, and its absence is not itself a predicate.)
 * https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-01)
 *
 * What the clauses cannot carry, as guidance: keep a rule to facts Claude
 * should hold whenever the rule is in scope — concrete enough to verify ("use
 * 2-space indentation", not "format code properly"). If an entry is a
 * multi-step procedure or only matters occasionally, it belongs in a skill
 * (on-demand) rather than a rule (always-on). Prefer path-scoped rules when
 * one convention governs scattered paths; prefer per-directory CLAUDE.md when
 * directory owners maintain their own. Treat rules like code: prune them when
 * behavior drifts, and test a change by watching whether Claude's behavior
 * actually shifts.
 */
export const ruleFloor: readonly Clause[] = [
  clause(forbiddenKeys(["description", "globs", "alwaysApply"]), {
    severity: "required",
    guidance:
      "Cursor `.mdc` keys. Claude Code's documented rules schema is `paths`-only; a rule authored with Cursor frontmatter is configuration another tool's semantics silently fail to honor — the rule loads, the scoping you meant does not. (That Claude Code ignores unknown keys is observed behavior, not documented contract — the documented schema is the citation.)",
    cite: "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-01)",
  }),
  clause(maxLines(200), {
    severity: "advisory",
    guidance:
      "Unconditional rules are always-on context, paid every session: the docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' (Distinct from the hard 200-line/25KB cutoff, which applies only to auto-memory MEMORY.md; rules load in full regardless of length.) For each line ask: would removing it cause Claude to make mistakes? If not, cut it.",
    cite: "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-01)",
  }),
];

/**
 * The floor for the qualified `claude-code.memory` kind — Anthropic's
 * documented contract for a project `CLAUDE.md` (`packages/memory.anthropic/PACKAGE.md`,
 * the curated authoring reference this migrates verbatim). Retrieved 2026-07-02.
 *
 * Deliberately near-empty, because the format is: `CLAUDE.md` is plain
 * markdown with no documented frontmatter and no required fields
 * (code.claude.com/docs/en/memory, retrieved 2026-07-02), so there is no
 * schema to gate — manufacturing a required field or a forbidden-key list
 * would fake a check the format does not carry. The single clause is a context-cost
 * budget; everything else the contract could say is guidance.
 *
 * What the clauses cannot carry, as guidance: a `paths:` frontmatter block
 * belongs on a `.claude/rules/*.md` file, not on `CLAUDE.md` — the memory
 * docs document `paths` only for rules, so a rules-style header on
 * `CLAUDE.md` is dead configuration. Split a large file with `@path` imports
 * (resolved relative to the importing file, absolute allowed, recursion
 * capped at four hops; wrap a path in backticks to mention it without
 * importing). If the repo already ships an `AGENTS.md` for other agents,
 * don't duplicate it — create a `CLAUDE.md` that `@AGENTS.md`-imports it (or
 * symlink, except on Windows where the import is the recommended bridge).
 * Mind the loading asymmetry: every ancestor `CLAUDE.md` loads in full at
 * launch, while files in subdirectories load only when Claude reads a file
 * there — so a rule that must always hold belongs above the working
 * directory, not below it. Personal, un-shared notes go in `CLAUDE.local.md`
 * (gitignored), appended after `CLAUDE.md` at its level.
 */
export const memoryAnthropicFloor: readonly Clause[] = [
  clause(maxLines(200), {
    severity: "advisory",
    guidance:
      "CLAUDE.md is always-on context, paid every session. The memory docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' For each line ask: would removing it cause Claude to make mistakes? If not, cut it. (Advisory: Claude Code loads the file in full regardless of length; this is a context-cost budget, not a hard cutoff.)",
    cite: "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-02)",
  }),
];

/**
 * The floor for the qualified `agents-md.memory` kind — the AGENTS.md
 * standard's contract for a memory file, which is that there is almost none
 * (`packages/memory.agents-md/PACKAGE.md`, the curated authoring reference
 * this migrates). Guidance-only, and that is the honest encoding: `AGENTS.md`
 * "is just standard Markdown" with no required fields, no sections, and no
 * frontmatter (agents.md, retrieved 2026-07-02); the format deliberately
 * constrains nothing. A floor that manufactured a required field, a size
 * gate, or a forbidden-key list would assert a contract the standard
 * disclaims. Even the tempting size
 * number is a *tool's* rule, not the format's: agents read the closest
 * `AGENTS.md` in the tree (nested, nearest-wins); Codex concatenates the
 * chain root-to-cwd and stops once combined size hits a byte budget, not a
 * per-file line count; Gemini CLI reads `GEMINI.md` by default and only
 * treats `AGENTS.md` as an alias when configured; Claude Code does not read
 * `AGENTS.md` natively — bridge it with a `CLAUDE.md` that
 * `@AGENTS.md`-imports it. All retrieved 2026-07-02.
 */
export const memoryAgentsMdFloor: readonly Clause[] = [];
