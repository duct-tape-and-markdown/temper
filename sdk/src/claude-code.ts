/**
 * The `@dtmd/temper/claude-code` subpath — the first-party Claude Code
 * provider face (`specs/architecture/50-distribution.md`, "Decision: one SDK
 * package — the provider face is a subpath export"). A harness author who
 * targets Claude Code imports the built-in kinds from here, never the root:
 * the root carries only the six-noun core, and identity travels by import
 * (`specs/architecture/15-kinds.md`), so a subpath specifier is a full module
 * specifier like any other. The built-in floors (exported clause arrays) join
 * this entry when they are authored in `sdk/src`; today only the three kinds
 * exist.
 */

export type { Memory, Rule, Skill } from "./builtins.js";
export { memory, rule, skill } from "./builtins.js";

// The prose constructors ride along so a harness author targeting Claude Code
// never reaches back to the root package mid-member (`specs/architecture/20-surface.md`,
// "The port scene" — the reviewed diff imports `skill`/`file` from one specifier).
export type { Blocks, File, Prose, Text } from "./prose.js";
export { blocks, file, text } from "./prose.js";
