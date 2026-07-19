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
  closedKeys,
  degree,
  deny,
  enumOf,
  extent,
  forbiddenKeys,
  globValid,
  maxLen,
  mentionReachable,
  minLen,
  nameMatchesDir,
  optional,
  required,
  shape,
  type,
  uniqueName,
  when,
} from "./contract.js";
import type { Clause } from "./contract.js";

/** A Claude Code skill — a directory whose entry file is `SKILL.md` with YAML frontmatter. */
export interface Skill {
 /**
   * The description trigger — always in context; the body loads on invocation
   * (code.claude.com/docs/en/skills, retrieved 2026-07-15).
 */
  readonly description: string;
  /**
   * Extra trigger context — phrases and example requests appended to
   * `description` in the skill listing, sharing its 1,536-character cap
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved
   * 2026-07-15).
   */
  readonly when_to_use?: string;
  /**
   * Autocomplete hint for the arguments the skill expects, e.g. `[issue-number]`
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly "argument-hint"?: string;
  /**
   * Named positional arguments for `$name` substitution in the body — a
   * space-separated string or a YAML list, mapped to positions in order
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly arguments?: readonly string[] | string;
  /**
   * Tools Claude may use without a permission prompt while the skill is active —
   * a space/comma-separated string or a YAML list; it grants, never restricts
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly "allowed-tools"?: readonly string[] | string;
  /**
   * Tools removed from the pool while the skill is active, cleared on your next
   * message — a space/comma-separated string or a YAML list
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly "disallowed-tools"?: readonly string[] | string;
  /**
   * Model for the rest of the turn while the skill is active — a `/model` value
   * or `inherit`; not saved to settings
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly model?: string;
  /**
   * Effort level while the skill is active, overriding the session's; the
   * available levels depend on the model
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly effort?: "low" | "medium" | "high" | "xhigh" | "max";
  /**
   * Set `fork` to run the skill in a forked subagent context, its body the
   * subagent's prompt
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly context?: "fork";
  /**
   * The subagent type a `context: fork` skill runs as — a built-in
   * (`Explore`/`Plan`/`general-purpose`) or a custom agent; defaults to
   * `general-purpose`
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly agent?: string;
  /**
   * Hooks scoped to this skill's lifecycle, in the hooks configuration format
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly hooks?: Readonly<Record<string, unknown>>;
  /**
   * Shell for the skill's `` !`command` `` injections — `bash` (default) or
   * `powershell`
   * (code.claude.com/docs/en/skills, "Frontmatter reference", retrieved 2026-07-15).
   */
  readonly shell?: "bash" | "powershell";
  /** The optional license field the skill spec carries (agentskills.io/specification). */
  readonly license?: string;
  /**
   * Set `true` to prevent Claude from automatically loading this skill — only
   * the user-invoked channel stays live (code.claude.com/docs/en/skills,
   * "Control who invokes a skill", retrieved 2026-07-15). Default: `false`.
   */
  readonly "disable-model-invocation"?: boolean;
  /**
   * Set `false` to hide the skill from the `/` menu — only the
   * description-trigger channel stays live (code.claude.com/docs/en/skills,
   * "Control who invokes a skill", retrieved 2026-07-15). Default: `true`.
   */
  readonly "user-invocable"?: boolean;
  /**
   * The optional path scope — a channel gate, not a channel of its own. A
   * present list removes the skill from *every* invocation channel — the `/`
   * listing, model invocation, and description-trigger invocation — until
   * Claude reads a file the globs match; an absent one leaves all channels
   * live. Distinct from a rule's `paths`, which registers the path-match as
   * the rule's channel: here the field gates the skill's existing channels
   * rather than being one, so it adds no `paths-match` registration entry
   * (code.claude.com/docs/en/skills, retrieved 2026-07-15; verified against
   * 2.1.210).
   */
  readonly paths?: readonly string[];
  readonly prose?: Prose;
}

/**
 * A skill's bundled reference document — a supporting file beside its `SKILL.md`,
 * loaded only when the skill's body points Claude at it. Prose and nothing else:
 * the documented shape is a markdown file in the skill's directory with no
 * frontmatter schema of its own (code.claude.com/docs/en/skills, "Add supporting
 * files", retrieved 2026-07-16).
 */
export interface SupportingDoc {
  readonly prose?: Prose;
}

/**
 * `supporting-doc` — a skill's bundled reference document, at the nested-file locus:
 * its path composes from its host skill's unit and the host's template pattern, so it
 * governs no glob of its own. Fields-free and frontmatterless (no `format` — the whole
 * file is body), identity from the filename, and channel-less: a supporting file reaches
 * the world only through the skill whose body references it, never on a channel of its
 * own (code.claude.com/docs/en/skills, "Add supporting files", retrieved 2026-07-16).
 */
export const supportingDoc: KindDefinition<SupportingDoc> = kind<SupportingDoc>({
  name: "supporting-doc",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
});

/**
 * `skill` — `.claude/skills/<name>/SKILL.md`, a directory unit, YAML frontmatter
 * carrying `name` then `description`; registers on both documented invocation
 * channels — user-invoked (`/name`) and description-trigger — modulated per
 * member by the `disable-model-invocation`/`user-invocable` fields
 * (code.claude.com/docs/en/skills, agentskills.io/specification, retrieved
 * 2026-07-15).
 *
 * Its one template layer names the bundled reference documents a skill's directory
 * carries: `supporting-doc` children at the directory's own markdown, the documented
 * placement (`my-skill/reference.md` beside `SKILL.md`; same source, "Add supporting
 * files"). The pattern claims the markdown subset the prose-only child kind can hold —
 * a supporting file of another type (the docs' own `scripts/helper.py`) matches nothing
 * and stays unmodeled rather than mis-typed.
 */
export const skill: KindDefinition<Skill> = kind<Skill>({
  name: "skill",
  locus: { kind: "at", root: ".claude/skills", glob: "*/SKILL.md" },
  format: "yaml-frontmatter",
  unitShape: "directory",
  registration: [{ via: "user-invoked" }, { via: "description-trigger", field: "description" }],
  identityField: "name",
  templates: [{ kind: supportingDoc, path: "*.md" }],
});

/**
 * `command` — `.claude/commands/*.md`, the skill surface's legacy file placement
 * (Claude Code merged commands into skills; code.claude.com/docs/en/skills,
 * retrieved 2026-07-15): a lone file (identity from the stem, so no
 * `identityField` — like `rule`), the skill's field schema by import, registering
 * on the same two documented invocation channels as `skill`.
 */
export const command: KindDefinition<Skill> = kind<Skill>({
  name: "command",
  locus: { kind: "at", root: ".claude/commands", glob: "*.md" },
  format: "yaml-frontmatter",
  unitShape: "file",
  registration: [{ via: "user-invoked" }, { via: "description-trigger", field: "description" }],
});

/** A Claude Code subagent definition. */
export interface Agent {
 /**
   * When Claude should delegate to this subagent — the sole registration
   * channel (code.claude.com/docs/en/sub-agents, retrieved 2026-07-15).
 */
  readonly description: string;
  /**
   * Tools the subagent may use; inherits all when omitted — a space/comma-separated
   * string or a YAML list
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly tools?: readonly string[] | string;
  /**
   * Tools denied — removed from the inherited or specified pool
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly disallowedTools?: readonly string[] | string;
  /**
   * Model to run as: `sonnet`/`opus`/`haiku`/`fable`, a full model id, or `inherit`
   * (the default)
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly model?: string;
  /**
   * Permission mode the subagent runs under, overriding the inherited one where the
   * parent mode does not take precedence (`manual` aliases `default`)
   * (code.claude.com/docs/en/sub-agents, "Permission modes", retrieved 2026-07-15).
   */
  readonly permissionMode?:
    | "default"
    | "acceptEdits"
    | "auto"
    | "dontAsk"
    | "bypassPermissions"
    | "plan"
    | "manual";
  /**
   * Maximum agentic turns before the subagent stops
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly maxTurns?: number;
  /**
   * Skills preloaded into the subagent's context at startup — full content, not just
   * descriptions
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly skills?: readonly string[] | string;
  /**
   * MCP servers available to the subagent — each entry a configured server's name or
   * an inline `name → config` definition
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly mcpServers?: readonly string[] | Readonly<Record<string, unknown>>;
  /**
   * Lifecycle hooks scoped to this subagent
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly hooks?: Readonly<Record<string, unknown>>;
  /**
   * Persistent memory scope enabling cross-session learning: `user`, `project`, or
   * `local`
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly memory?: "user" | "project" | "local";
  /**
   * Set `true` to always run this subagent as a background task
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly background?: boolean;
  /**
   * Effort level while the subagent is active, overriding the session's; the
   * available levels depend on the model
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly effort?: "low" | "medium" | "high" | "xhigh" | "max";
  /**
   * Set `worktree` to run the subagent in a temporary git worktree — an isolated repo
   * copy, auto-cleaned when it makes no changes
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly isolation?: "worktree";
  /**
   * Display color in the task list and transcript
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly color?: "red" | "blue" | "green" | "yellow" | "purple" | "orange" | "pink" | "cyan";
  /**
   * Auto-submitted first user turn when the agent runs as the main session agent (via
   * `--agent`/the `agent` setting); prepended to any user prompt
   * (code.claude.com/docs/en/sub-agents, "Supported frontmatter fields", retrieved 2026-07-15).
   */
  readonly initialPrompt?: string;
  readonly prose?: Prose;
}

