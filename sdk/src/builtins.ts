/**
 * The built-in Claude Code kinds — the face nouns a harness author imports
 * (`specs/architecture/15-kinds.md`, "Built-in and custom kinds"). Each is an
 * ordinary `kind<T>()` value built with the same constructor every provider uses
 * (ownership not privilege). Their five facts are external facts about the Claude
 * Code harness, cited at the point of claim.
 *
 * In the shipped product these live in the published `@temper/claude-code` module
 * (`50-distribution.md`); here they are the SDK's own exports, the first dogfood.
 */

import { kind } from "./kind.js";
import type { KindDefinition } from "./kind.js";
import type { Prose } from "./prose.js";

/** A Claude Code skill — a directory whose entry file is `SKILL.md` with YAML frontmatter. */
export interface Skill {
  /**
   * The description trigger — always in context; the body loads on invocation
   * (code.claude.com/docs/en/skills, retrieved 2026-07-02).
   */
  readonly description: string;
  /** The optional license field the skill spec carries (agentskills.io/specification). */
  readonly license?: string;
  readonly prose?: Prose;
}

/**
 * `skill` — `.claude/skills/<name>/SKILL.md`, a directory unit, YAML frontmatter
 * carrying `name` then `description`; registers a description trigger
 * (code.claude.com/docs/en/skills, agentskills.io/specification, retrieved
 * 2026-07-02).
 */
export const skill: KindDefinition<Skill> = kind<Skill>({
  name: "skill",
  locus: { kind: "at", root: ".claude/skills", glob: "*/SKILL.md" },
  format: "yaml-frontmatter",
  unitShape: "directory",
  registration: { via: "description-trigger", field: "description" },
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
  registration: { via: "paths-match", field: "paths" },
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
  registration: { via: "always" },
});
