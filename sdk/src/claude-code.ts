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
  GateHookEvent,
  GateHookInit,
  Handler,
  Hook,
  HookInit,
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
  gateHook,
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

// Retired exports, declared for the type checker alone: no runtime binding carries
// them, so a program still naming one fails `tsc` with its replacement spelled out.
import type { Remedy } from "./kind.js";
export declare const GUARD_COMMAND: Remedy<"GUARD_COMMAND is retired: declare the hook as `gateHook({ name, matcher })`">;
export declare const SESSION_START_COMMAND: Remedy<"SESSION_START_COMMAND is retired: declare the hook as `gateHook({ name: 'SessionStart' })`">;

// The prose constructors ride along so a harness author targeting Claude Code
// never reaches back to the root package mid-member.
export type { Blocks, File, Prose, Text } from "./prose.js";
export { blocks, file, text } from "./prose.js";