/**
 * `agent` — every markdown file under `.claude/agents`, discovered recursively (a
 * containing subdirectory is purely organizational), YAML frontmatter carrying
 * `name` then `description`; identity is the `name` field (never the filename),
 * the named-field mode; registers on the description-trigger channel only — no
 * user-invoked slash command (code.claude.com/docs/en/sub-agents, retrieved
 * 2026-07-15).
 */
export const agent: KindDefinition<Agent> = kind<Agent>({
  name: "agent",
  locus: { kind: "at", root: ".claude/agents", glob: "**/*.md" },
  format: "yaml-frontmatter",
  unitShape: "named-field",
  registration: [{ via: "description-trigger", field: "description" }],
  identityField: "name",
});

/** A Claude Code rule — a flat markdown file with an optional `paths` scope. */
export interface Rule {
 /**
   * The path scope — a present list matching zero files is a dead edge; an absent
   * one loads unconditionally (code.claude.com/docs/en/memory, retrieved 2026-07-15).
 */
  readonly paths?: readonly string[];
  readonly prose?: Prose;
}

/**
 * `rule` — `.claude/rules/<name>.md`, a lone file (identity from the stem), YAML
 * frontmatter; registers a path scope (code.claude.com/docs/en/memory, retrieved
 * 2026-07-15).
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
 * retrieved 2026-07-15): the whole file is the body, so the kind declares no
 * `format`. A project `CLAUDE.md` may sit at either `./CLAUDE.md` or
 * `./.claude/CLAUDE.md` — equal documented locations (same source) — and the
 * any-depth locus (a `CLAUDE.md` at any directory) covers both; a
 * module-carried memory projects the root file.
 */
export const memory: KindDefinition<Memory> = kind<Memory>({
  name: "memory",
  locus: { kind: "at", root: ".", glob: "**/CLAUDE.md" },
  unitShape: "file",
  registration: [{ via: "always" }],
});

/**
 * A Claude Code hook — a fields-only registration member surfacing inside
 * `settings.json`, keyed under its lifecycle event. It owns no artifact of its own; a
 * handler names how it fires (`command`/`http`/`mcp_tool`/`prompt`/`agent`) plus the
 * documented common fields (code.claude.com/docs/en/hooks, retrieved 2026-07-15).
 * Authoring `hook(...)` builds a member whose typed fields fold into its manifest entry;
 * emit erases it into a registration write fact (`emit.ts`).
 */
export interface Hook {
  /** The handler kind — how the hook fires when its event matches. */
  readonly type?: "command" | "http" | "mcp_tool" | "prompt" | "agent";
  /** The shell command or executable a `command` handler runs. */
  readonly command?: string;
  /** Seconds before the handler is canceled. */
  readonly timeout?: number;
  /** The tool-name filter a tool-scoped event fires on (`"*"`/`""`/absent = all). */
  readonly matcher?: string;
}

/**
 * The manifest Claude Code's harness-level settings reside in — the file the assembly's
 * residual settings keys fold into as opaque residue, the same manifest the `hook` kind's
 * registrations surface inside (code.claude.com/docs/en/settings, retrieved 2026-07-16).
 */
export const SETTINGS_MANIFEST = "settings.json";

/**
 * `hook` — a `settings.json` `hooks.<Event>` registration member: a fields-only kind (no
 * body slot), its members discovered off the `.claude/settings.json` manifest at the
 * `hooks.<Event>` collection address, keyed by lifecycle event; registers on the `event`
 * channel (code.claude.com/docs/en/hooks, retrieved 2026-07-15). The first manifest kind
 * temper ships — the read side of 0021's manifest-authoring surface.
 */
export const hook: KindDefinition<Hook> = kind<Hook>({
  name: "hook",
  locus: { kind: "at", root: ".claude", glob: "settings.json" },
  unitShape: "file",
  registration: [{ via: "event", field: "event" }],
  shape: "fields",
  collectionAddress: { manifest: SETTINGS_MANIFEST, keyPath: "hooks.<Event>", entryShape: "group-array(hooks;matcher)" },
});

/**
 * A Claude Code MCP server — a fields-only registration member surfacing inside
 * `.mcp.json`, keyed by name under `mcpServers`. It owns no artifact of its own; its
 * `type` names the transport (`stdio` default, or `http`/`streamable-http`/`sse`/`ws`),
 * and each transport reads a different field set — `command`/`args`/`env` for a local
 * stdio process, `url`/`headers` for a remote connection
 * (code.claude.com/docs/en/mcp, retrieved 2026-07-15). Authoring `mcpServer(...)` builds a
 * member whose typed fields fold into its `mcpServers.*` entry; emit erases it into a
 * registration write fact (`emit.ts`).
 */
export interface McpServer {
  /**
   * The transport. Absent reads as `stdio`, so an entry that carries a `url` but no
   * `type` is a configuration error — Claude Code treats it as a stdio server and skips
   * it. `streamable-http` is an alias for `http`.
   */
  readonly type?: "stdio" | "http" | "streamable-http" | "sse" | "ws";
  /** The executable a stdio server runs. */
  readonly command?: string;
  /** The arguments passed to a stdio server's `command`. */
  readonly args?: readonly string[];
  /** Environment variables set in a stdio server's process. */
  readonly env?: Readonly<Record<string, string>>;
  /** The endpoint a remote (`http`/`sse`/`ws`) server connects to. */
  readonly url?: string;
  /** Static headers sent to a remote server. */
  readonly headers?: Readonly<Record<string, string>>;
  /** Milliseconds before a tool call to this server aborts. */
  readonly timeout?: number;
}

/**
 * `mcpServer` — a `.mcp.json` `mcpServers.*` registration member: a fields-only kind (no
 * body slot), its members discovered off the `.mcp.json` manifest at the `mcpServers.*`
 * collection address, keyed by server name; registers on the `connection` channel
 * (code.claude.com/docs/en/mcp, retrieved 2026-07-15). The second manifest kind temper
 * ships, and the first whose entries are objects — each server's fields fold into the
 * member the read surfaces.
 */
export const mcpServer: KindDefinition<McpServer> = kind<McpServer>({
  name: "mcp-server",
  locus: { kind: "at", root: ".", glob: ".mcp.json" },
  unitShape: "file",
  registration: [{ via: "connection" }],
  shape: "fields",
  collectionAddress: { manifest: ".mcp.json", keyPath: "mcpServers.*", entryShape: "object" },
});

/**
 * A Claude Code installed plugin — a fields-only registration member surfacing inside
 * `.claude/settings.json`, keyed by its `<plugin>@<marketplace>` identity
 * (`formatter@my-marketplace`). It owns no artifact of its own, and unlike a hook (array
 * value) or an MCP server (object value) its entry's value is a bare scalar, so the
 * member carries exactly one field (code.claude.com/docs/en/plugins-reference, retrieved
 * 2026-07-16).
 *
 * The members a plugin *contributes* — its skills, agents, hooks, MCP servers — live in
 * the plugin cache, outside the corpus. Their reach is unmodeled and named as such: this
 * kind types the enablement entry, never the plugin's own surface.
 */
export interface InstalledPlugin {
  /**
   * Whether the harness loads the plugin. Claude Code writes `true` at install or enable
   * time; `false` is a plugin left installed but not loaded, and its documented semantics
   * gate the member off its one channel — the gate rides this field, never a second
   * registration entry.
   */
  readonly enabled: boolean;
}

/**
 * `installedPlugin` — a `settings.json` `enabledPlugins` registration member: a
 * fields-only kind (no body slot), its members discovered off the `.claude/settings.json`
 * manifest at the `enabledPlugins.*` collection address, keyed by plugin identity;
 * registers on the `enablement` channel — the entry's own presence is the registration
 * (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-16). The third manifest
 * kind temper ships, and the first whose entries are scalars.
 */
export const installedPlugin: KindDefinition<InstalledPlugin> = kind<InstalledPlugin>({
  name: "installed-plugin",
  locus: { kind: "at", root: ".claude", glob: "settings.json" },
  unitShape: "file",
  registration: [{ via: "enablement", field: "enabled" }],
  shape: "fields",
  collectionAddress: { manifest: "settings.json", keyPath: "enabledPlugins.*", entryShape: "scalar(enabled)" },
  // The marketplace half of the `<plugin>@<marketplace>` key is a declared edge to the
  // `known-marketplace` member it names (decision 0039). The half is not an authored field
  // — the engine splits it off the composite key at read (`src/kind.rs`, the read-time fold
  // that surfaces it under `marketplace`) — so the edge resolves on the reference graph like
  // any other, and an enablement naming a marketplace no registration declares dangles.
  edgeFields: [{ field: "marketplace", to: ["known-marketplace"] }],
});

