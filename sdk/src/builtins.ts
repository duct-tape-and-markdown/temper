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
  degree,
  deny,
  enumOf,
  forbiddenKeys,
  globValid,
  maxLen,
  maxLines,
  mentionReachable,
  minLen,
  nameMatchesDir,
  required,
  uniqueName,
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
  collectionAddress: { manifest: "settings.json", keyPath: "hooks.<Event>" },
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
  collectionAddress: { manifest: ".mcp.json", keyPath: "mcpServers.*" },
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
  registration: [{ via: "enablement" }],
  shape: "fields",
  collectionAddress: { manifest: "settings.json", keyPath: "enabledPlugins.*" },
});

/**
 * The default contract for `installed-plugin` — **deliberately empty**. The format
 * documents almost no contract, so it earns an almost-empty default: the honest encoding,
 * not a gap.
 *
 * The one clause a reader would reach for — a shape check on the
 * `<plugin>@<marketplace>` key — has nothing decidable to range over. The key is the
 * member's identity, not a declared field, and the two sources that describe it do not
 * settle a charset: the plugins-reference documents the identity as
 * `formatter@my-marketplace` and schemastore's `claude-code-settings.json` constrains its
 * `enabledPlugins` keys with no `propertyNames` pattern at all (both retrieved
 * 2026-07-16). A clause against either spelling would forge findings on valid harnesses.
 *
 * `enabled` needs no `required` clause: the type already holds it, and a member that
 * omits it projects the `true` Claude Code itself writes.
 */
export const installedPluginDefaultContract: readonly Clause[] = [];

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
 * vagueness/no-op detection (semantic); gerund naming (judgment). Two
 * decidable spec rules are also absent, pending a vocabulary addition (a
 * narrow shape predicate governs additions): the name
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
  clause(maxLen("compatibility", 500), {
    severity: "required",
    guidance: "Optional field; when present the spec caps it at 500 characters. Most skills do not need it.",
    cite: "https://agentskills.io/specification#frontmatter (retrieved 2026-07-15)",
  }),
  clause(maxLines(500), {
    severity: "advisory",
    guidance:
      "Progressive disclosure: keep SKILL.md under 500 lines and move detailed reference material to separate files, one level deep. Once a skill loads, its body stays in context across turns — every line is a recurring token cost. The context window is a public good.",
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
  clause(maxLines(200), {
    severity: "advisory",
    guidance:
      "Unconditional rules are always-on context, paid every session: the docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' (Distinct from the hard 200-line/25KB cutoff, which applies only to auto-memory MEMORY.md; rules load in full regardless of length.) For each line ask: would removing it cause Claude to make mistakes? If not, cut it.",
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
  clause(maxLines(200), {
    severity: "advisory",
    guidance:
      "CLAUDE.md is always-on context, paid every session. The memory docs' size target is under 200 lines per memory file — 'longer files consume more context and reduce adherence.' For each line ask: would removing it cause Claude to make mistakes? If not, cut it. (Advisory: Claude Code loads the file in full regardless of length; this is a context-cost budget, not a hard cutoff.)",
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
 * Deliberately absent — the handler's own schema (`type`/`command`/`url`/`timeout`, the
 * matcher grammar) lives one array level deeper than `hooks.<Event>`, inside each event's
 * matcher-group list, which the collection address does not walk into; a clause over it
 * would range over a field the read never surfaces, so it is no clause at all. What the
 * clauses cannot carry, as guidance: keep a handler's `type` among
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
 * Deliberately absent — the per-transport requirements are conditional on `type`, which no
 * single-field clause can decide: a `url` with no `type` is a configuration error (Claude
 * Code reads it as a stdio server and skips it), a stdio server needs a `command`, and a
 * remote server needs a `url` — each a two-field implication the closed predicate
 * vocabulary cannot express, so it rides guidance rather than a clause that would range
 * over a field the shape of the check cannot see.
 */
export const mcpServerDefaultContract: readonly Clause[] = [
  clause(enumOf("type", DOCUMENTED_MCP_TRANSPORTS), {
    severity: "required",
    guidance:
      "A server's `type` names its transport; a value outside the documented set is one Claude Code cannot honor. Absent reads as `stdio` — but an entry that carries a `url` with no `type` is then a configuration error, because Claude Code treats it as a stdio server and skips it: add `type: \"http\"` (or `sse`/`ws`). A stdio server needs a `command`; a remote server needs a `url`. If this is a newly-documented transport, re-fetch code.claude.com/docs/en/mcp and extend temper's cited set rather than working around the finding.",
    cite: "https://code.claude.com/docs/en/mcp (retrieved 2026-07-15)",
  }),
];
