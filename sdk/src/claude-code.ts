/**
 * The `@dtmd/temper/claude-code` subpath — the first-party Claude Code
 * provider face. A harness author who
 * targets Claude Code imports the built-in kinds from here, never the root:
 * the root carries only the six-noun core, and identity travels by import,
 * so a subpath specifier is a full module
 * specifier like any other. The built-in default contracts join the kinds here
 * too: adoption is `import { skill, skillDefaultContract } from "@dtmd/temper/claude-code"`.
 */

export type {
  Agent,
  Handler,
  Hook,
  InstalledPlugin,
  KnownMarketplace,
  Marketplace,
  MarketplacePlugin,
  MarketplaceSource,
  Mcp,
  McpServer,
  Memory,
  PluginManifest,
  Rule,
  Settings,
  SettingsLocal,
  Skill,
  SupportingDoc,
} from "./builtins.js";
export {
  agent,
  agentDefaultContract,
  command,
  commandDefaultContract,
  handler,
  handlerDefaultContract,
  hook,
  hookDefaultContract,
  installedPlugin,
  installedPluginDefaultContract,
  knownMarketplace,
  knownMarketplaceDefaultContract,
  marketplace,
  marketplaceDefaultContract,
  mcp,
  mcpDefaultContract,
  mcpServer,
  mcpServerDefaultContract,
  memory,
  memoryAnthropicDefaultContract,
  pluginManifest,
  pluginManifestDefaultContract,
  rule,
  ruleDefaultContract,
  settings,
  settingsDefaultContract,
  settingsLocal,
  settingsLocalDefaultContract,
  skill,
  skillDefaultContract,
  supportingDoc,
  supportingDocDefaultContract,
} from "./builtins.js";

// temper's own gate commands — the exec-form strings its `SessionStart` and guard hook
// members run. The engine is their one home (`src/install.rs`); these bindings are
// machine-written across the `generated/` seam from those constants and held byte-equal by
// the seam gate (`tests/it/seam_bindings_current.rs`), so a scaffolded gate-hook member
// imports the command it fires rather than spelling a literal that goes stale the day the
// command changes (decision 0073).
export { GUARD_COMMAND, SESSION_START_COMMAND } from "./generated/index.js";

// The prose constructors ride along so a harness author targeting Claude Code
// never reaches back to the root package mid-member.
export type { Blocks, File, Prose, Text } from "./prose.js";
export { blocks, file, text } from "./prose.js";
