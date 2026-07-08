/**
 * The `@dtmd/temper/claude-code` subpath — the first-party Claude Code
 * provider face. A harness author who
 * targets Claude Code imports the built-in kinds from here, never the root:
 * the root carries only the six-noun core, and identity travels by import,
 * so a subpath specifier is a full module
 * specifier like any other. The built-in floors join the kinds here
 * too: adoption is `import { skill, skillFloor } from "@dtmd/temper/claude-code"`.
 */

export type { Memory, Rule, Skill } from "./builtins.js";
export {
  command,
  commandFloor,
  memory,
  memoryAgentsMdFloor,
  memoryAnthropicFloor,
  rule,
  ruleFloor,
  skill,
  skillFloor,
} from "./builtins.js";

// The prose constructors ride along so a harness author targeting Claude Code
// never reaches back to the root package mid-member.
export type { Blocks, File, Prose, Text } from "./prose.js";
export { blocks, file, text } from "./prose.js";
