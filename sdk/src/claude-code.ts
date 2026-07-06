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