/**
 * The default contract for `installed-plugin` — **deliberately empty**. The format
 * documents almost no contract, so it earns an almost-empty default: the honest encoding,
 * not a gap.
 *
 * A charset clause on the `<plugin>@<marketplace>` key has nothing decidable to range
 * over: the key is the member's identity, and the two sources that describe it do not
 * settle a charset — the plugins-reference documents the identity as
 * `formatter@my-marketplace` and schemastore's `claude-code-settings.json` constrains its
 * `enabledPlugins` keys with no `propertyNames` pattern at all (both retrieved
 * 2026-07-16), so a clause against either spelling would forge findings on valid harnesses.
 * The marketplace half of the key is not un-typed, though: it is a declared edge to the
 * `known-marketplace` member it names, resolved on the reference graph rather than by a
 * contract clause — an enablement naming a marketplace no registration declares is a
 * dangling-edge finding, never a convention (decision 0039).
 *
 * `enabled` needs no `required` clause: the type already holds it, and a member that
 * omits it projects the `true` Claude Code itself writes.
 */
export const installedPluginDefaultContract: readonly Clause[] = [];

/**
 * A Claude Code known marketplace — the consumer half of the plugin-distribution graph, a
 * fields-only registration member surfacing inside `.claude/settings.json` under
 * `extraKnownMarketplaces`, keyed by the marketplace name a user has registered. Distinct
 * from the publisher-side {@link Marketplace} catalog: that document is a marketplace's own
 * `.claude-plugin/marketplace.json`, this entry is one consumer's record that they have
 * added it — two documents, two owners. The product stores the same registry per-user in
 * `known_marketplaces.json`; `extraKnownMarketplaces` is the committable, project-scoped
 * spelling temper types (code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-17).
 *
 * Its value is an object (unlike an installed plugin's bare boolean), so the member folds
 * the object's fields: where to fetch the marketplace from, and whether the harness keeps it
 * up to date.
 */
export interface KnownMarketplace {
  /**
   * Where the harness fetches this marketplace from — the same documented `source` union a
   * marketplace lists its plugins by ({@link MarketplaceSource}): a `./`-relative path, or an
   * object naming `github`, `url`, `git-subdir`, or `npm`
   * (code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-17).
   */
  readonly source: MarketplaceSource;
  /**
   * Whether the harness re-fetches the marketplace catalog on its own. Omitting it leaves the
   * harness's default in force; the field only pins the choice
   * (code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-17).
   */
  readonly autoUpdate?: boolean;
}

/**
 * `knownMarketplace` — a `settings.json` `extraKnownMarketplaces` registration member: a
 * fields-only kind (no body slot), its members discovered off the `.claude/settings.json`
 * manifest at the `extraKnownMarketplaces.*` collection address, keyed by marketplace name;
 * registers on the `registry` channel — the entry's own presence is the registration, and
 * whether the marketplace it names actually resolves is a fetch-time fact temper cannot
 * decide, so the channel is never provably dead (code.claude.com/docs/en/plugin-marketplaces,
 * retrieved 2026-07-17). The fourth registration member temper ships.
 */
export const knownMarketplace: KindDefinition<KnownMarketplace> = kind<KnownMarketplace>({
  name: "known-marketplace",
  locus: { kind: "at", root: ".claude", glob: "settings.json" },
  unitShape: "file",
  registration: [{ via: "registry" }],
  shape: "fields",
  collectionAddress: { manifest: "settings.json", keyPath: "extraKnownMarketplaces.*", entryShape: "object" },
});

/**
 * The default contract for `known-marketplace` — **deliberately empty**. The format
 * documents no decidable schema beyond the shape the {@link KnownMarketplace} type already
 * holds: `source` is the same union {@link Marketplace} carries and reuses its typing,
 * `autoUpdate` is a bare optional boolean, and the key is the member's identity (a
 * marketplace name) rather than a declared field a clause could range over. Nothing decidable
 * survives that the type does not already enforce, so the honest encoding is the empty
 * contract, not a forged clause — the `installedPluginDefaultContract` precedent.
 */
export const knownMarketplaceDefaultContract: readonly Clause[] = [];

/**
 * A Claude Code plugin manifest — `.claude-plugin/plugin.json`, the pack's identity and
 * the metadata a marketplace lists it by. The manifest itself is optional (Claude Code
 * auto-discovers components in their default locations and derives the name from the
 * directory); once written, `name` is the only field it must carry
 * (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-16).
 *
 * Every other field is optional and typed. The component path fields do not describe the
 * plugin — they *relocate* its components, adding to or replacing the default scan, so a
 * wrong type on one is a load error rather than bad metadata (same source).
 */
export interface PluginManifest {
  /**
   * The pack's unique identifier — kebab-case, no spaces — and the namespace its
   * components surface under (`plugin-dev:agent-creator`). When a marketplace entry lists
   * the plugin under a different name, the marketplace entry's name is what `enabledPlugins`
   * keys and `/plugin` shows.
   */
  readonly name: string;
  /** JSON Schema URL for editor autocomplete; Claude Code ignores it at load time. */
  readonly $schema?: string;
  /**
   * Human-readable name for the `/plugin` picker, falling back to `name`. May carry spaces
   * and any casing; never used for namespacing or lookup. Requires Claude Code v2.1.143+.
   */
  readonly displayName?: string;
  /**
   * Semantic version. Setting it pins the plugin to that string, so users see an update
   * only when it is bumped; omitting it falls back to the git commit SHA, making every
   * commit a new version. Set in both places, `plugin.json` wins over the marketplace entry.
   */
  readonly version?: string;
  /** Brief explanation of the plugin's purpose. */
  readonly description?: string;
  readonly author?: Readonly<{ name?: string; email?: string; url?: string }>;
  /** Documentation URL. */
  readonly homepage?: string;
  /** Source code URL. */
  readonly repository?: string;
  /** License identifier (`MIT`, `Apache-2.0`). */
  readonly license?: string;
  /** Discovery tags. */
  readonly keywords?: readonly string[];
  /**
   * Whether the plugin starts enabled when the user has not set a state; defaults to
   * `true`. A `enabledPlugins` entry at any settings scope and a dependency requirement
   * each take precedence over it, as does the marketplace entry's own copy of the field.
   * Requires Claude Code v2.1.154+.
   */
  readonly defaultEnabled?: boolean;
  /** Custom skill directories holding `<name>/SKILL.md`; adds to the default `skills/` scan. */
  readonly skills?: readonly string[] | string;
  /** Custom flat `.md` skill files or directories; replaces the default `commands/` scan. */
  readonly commands?: readonly string[] | string;
  /** Custom agent files; replaces the default `agents/` scan. */
  readonly agents?: readonly string[] | string;
  /** Hook config paths, or the hook config inline. */
  readonly hooks?: readonly string[] | string | Readonly<Record<string, unknown>>;
  /** MCP config paths, or the server config inline. */
  readonly mcpServers?: readonly string[] | string | Readonly<Record<string, unknown>>;
  /** Custom output style files/directories; replaces the default `output-styles/` scan. */
  readonly outputStyles?: readonly string[] | string;
  /** Language Server Protocol configs powering code intelligence. */
  readonly lspServers?: readonly string[] | string | Readonly<Record<string, unknown>>;
  /**
   * The components whose manifest schema may still change between releases. Declaring
   * `themes`/`monitors` at the top level instead still works today, but `claude plugin
   * validate` warns and a future release will require them here.
   */
  readonly experimental?: Readonly<{
    themes?: readonly string[] | string;
    monitors?: readonly string[] | string;
  }>;
  /** User-configurable values Claude Code prompts for when the plugin is enabled. */
  readonly userConfig?: Readonly<Record<string, unknown>>;
  /** Channel declarations for message injection. */
  readonly channels?: readonly unknown[];
  /** Other plugins this one requires, optionally with semver constraints. */
  readonly dependencies?: readonly (string | Readonly<{ name: string; version?: string }>)[];
}

/**
 * `plugin-manifest` — `.claude-plugin/plugin.json`, a whole-file JSON document (never
 * frontmatter) whose top-level keys are its fields; identity is read from `name`, the
 * named-field mode, because the file's stem is `plugin` for every manifest ever written.
 * It owns its file, so it carries no collection address, and it is channel-less: a
 * manifest carries distribution metadata rather than session content, so it reaches the
 * model on no channel of its own — what it reaches is the installer
 * (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-16).
 */
export const pluginManifest: KindDefinition<PluginManifest> = kind<PluginManifest>({
  name: "plugin-manifest",
  locus: { kind: "at", root: ".claude-plugin", glob: "plugin.json" },
  format: "json-document",
  unitShape: "named-field",
  registration: [],
  identityField: "name",
});

/**
 * The default contract for `plugin-manifest` — the documented profile of `claude plugin
 * validate --strict`, which is the portable bar: Claude Code's own runtime is deliberately
 * forgiving of unrecognized fields, `--strict` is the CI bar that catches them, and where
 * the two diverge each clause's guidance says so (all facts
 * code.claude.com/docs/en/plugins-reference, retrieved 2026-07-16).
 *
 * The whole profile ships: every documented rule the vocabulary can decide is a clause
 * below, and nothing decidable is held.
 *
 * **Unrecognized top-level fields** — the substance of `--strict` — are the `closedKeys()`
 * clause at the end, over the `required`/`optional` rows above it: Claude Code ignores a
 * key it does not recognize so a manifest doubling as a `package.json` still loads, and
 * `claude plugin validate` warns rather than fails, but `--strict` turns those warnings
 * into the CI errors this contract is the profile of. The declared key set is the union of
 * the two documented sources — the reference's own field tables and the published schema
 * — because a key *either* one recognizes is a key no clause may indict.
 *
 * The component-path fields are gated below rather than held: `type` declares a *set* of
 * lattice kinds, so a field documented `string|array` is checked as the union it is. Each
 * clause declares the widest union its documentation states — a narrower set would reject
 * a documented form, which is a false positive no clause may produce, and the strictest
 * *documented* profile of a union-typed field is the union.
 *
 * Deliberately absent as undecidable: whether the `description` reads well, whether
 * `keywords` aid discovery, whether `name` names the pack aptly.
 *
 * Authoring notes the clauses cannot carry: leave `version` unset while iterating, so the
 * commit SHA drives updates and users are not stranded on a stale pin; set it once the
 * plugin has a release cycle, and bump it every time — pushing commits without bumping is
 * a no-op. Reach for `defaultEnabled: false` when the plugin costs money or scope on load.
 */
export const pluginManifestDefaultContract: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "A manifest declares a `name` — the pack's identity and the namespace its components surface under (`plugin-dev:agent-creator`). Omitting the manifest entirely is supported and Claude Code then derives the name from the directory; a manifest that exists and has no `name` is the one case the loader rejects outright: `name: Invalid input: expected string, received undefined`.",
    cite: "https://code.claude.com/docs/en/plugins-reference#required-fields (retrieved 2026-07-16)",
  }),
  clause(minLen("name", 1), {
    severity: "required",
    guidance:
      "A present-but-empty name cannot namespace a component or key an install. Drop the field to let Claude Code derive the name from the directory, or give it a real one.",
    cite: "https://code.claude.com/docs/en/plugins-reference#required-fields (retrieved 2026-07-16)",
  }),
  clause(allowedChars("name", { ranges: ["a-z", "0-9"], chars: "-" }), {
    severity: "required",
    guidance:
      "Kebab-case, no spaces — lowercase letters, digits, and hyphens. The name is a namespace prefix and an install key, so a space or a capital makes it unquotable in the places it is typed. Use `displayName` for the human-readable label: it may carry spaces and any casing, and is never used for lookup.",
    cite: "https://code.claude.com/docs/en/plugins-reference#required-fields (retrieved 2026-07-16)",
  }),
  clause(forbiddenKeys(["themes", "monitors"]), {
    severity: "required",
    guidance:
      "`themes` and `monitors` are experimental components and belong under the `experimental` key. Declaring them at the top level still loads today and `claude plugin validate` only warns — but `--strict` fails it, and a future release will require `experimental.*`, so a top-level spelling is a migration already scheduled against you.",
    cite: "https://code.claude.com/docs/en/plugins-reference#experimental-components (retrieved 2026-07-16)",
  }),
  clause(type("keywords", ["list"]), {
    severity: "required",
    guidance:
      "`keywords` is an array of discovery tags, and a bare string is not a shorter spelling of a one-tag list: a wrong-typed field is a load error, so the plugin does not load at all — everywhere, not merely under `--strict`, which is the one rule here the forgiving runtime does not wave through. Write `[\"deployment\"]` for a single tag.",
    cite: "https://code.claude.com/docs/en/plugins-reference#unrecognized-fields (retrieved 2026-07-16)",
  }),
  clause(type("skills", ["string", "list"]), {
    severity: "required",
    guidance:
      "`skills` names custom skill directories — each holding a `<name>/SKILL.md` — as one `\"./custom/skills/\"` path or a list of them. Unlike its neighbours it *adds* to the default `skills/` scan rather than replacing it, so listing `\"./skills/\"` alongside your extra path is redundant, not required.",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(type("commands", ["string", "list", "map"]), {
    severity: "required",
    guidance:
      "`commands` names flat `.md` skill files or directories, as one path, a list of them, or an object keyed by command name (each entry carrying `source` or inline `content`). Setting it *replaces* the default `commands/` scan — to keep the default and add to it, list it explicitly: `[\"./commands/\", \"./extras/\"]`. The reference table names only the string and array forms; the object form is the published schema's, so a manifest using it is valid and this clause admits all three.",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17); object form per https://json.schemastore.org/claude-code-plugin-manifest.json (retrieved 2026-07-16)",
  }),
  clause(type("agents", ["string", "list"]), {
    severity: "required",
    guidance:
      "`agents` names custom agent files as one path or a list of them, and *replaces* the default `agents/` scan — a manifest that sets it and expects `agents/` to still be read ships a plugin missing every agent in that folder. Claude Code v2.1.140 and later warns about the ignored folder in `claude plugin list`.",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(type("hooks", ["string", "list", "map"]), {
    severity: "required",
    guidance:
      "`hooks` is a path to a hook config JSON file, a list of them, or the config inline as an object keyed by event name. The inline object is the whole `hooks.json` body, not a fragment of it — the same shape `hooks/hooks.json` carries.",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(type("mcpServers", ["string", "list", "map"]), {
    severity: "required",
    guidance:
      "`mcpServers` is a path to an MCP config JSON file, a list of them, or the config inline as an object keyed by server name — the same shape `.mcp.json` carries. A `channels` entry's `server` must match a key here, so the inline object is the form to reach for when the two are authored together.",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(type("lspServers", ["string", "list", "map"]), {
    severity: "required",
    guidance:
      "`lspServers` is a path to an LSP config JSON file, a list of them, or the config inline as an object keyed by server name — the same shape `.lsp.json` carries. These drive code intelligence (go to definition, find references), so a wrong-typed value costs the capability silently rather than loudly.",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),

  // Every documented top-level key the manifest recognizes, `name` (required, above)
  // aside — the allow-list `closedKeys()` reads, declared here once and consumed there
  // rather than restated as a second list that could disagree with this one. The clauses
  // above refine some of these keys' values; a refinement never declares its key, so a key
  // absent from this block is one `closedKeys()` indicts.
  clause(optional("$schema"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("displayName"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("version"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("description"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("author"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("homepage"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("repository"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("license"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("keywords"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("defaultEnabled"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#metadata-fields (retrieved 2026-07-17)",
  }),
  clause(optional("skills"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("commands"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("agents"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("hooks"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("mcpServers"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("outputStyles"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("lspServers"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("experimental"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("userConfig"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("channels"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  clause(optional("dependencies"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#component-path-fields (retrieved 2026-07-17)",
  }),
  // Recognized at the top level, and separately denied there by the `forbiddenKeys` clause
  // above: the migration is where they are declared, not whether the key is known. Leaving
  // them out of the allow-list would have `closedKeys()` call them unrecognized — false of
  // the format, and a second finding for one mistake.
  clause(optional("themes"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#experimental-components (retrieved 2026-07-17)",
  }),
  clause(optional("monitors"), {
    severity: "required",
    cite: "https://code.claude.com/docs/en/plugins-reference#experimental-components (retrieved 2026-07-17)",
  }),
  // The published schema's own property, absent from the reference's field tables. The
  // allow-list spans both sources: this contract already treats the schema as documentation
  // (the `commands` object form is cited to it), so indicting a key it declares would be a
  // false positive.
  clause(optional("settings"), {
    severity: "required",
    cite: "https://json.schemastore.org/claude-code-plugin-manifest.json (retrieved 2026-07-16)",
  }),

  clause(closedKeys(), {
    severity: "required",
    guidance:
      "This key is not one the manifest format documents. Claude Code ignores an unrecognized top-level field, so the plugin still loads — that is deliberate, and it is what lets one `plugin.json` double as a VS Code or Cursor extension manifest, an npm `package.json`, or an MCPB/DXT bundle manifest. `claude plugin validate` reports it as a warning; `--strict` fails it, which is the CI bar this contract holds, so the usual cause is a typo or a field left over from another tool. If the key is deliberate foreign metadata, this clause is the one to drop from your adopted contract.",
    cite: "https://code.claude.com/docs/en/plugins-reference#unrecognized-fields (retrieved 2026-07-17)",
  }),
];

/**
 * Where a marketplace fetches one listed plugin from — the documented `source` union
 * (code.claude.com/docs/en/plugin-marketplaces, "Plugin sources", retrieved 2026-07-16).
 *
 * A relative-path string is the local form: it must start with `./` and resolves against
 * the *marketplace root* — the directory containing `.claude-plugin/` — never against
 * `.claude-plugin/` itself. The object forms carry a `source` discriminator naming the
 * fetch mechanism.
 *
 * On the three git-based forms (`github`, `url`, `git-subdir`), `ref` names a branch or
 * tag and `sha` an exact commit; when both are set the `sha` is the effective pin.
 */
export type MarketplaceSource =
  | string
  | Readonly<{ source: "github"; repo: string; ref?: string; sha?: string }>
  | Readonly<{ source: "url"; url: string; ref?: string; sha?: string }>
  | Readonly<{ source: "git-subdir"; url: string; path: string; ref?: string; sha?: string }>
  | Readonly<{ source: "npm"; package: string; version?: string; registry?: string }>;

/**
 * One entry in a marketplace's `plugins` array: `name` and `source` are required, and
 * every field of the plugin manifest schema may be restated here alongside the
 * marketplace-specific `source`, `category`, `tags`, `strict`, and `relevance`
 * (code.claude.com/docs/en/plugin-marketplaces, "Plugin entries", retrieved 2026-07-16).
 */
export interface MarketplacePlugin extends Omit<PluginManifest, "name"> {
  /** The public-facing plugin identifier users type: `/plugin install <name>@<market>`. */
  readonly name: string;
  /** Where to fetch this plugin from. */
  readonly source: MarketplaceSource;
  /** Category for organization in the picker. */
  readonly category?: string;
  /** Tags for searchability. */
  readonly tags?: readonly string[];
  /**
   * Whether `plugin.json` is the authority for component definitions (default `true`).
   */
  readonly strict?: boolean;
  /**
   * Signals telling Claude Code when to suggest this plugin. Takes effect only for
   * marketplaces an administrator allowlists in managed settings. Requires v2.1.152+.
   */
  readonly relevance?: Readonly<Record<string, unknown>>;
}

/**
 * `.claude-plugin/marketplace.json` — the distribution catalog
 * (code.claude.com/docs/en/plugin-marketplaces, "Marketplace schema", retrieved
 * 2026-07-16).
 */
export interface Marketplace {
  /**
   * The marketplace identifier (kebab-case, no spaces), public-facing: users see it when
   * installing (`/plugin install my-tool@your-marketplace`). Each user registers only one
   * marketplace per name — adding a second under the same name replaces the first — and
   * the name is checked against a reserved deny list.
   */
  readonly name: string;
  /** The maintainer: `name` required, `email` optional. */
  readonly owner: Readonly<{ name: string; email?: string }>;
  /** The catalog itself — every plugin this marketplace distributes. */
  readonly plugins: readonly MarketplacePlugin[];
  /** JSON Schema URL for editor autocomplete. Claude Code ignores it at load time. */
  readonly $schema?: string;
  /** Brief marketplace description. */
  readonly description?: string;
  /** Marketplace manifest version. */
  readonly version?: string;
  /**
   * `pluginRoot` is a base directory prepended to relative plugin source paths, so
   * `"source": "formatter"` resolves under it. `description`/`version` are also accepted
   * here for backward compatibility.
   */
  readonly metadata?: Readonly<{ pluginRoot?: string; description?: string; version?: string }>;
  /**
   * Other marketplaces whose plugins this marketplace's plugins may depend on. A
   * dependency on a marketplace absent from this list is blocked at install.
   */
  readonly allowCrossMarketplaceDependenciesOn?: readonly string[];
  /**
   * Map from a former plugin `name` to its current name, or to `null` if it was removed —
   * how existing users migrate across a rename. Requires Claude Code v2.1.193 or later.
   */
  readonly renames?: Readonly<Record<string, string | null>>;
}

/**
 * `marketplace` — `.claude-plugin/marketplace.json`, a whole-file JSON document whose
 * top-level keys are its fields; identity from `name`, the named-field mode, because the
 * stem is `marketplace` for every catalog ever written. Like its `plugin-manifest`
 * sibling it owns its file (no collection address) and is channel-less: a catalog is read
 * by the installer, never surfaced to the model
 * (code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-16).
 */
export const marketplace: KindDefinition<Marketplace> = kind<Marketplace>({
  name: "marketplace",
  locus: { kind: "at", root: ".claude-plugin", glob: "marketplace.json" },
  format: "json-document",
  unitShape: "named-field",
  registration: [],
  identityField: "name",
});

/**
 * The reserved marketplace names — reserved for official Anthropic use and refused to a
 * third-party marketplace, re-read from the docs at encode time rather than trusted from
 * memory (code.claude.com/docs/en/plugin-marketplaces, "Marketplace schema", retrieved
 * 2026-07-16). The list grows: before v2.1.205 `first-party-plugins` and `healthcare`
 * were not reserved, so the update ritual is to walk this array against the page, never
 * to re-derive it.
 */
const RESERVED_MARKETPLACE_NAMES: readonly string[] = [
  "claude-code-marketplace",
  "claude-code-plugins",
  "claude-plugins-official",
  "claude-plugins-community",
  "claude-community",
  "anthropic-marketplace",
  "anthropic-plugins",
  "agent-skills",
  "anthropic-agent-skills",
  "knowledge-work-plugins",
  "life-sciences",
  "claude-for-legal",
  "claude-for-financial-services",
  "financial-services-plugins",
  "first-party-plugins",
  "healthcare",
];

/**
 * The default contract for `marketplace` — the strictest documented profile of the catalog
 * format (all facts code.claude.com/docs/en/plugin-marketplaces, retrieved 2026-07-16).
 *
 * The reserved-names clause is the load-bearing one, and it gates a *loud* failure: Claude
 * Code re-checks reserved names on every load, not only on `/plugin marketplace add`, so a
 * catalog published under a name that later becomes reserved stops loading for every user
 * who already added it. That is the one clause here worth more than a lint.
 *
 * The `source` union's per-form required fields are now gated via `when` clauses: decision
 * 0041's Rust implementation shipped in src/contract.rs (Predicate::When, src/engine.rs:1207
 * decide logic), and the SDK's `when()` export has been available since 884a704 — both well
 * before these comments were last touched. The per-source-form requirements now hold via guarded
 * clauses at `plugins[*].source`: the string form needs `leading-dot-slash` shape; each object
 * form (`github`, `url`, `git-subdir`, `npm`) needs its required fields.
 *
 * Deliberately absent as undecidable, and never a clause (`specs/intent.md`, invariant 2):
 * the docs *also* block names that "impersonate official marketplaces" (`official-claude-plugins`,
 * `anthropic-plugins-v2` are the page's own examples). Impersonation is semantic judgment —
 * there is no predicate that decides it, and a clause that guessed would fire on true
 * negatives. The enumerated deny list is the decidable subset; the impersonation rule rides
 * as guidance below.
 *
 * Authoring notes the clauses cannot carry: a relative-path `source` resolves against a
 * *local copy* of the marketplace, so it silently fails to resolve for users who added the
 * marketplace by direct URL to `marketplace.json` — only that one file is downloaded. Reach
 * for `github`, `url`, or `npm` when the catalog is distributed by URL. Where a git source
 * pins both `ref` and `sha`, the `sha` is the effective pin. A marketplace entry's
 * `defaultEnabled` beats the same field in the plugin's own `plugin.json`, while `version`
 * runs the other way — `plugin.json` wins.
 */
export const marketplaceDefaultContract: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "A marketplace declares a `name` — the identity users type when installing from it (`/plugin install my-tool@your-marketplace`) and the key each user registers it under. There is no directory fallback the way `plugin.json` has one.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#required-fields (retrieved 2026-07-16)",
  }),
  clause(minLen("name", 1), {
    severity: "required",
    guidance:
      "A present-but-empty name cannot key an install or qualify a plugin reference. Give it a real one.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#required-fields (retrieved 2026-07-16)",
  }),
  clause(allowedChars("name", { ranges: ["a-z", "0-9"], chars: "-" }), {
    severity: "required",
    guidance:
      "Kebab-case, no spaces — lowercase letters, digits, and hyphens. The name is typed after an `@` to qualify every plugin installed from the marketplace, so a space or a capital makes it unquotable exactly where users type it.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#required-fields (retrieved 2026-07-16)",
  }),
  clause(deny("name", RESERVED_MARKETPLACE_NAMES), {
    severity: "required",
    guidance:
      "This name is reserved for official Anthropic use and a third-party marketplace cannot load under it. Claude Code re-checks the reserved list on every load, not only when the marketplace is added — so publishing under one of these names strands every user who already added you: the marketplace stops loading and reports that it is registered from an untrusted source, and they must remove and re-add it. The list also grows (before v2.1.205, `first-party-plugins` and `healthcare` were not on it). Names that merely *impersonate* an official marketplace — `official-claude-plugins`, `anthropic-plugins-v2` — are blocked too, but impersonation is a judgment no clause can decide: this clause holds the enumerated list, and steering clear of the lookalikes is yours.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#required-fields (retrieved 2026-07-16)",
  }),
  clause(required("owner"), {
    severity: "required",
    guidance:
      "A marketplace names its maintainer. This clause decides the `owner` object's presence; the clause below decides that its `name` is filled.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#owner-fields (retrieved 2026-07-17)",
  }),
  clause(required("owner.name"), {
    severity: "required",
    guidance:
      "`owner.name` is the maintainer or team behind the catalog, and it is required — an `owner` object carrying only `email` does not satisfy the schema. `owner.email` beside it is optional.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#owner-fields (retrieved 2026-07-17)",
  }),
  clause(required("plugins"), {
    severity: "required",
    guidance:
      "The `plugins` array is the catalog — a marketplace without it lists nothing. An empty array is a valid, if empty, catalog.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#required-fields (retrieved 2026-07-17)",
  }),
  clause(required("plugins[*].name"), {
    severity: "required",
    guidance:
      "Every catalog entry declares a `name` — the plugin identifier users type when installing (`/plugin install my-plugin@marketplace`), kebab-case and without spaces. An entry with no name cannot be installed, and this clause names the entry that omitted it by its own index.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-entries (retrieved 2026-07-17)",
  }),
  clause(required("plugins[*].source"), {
    severity: "required",
    guidance:
      "Every catalog entry declares a `source` — where the plugin is fetched from. A listed plugin with no source resolves to nothing. Note that a relative-path source resolves against a *local copy* of the marketplace, so it fails to resolve for users who added the marketplace by direct URL to `marketplace.json`.",
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-entries (retrieved 2026-07-17)",
  }),
  when(type("plugins[*].source", ["string"]), [
    clause(shape("plugins[*].source", "leading-dot-slash"), {
      severity: "required",
      guidance:
        "A relative-path source must start with `./` to resolve correctly against the marketplace root. A path without `./` is treated as a URL or package name, which is not the intent for local files.",
      cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
    }),
  ], {
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
  }),
  when(enumOf("plugins[*].source.source", ["github"]), [
    clause(required("source.repo"), {
      severity: "required",
      guidance:
        "A `github` source entry must specify the `repo` field — the repository in `owner/repo` format — so the plugin can be fetched from GitHub.",
      cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
    }),
  ], {
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
  }),
  when(enumOf("plugins[*].source.source", ["url"]), [
    clause(required("source.url"), {
      severity: "required",
      guidance:
        "A `url` source entry must specify the `url` field — the HTTPS URL to a git repository — so the plugin can be cloned from that location.",
      cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
    }),
  ], {
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
  }),
  when(enumOf("plugins[*].source.source", ["git-subdir"]), [
    clause(required("source.url"), {
      severity: "required",
      guidance:
        "A `git-subdir` source entry must specify the `url` field — the HTTPS URL to the git repository containing the plugin.",
      cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
    }),
    clause(required("source.path"), {
      severity: "required",
      guidance:
        "A `git-subdir` source entry must specify the `path` field — the path within the repository to the plugin's root directory.",
      cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
    }),
  ], {
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
  }),
  when(enumOf("plugins[*].source.source", ["npm"]), [
    clause(required("source.package"), {
      severity: "required",
      guidance:
        "An `npm` source entry must specify the `package` field — the npm package name (e.g., `@scope/package`) — so the plugin can be installed from the npm registry.",
      cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
    }),
  ], {
    cite: "https://code.claude.com/docs/en/plugin-marketplaces#plugin-sources (retrieved 2026-07-16)",
  }),
];

/**
 * A Claude Code local settings overlay — `.claude/settings.local.json`, the machine's own
 * per-project settings at the **local** scope: personal overrides not checked in. Claude
 * Code gitignores the file when it creates it, and one written by hand belongs in
 * `.gitignore` too (code.claude.com/docs/en/settings, "Settings files", retrieved
 * 2026-07-16).
 *
 * Only the documented top-level keys a local overlay commonly carries are typed here; the
 * settings schema is large and version-evolving, and every key not named below survives as
 * opaque residue named as such — the partial-governance posture 0036 settles. This is
 * *fields*, not members: a hook registered under `hooks` or a plugin enablement under
 * `enabledPlugins` here stays part of that field's opaque value, never a modeled `hook`/
 * `installed-plugin` member of its own (those kinds read the committed `settings.json`,
 * not this uncommitted overlay).
 */
export interface SettingsLocal {
  /** JSON Schema URL for editor autocomplete; Claude Code ignores it at load time. */
  readonly $schema?: string;
  /** Tool-permission rules: `{ allow, ask, deny }`, each an array of rule strings. */
  readonly permissions?: Readonly<Record<string, unknown>>;
  /** Environment variables applied to every session, a map of string to string. */
  readonly env?: Readonly<Record<string, string>>;
  /** Event hook handlers, keyed by lifecycle event — the hooks-configuration object. */
  readonly hooks?: Readonly<Record<string, unknown>>;
  /** The model this project runs as. */
  readonly model?: string;
  /** Whether project MCP servers from `.mcp.json` are auto-approved. */
  readonly enableAllProjectMcpServers?: boolean;
  /** MCP servers from `.mcp.json` to approve. */
  readonly enabledMcpjsonServers?: readonly string[];
  /** MCP servers from `.mcp.json` to reject. */
  readonly disabledMcpjsonServers?: readonly string[];
  /** Whether to add a Claude co-author trailer to git commits. */
  readonly includeCoAuthoredBy?: boolean;
  /** The output rendering style. */
  readonly outputStyle?: string;
}

/**
 * `settings-local` — `.claude/settings.local.json`, a whole-file JSON document at the
 * **local** commitment class: read in place at check and gated, never an `emit` input or
 * target, its rows derived at read time and no row of it ever landing in the lock. Its
 * top-level keys are its fields; identity is the fixed singleton stem `settings.local` (the
 * `file` unit shape — every machine's overlay is the one file at this path, so no declared
 * key names it). Channel-less: machine configuration read by the harness, never surfaced to
 * the model (code.claude.com/docs/en/settings, retrieved 2026-07-16; decisions
 * 0032/0034/0036).
 */
export const settingsLocal: KindDefinition<SettingsLocal> = kind<SettingsLocal>({
  name: "settings-local",
  locus: { kind: "at", root: ".claude", glob: "settings.local.json", commitment: "local" },
  format: "json-document",
  unitShape: "file",
  registration: [],
});

/**
 * The default contract for `settings-local` — deliberately near-empty. The settings format
 * documents a large, version-evolving key set (many keys managed-scope-only, most of them
 * scalar preferences), so a `closedKeys()` allow-list would strand every valid local
 * overlay the moment upstream adds a key: 0036 settles the residue *opaque*, not indicted.
 * What stays decidable is the shape of the few structural container keys a local overlay
 * carries — `permissions`, `env`, and `hooks` are each a documented JSON *object*, and a
 * value that is not one cannot be applied — so each is gated as a `map` and everything else
 * rides opaque. (All facts code.claude.com/docs/en/settings, retrieved 2026-07-16.)
 *
 * Deliberately absent as undecidable: whether a permission rule reads correctly, whether an
 * env var is one this machine needs, whether the chosen `model` exists — semantic judgment,
 * never a gate clause.
 */
export const settingsLocalDefaultContract: readonly Clause[] = [
  clause(type("permissions", ["map"]), {
    severity: "required",
    guidance:
      "`permissions` is the tool-permission object — `{ allow, ask, deny }`, each an array of rule strings. A value that is not an object carries no rules Claude Code can read, so the permission overlay this file exists to hold silently applies nothing.",
    cite: "https://code.claude.com/docs/en/settings#permission-settings (retrieved 2026-07-16)",
  }),
  clause(type("env", ["map"]), {
    severity: "required",
    guidance:
      "`env` is a map of environment variables — string keys to string values — applied to every session. A non-object value is not a shorter spelling of one variable; it is a shape the loader cannot expand, so none of the variables take effect.",
    cite: "https://code.claude.com/docs/en/settings#available-settings (retrieved 2026-07-16)",
  }),
  clause(type("hooks", ["map"]), {
    severity: "required",
    guidance:
      "`hooks` is the hooks-configuration object, keyed by lifecycle event. A locally-registered hook lives inside this object as opaque residue — it is not modeled as a `hook` member of its own, since that kind reads the committed `settings.json`. A value that is not an object registers no hooks at all.",
    cite: "https://code.claude.com/docs/en/settings#available-settings (retrieved 2026-07-16)",
  }),
];

/**
 * The default contract for `skill` — Anthropic's documented skill contract: the Agent
 * Skills open standard (agentskills.io), Anthropic's platform upload
 * validation, and Claude Code's own docs.
 * All sources retrieved 2026-07-15.
 *
 * Checks the strictest documented profile: the spec and upload validation are
 * hard, Claude Code's runtime is deliberately forgiving ("All fields are
 * optional") — a skill that passes here loads everywhere the format is
 * honored, not merely on the machine it was written on; where the runtime
 * diverges, the clause's own guidance says so.
 *
 * Deliberately absent — undecidable, so never gate clauses: whether the
 * description actually triggers well or reads third-person (semantic);
 * vagueness/no-op detection (semantic); gerund naming (judgment). Nothing
 * decidable is held: the name's hyphen placement and the platform's "no XML
 * tags in the description" are the two `shape` clauses below.
 *
 * Authoring notes the clauses cannot carry: prefer gerund or noun-phrase
 * names (`processing-pdfs`, `pdf-processing`) over vague ones (`helper`,
 * `utils`); `disable-model-invocation: true` for side-effectful workflows you
 * want to time yourself; `user-invocable: false` for background knowledge
 * that is not a command; `metadata` is the sanctioned home for versioning —
 * there is no top-level `version` field.
 */
export const skillDefaultContract: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "Every skill declares a `name` — the slug the harness binds to. Claude Code alone would default it from the directory name, but a nameless skill is not portable: the spec and Anthropic's upload validation both require it.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(minLen("name", 1), {
    severity: "required",
    guidance: "A present-but-empty name fails the spec's 1-64 character bound.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(allowedChars("name", { ranges: ["a-z", "0-9"], chars: "-" }), {
    severity: "required",
    guidance:
      "Lowercase letters, digits, and hyphens only — `PDF-Processing` is the spec's own counter-example. The charset also keeps XML out of the name, which Anthropic's upload validation separately forbids.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(maxLen("name", 64), {
    severity: "required",
    guidance: "Keep the name short and slug-like; it becomes a directory and an id.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(shape("name", "hyphen-placement"), {
    severity: "required",
    guidance:
      "A hyphen separates segments, so it may not lead, trail, or double — `-pdf`, `pdf-`, and `pdf--processing` are the spec's own counter-examples. The charset clause above admits the hyphen; this places it.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-17)",
  }),
  clause(deny("name", ["anthropic", "claude"]), {
    severity: "required",
    guidance:
      "Reserved words, enforced by Anthropic's platform upload validation (not by the open spec, and not by Claude Code's runtime — which itself ships a `claude-api` skill). Keep them out if the skill will ever travel through the API or claude.ai.",
    cite: "https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview#skill-structure (retrieved 2026-07-15)",
  }),
  clause(nameMatchesDir(), {
    severity: "required",
    guidance:
      "The spec requires the name to match its parent directory. Claude Code decouples the two (the frontmatter name is a display label; the directory names the slash command, except for a plugin-root SKILL.md) — but a mismatch is a portability trap and a reader trap even where it loads.",
    cite: "https://agentskills.io/specification#name-field (retrieved 2026-07-15)",
  }),
  clause(required("description"), {
    severity: "required",
    guidance:
      "The description is how the model chooses this skill from potentially 100+ available — it is the skill's API. Claude Code would fall back to the body's first paragraph; the spec and upload validation require it declared.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(minLen("description", 1), {
    severity: "required",
    guidance:
      "Say both what the skill does and when to use it, with the keywords a user would naturally say. Write in third person — the text is injected into the system prompt, and inconsistent point-of-view causes discovery problems.",
    cite: "https://agentskills.io/specification#description-field (retrieved 2026-07-15)",
  }),
  clause(maxLen("description", 1024), {
    severity: "required",
    guidance:
      "The spec's cap. Claude Code additionally truncates the skill listing at 1,536 combined characters (description + when_to_use) — truncation, not rejection, but text past the fold cannot help the model choose.",
    cite: "https://agentskills.io/specification#description-field (retrieved 2026-07-15)",
  }),
  clause(shape("description", "no-xml-tags"), {
    severity: "required",
    guidance:
      "Anthropic's platform upload validation rejects a description carrying an XML tag — the description is injected into the system prompt, where a tag is markup rather than text. Neither the open spec nor Claude Code's runtime enforces it, so this is the clause that keeps a skill portable through the API and claude.ai. Prose spelling a comparison (`use when x < y`) is not a tag and does not fire.",
    cite: "https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview#skill-structure (retrieved 2026-07-17)",
  }),
  clause(maxLen("compatibility", 500), {
    severity: "required",
    guidance: "Optional field; when present the spec caps it at 500 characters. Most skills do not need it.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(extent("lines", 500), {
    severity: "advisory",
    guidance:
      "Progressive disclosure: keep SKILL.md under 500 rendered lines and move detailed reference material to separate files, one level deep. Measured render-side — the lines the projected artifact contributes to context, not the source body before any include resolves. Once a skill loads, its body stays in context across turns — every line is a recurring token cost. The context window is a public good.",
    cite: "https://agentskills.io/specification#progressive-disclosure (retrieved 2026-07-15)",
  }),
  clause(forbiddenKeys(["globs", "alwaysApply"]), {
    severity: "required",
    guidance:
      "Cursor `.mdc` keys. Nothing in the Agent Skills spec or Claude Code's documented frontmatter accepts them — a skill authored with them is carrying dead configuration that another tool's semantics silently fail to apply.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(globValid("paths"), {
    severity: "required",
    guidance:
      "The optional `paths` scope gates every invocation channel until Claude reads a file its globs match; each entry is a glob (brace expansion supported). An unparseable pattern — an unclosed `[`, say — is invalid under globset and silently matches nothing, so the gate never opens and the skill never registers, with no error surfaced. Fix the pattern or drop the field.",
    cite: "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-15)",
  }),
];

/**
 * The default contract for `supporting-doc` — one clause, because the format documents
 * exactly one thing about a supporting file that is decidable, and it is not a fact
 * about the file's own contents: a supporting file is prose Claude reads when the
 * skill's body points at it, so an unreferenced one is never read at all
 * (code.claude.com/docs/en/skills, "Add supporting files", retrieved 2026-07-16). The
 * format carries no frontmatter schema, no required field and no cap of its own, so
 * nothing else joins it — an almost-empty format gets an almost-empty contract, and
 * manufacturing a second clause would fake a check the format does not carry.
 *
 * The reach bound is a property of the file's *place in the graph*, not its bytes, and
 * the fact holds of every supporting document — so it is spelled as the by-kind
 * universal binding at the `each` grain (`model/contract.md`, "selection"), never as a
 * requirement: a requirement is the opt-in selector, and routing a vendor fact through
 * one would make the harness's own truth a consumer's ceremony.
 */
export const supportingDocDefaultContract: readonly Clause[] = [
  clause(degree({ incoming: { min: 1 } }), {
    severity: "advisory",
    guidance:
      "Reference the file from `SKILL.md` — a supporting file the skill's body never points at is invisible: Claude has no way to learn what it holds or when to load it, and it ships as dead weight in the bundle. Any resolved edge from the host skill counts, a mention included; what the edge cannot decide is whether the reference tells Claude *when* to follow it, and that sentence is the point of writing one.",
    cite: "https://code.claude.com/docs/en/skills (retrieved 2026-07-16)",
  }),
];

/**
 * The default contract for `command` — `skillDefaultContract`'s clauses minus `nameMatchesDir`: a
 * command is a lone file with no parent directory to match, so the one clause
 * that ranges over the directory relationship does not apply; every other
 * documented skill-schema recommendation, name-requiredness included, still
 * governs a command by the same import (code.claude.com/docs/en/skills,
 * retrieved 2026-07-15).
 */
export const commandDefaultContract: readonly Clause[] = skillDefaultContract.filter(
  (entry) => entry.predicate.key !== "name-matches-dir",
);

/**
 * The default contract for `agent` — Anthropic's documented subagent contract
 * (code.claude.com/docs/en/sub-agents, retrieved 2026-07-15): `name` and
 * `description` are the only required fields, `name` is a "unique identifier
 * using lowercase letters and hyphens" (no digits, unlike a skill's `name`), and
 * "keep `name` values unique across the whole tree" — a same-scope collision
 * loads only one definition.
 *
 * Deliberately narrow, like `ruleDefaultContract`: undecidable properties (whether the
 * description triggers well, model/permissionMode's semi-open vocabularies) stay
 * out of the gate — the format documents little else that is decidable.
 */
export const agentDefaultContract: readonly Clause[] = [
  clause(required("name"), {
    severity: "required",
    guidance:
      "Every subagent declares a `name` — its unique identifier. Claude Code binds identity to this field alone, never the filename, so a nameless subagent cannot be delegated to.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
  clause(allowedChars("name", { ranges: ["a-z"], chars: "-" }), {
    severity: "required",
    guidance:
      "Lowercase letters and hyphens only — no digits, unlike a skill's `[a-z0-9-]` name. Hooks receive this value as `agent_type`.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
  clause(uniqueName(), {
    severity: "required",
    guidance:
      "Keep `name` values unique across the whole tree — when two files in one scope declare the same name, Claude Code loads only one of them, silently shadowing the other.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
  clause(required("description"), {
    severity: "required",
    guidance:
      "The description is how Claude decides when to delegate to this subagent — write it so the trigger is unambiguous.",
    cite: "https://code.claude.com/docs/en/sub-agents (retrieved 2026-07-15)",
  }),
];

/**
 * The default contract for `rule` — Anthropic's documented contract for a Claude Code
 * rules file, sourced from the memory docs (`.claude/rules/` landed in
 * v2.0.64). All sources retrieved 2026-07-15.
 *
 * `paths` is the one documented frontmatter key for rules: glob patterns
 * (brace expansion supported) that scope the rule to matching files. Rules
 * without it load at launch with the same priority as CLAUDE.md; path-scoped
 * rules load when Claude reads a matching file. Note skills now take a
 * `paths` key too — the two schemas are separate. (Guidance only: an
 * optional field asserts nothing decidable, so it carries no clause of its
 * own: `required` is the one
 * presence predicate, and its absence is not itself a predicate.)
 * https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-15)
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
export const ruleDefaultContract: readonly Clause[] = [
  clause(forbiddenKeys(["description", "globs", "alwaysApply"]), {
    severity: "required",
    guidance:
      "Cursor `.mdc` keys. Claude Code's documented rules schema is `paths`-only; a rule authored with Cursor frontmatter is configuration another tool's semantics silently fail to honor — the rule loads, the scoping you meant does not. (That Claude Code ignores unknown keys is observed behavior, not documented contract — the documented schema is the citation.)",
    cite: "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-15)",
  }),
  clause(globValid("paths"), {
    severity: "required",
    guidance:
      "`paths` is the one documented rules key: globs (brace expansion supported) that scope the rule to matching files. An unparseable pattern — an unclosed `[`, say — is invalid under globset and silently matches nothing, so the rule never loads where you meant it to, with no error surfaced. Fix the pattern or drop it.",
    cite: "https://code.claude.com/docs/en/memory#path-specific-rules (retrieved 2026-07-15)",
  }),
  clause(extent("lines", 200), {
    severity: "advisory",
    guidance:
      "Unconditional rules are always-on context, paid every session: the docs' size target is under 200 rendered lines per memory file — 'longer files consume more context and reduce adherence.' Measured render-side, off the projected artifact rather than the source body. (Distinct from the hard 200-line/25KB cutoff, which applies only to auto-memory MEMORY.md; rules load in full regardless of length.) For each line ask: would removing it cause Claude to make mistakes? If not, cut it.",
    cite: "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-15)",
  }),
  clause(mentionReachable("paths", "paths"), {
    severity: "advisory",
    guidance:
      "A mention of a gated member is actionable only where that member can be invoked. A `paths` gate removes its member from every invocation channel until Claude reads a matching file, and invoking a gated member from outside its gate hard-errors (`Unknown skill`) — the harness then tells the user it doesn't exist. So a rule that loads where its target cannot be invoked hands Claude an obligation it cannot act on. Two remedies: scope this rule's `paths` to the target's gate, or ungate the target. Advisory because the containment test is literal — every glob here must appear verbatim in the gate — so a semantically narrower glob (`src/**/*.ts` inside `src/**`) false-fires; retune or drop this clause in your own contract when it does.",
    cite: "https://code.claude.com/docs/en/skills (retrieved 2026-07-16; gating hard-error verified against 2.1.211)",
  }),
];

/**
 * The default contract for the qualified `claude-code.memory` kind — Anthropic's
 * documented contract for a project `CLAUDE.md`. Retrieved 2026-07-15.
 *
 * Deliberately near-empty, because the format is: `CLAUDE.md` is plain
 * markdown with no documented frontmatter and no required fields
 * (code.claude.com/docs/en/memory, retrieved 2026-07-15), so there is no
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
export const memoryAnthropicDefaultContract: readonly Clause[] = [
  clause(extent("lines", 200), {
    severity: "advisory",
    guidance:
      "CLAUDE.md is always-on context, paid every session. The memory docs' size target is under 200 rendered lines per memory file — 'longer files consume more context and reduce adherence.' Measured render-side, off the projected artifact rather than the source body. For each line ask: would removing it cause Claude to make mistakes? If not, cut it. (Advisory: Claude Code loads the file in full regardless of length; this is a context-cost budget, not a hard cutoff.)",
    cite: "https://code.claude.com/docs/en/memory#write-effective-instructions (retrieved 2026-07-15)",
  }),
];

/**
 * Every documented Claude Code hook lifecycle event — the closed set a `hooks.<Event>`
 * key is drawn from (code.claude.com/docs/en/hooks, "Hook events", retrieved 2026-07-15).
 * The allowlist the `hook` default contract's one decidable clause ranges over; the
 * update ritual when the docs add an event is to re-fetch and extend this set, never to
 * re-derive from memory.
 */
const DOCUMENTED_HOOK_EVENTS = [
  "SessionStart",
  "Setup",
  "UserPromptSubmit",
  "UserPromptExpansion",
  "PreToolUse",
  "PermissionRequest",
  "PermissionDenied",
  "PostToolUse",
  "PostToolUseFailure",
  "PostToolBatch",
  "Notification",
  "MessageDisplay",
  "SubagentStart",
  "SubagentStop",
  "TaskCreated",
  "TaskCompleted",
  "Stop",
  "StopFailure",
  "TeammateIdle",
  "InstructionsLoaded",
  "ConfigChange",
  "CwdChanged",
  "FileChanged",
  "WorktreeCreate",
  "WorktreeRemove",
  "PreCompact",
  "PostCompact",
  "Elicitation",
  "ElicitationResult",
  "SessionEnd",
] as const;

/**
 * The default contract for `hook` — Anthropic's documented hooks contract
 * (code.claude.com/docs/en/hooks, retrieved 2026-07-15). A hook surfaces at
 * `hooks.<Event>`, so the member the gate reads is the lifecycle event itself, its name
 * carried as the `event` field off the collection key. The one decidable, cited property
 * of that member is its event: a key outside the documented set is dead configuration —
 * Claude Code silently never fires a hook under an unrecognized event, so the strictest
 * documented profile is that the event is one temper's cited docs name.
 *
 * **Re-examined against decision 0041's widened vocabulary and confirmed to still hold**:
 * The handler's own schema (`type`/`command`/`url`/`timeout`, the matcher grammar) lives
 * one array level deeper than `hooks.<Event>`, inside each event's matcher-group list. The
 * collection address `hooks.<Event>` does not walk into arrays, and even with the guard
 * vocabulary's when/enumOf/type extensions, addressing still cannot spell a path into the
 * handler array (e.g., `hooks.<Event>[0].type`). A clause over it would range over a field
 * the read never surfaces, so it is no clause at all — the addressing-reach gap remains.
 * What the clauses cannot carry, as guidance: keep a handler's `type` among
 * `command`/`http`/`mcp_tool`/`prompt`/`agent`; a `command` handler needs a `command`, an
 * `http` handler a `url`; the `matcher` filters tool-scoped events and is inert on events
 * that carry no tool (`UserPromptSubmit`, `Stop`, and their siblings).
 */
export const hookDefaultContract: readonly Clause[] = [
  clause(enumOf("event", DOCUMENTED_HOOK_EVENTS), {
    severity: "required",
    guidance:
      "A hook keys under its lifecycle event; an event outside the documented set is dead configuration — Claude Code silently never fires a hook under an unrecognized event. If this is a newly-documented event, re-fetch code.claude.com/docs/en/hooks and extend temper's cited set rather than working around the finding.",
    cite: "https://code.claude.com/docs/en/hooks (retrieved 2026-07-15)",
  }),
];

/**
 * Every documented `.mcp.json` server transport — the closed set a server entry's `type`
 * is drawn from (code.claude.com/docs/en/mcp, retrieved 2026-07-15). `stdio` is the
 * default when `type` is absent; `streamable-http` is the MCP spec's own name for `http`,
 * accepted as an alias so configurations copied from server docs work unchanged; `sse` is
 * documented but deprecated; `ws` is the WebSocket transport. The update ritual when the
 * docs add a transport is to re-fetch and extend this set, never to re-derive from memory.
 */
const DOCUMENTED_MCP_TRANSPORTS = ["stdio", "http", "streamable-http", "sse", "ws"] as const;

/**
 * The default contract for `mcpServer` — Anthropic's documented `.mcp.json` contract
 * (code.claude.com/docs/en/mcp, retrieved 2026-07-15). A server surfaces at `mcpServers.*`,
 * keyed by name, its transport-specific fields folded into the member. The one decidable,
 * cited property that holds across every transport is `type`: a value outside the
 * documented set is a transport Claude Code cannot honor, so the strictest documented
 * profile is that a present `type` names one temper's cited docs carry. An absent `type`
 * passes — Claude Code reads it as `stdio`, the documented default.
 *
 * The per-transport requirements are now gated via `when` clauses: a stdio server (type
 * absent or `stdio`) needs a `command`, and a remote server (type `http`, `streamable-http`,
 * `sse`, or `ws`) needs a `url`. Decision 0041's Rust implementation shipped in
 * src/contract.rs and the SDK's `when()` export has been available since 884a704 — both
 * well before these comments were last written.
 */
export const mcpServerDefaultContract: readonly Clause[] = [
  clause(enumOf("type", DOCUMENTED_MCP_TRANSPORTS), {
    severity: "required",
    guidance:
      "A server's `type` names its transport; a value outside the documented set is one Claude Code cannot honor. Absent reads as `stdio`, the documented default. If this is a newly-documented transport, re-fetch code.claude.com/docs/en/mcp and extend temper's cited set rather than working around the finding.",
    cite: "https://code.claude.com/docs/en/mcp (retrieved 2026-07-15)",
  }),
  when(enumOf("type", ["stdio"]), [
    clause(required("command"), {
      severity: "required",
      guidance:
        "A stdio server must specify a `command` — the executable path and arguments to run the server process. Without it, Claude Code cannot start the server. (Note: when `type` is absent, Claude Code reads it as `stdio`; a `command` is also required in that case.)",
      cite: "https://code.claude.com/docs/en/mcp#transport-options (retrieved 2026-07-15)",
    }),
  ], {
    cite: "https://code.claude.com/docs/en/mcp#transport-options (retrieved 2026-07-15)",
  }),
  when(enumOf("type", ["http", "streamable-http", "sse", "ws"]), [
    clause(required("url"), {
      severity: "required",
      guidance:
        "A remote server (http, streamable-http, sse, or ws transport) must specify a `url` — the endpoint to connect to. Without it, Claude Code cannot establish the connection.",
      cite: "https://code.claude.com/docs/en/mcp#transport-options (retrieved 2026-07-15)",
    }),
  ], {
    cite: "https://code.claude.com/docs/en/mcp#transport-options (retrieved 2026-07-15)",
  }),
];
