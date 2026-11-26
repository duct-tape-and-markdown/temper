<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `plugin-manifest` kind

leave `version` unset while iterating, so the commit SHA drives updates and users are not stranded on a stale pin; set it once the plugin has a release cycle, and bump it every time — pushing commits without bumping is a no-op. Reach for `defaultEnabled: false` when the plugin costs money or scope on load.

## Locus

Each member is its own document, at:

- `.claude-plugin/plugin.json`

## Clauses

- `required` on `name` (required): A manifest declares a `name` — the pack's identity and the namespace its components surface under (`plugin-dev:agent-creator`). Omitting the manifest entirely is supported and Claude Code then derives the name from the directory; a manifest that exists and has no `name` is the one case the loader rejects outright: `name: Invalid input: expected string, received undefined`.
- `min_len` on `name` (required): A present-but-empty name cannot namespace a component or key an install. Drop the field to let Claude Code derive the name from the directory, or give it a real one.
- `allowed_chars` on `name` (required): Kebab-case, no spaces — lowercase letters, digits, and hyphens. The name is a namespace prefix and an install key, so a space or a capital makes it unquotable in the places it is typed. Use `displayName` for the human-readable label: it may carry spaces and any casing, and is never used for lookup.
- `forbidden_keys` (required): `themes` and `monitors` are experimental components and belong under the `experimental` key. Declaring them at the top level still loads today and `claude plugin validate` only warns — but `--strict` fails it, and a future release will require `experimental.*`, so a top-level spelling is a migration already scheduled against you.
- `type` on `keywords` (required): `keywords` is an array of discovery tags, and a bare string is not a shorter spelling of a one-tag list: a wrong-typed field is a load error, so the plugin does not load at all — everywhere, not merely under `--strict`, which is the one rule here the forgiving runtime does not wave through. Write `["deployment"]` for a single tag.
- `type` on `skills` (required): `skills` names custom skill directories — each holding a `<name>/SKILL.md` — as one `"./custom/skills/"` path or a list of them. Unlike its neighbours it *adds* to the default `skills/` scan rather than replacing it, so listing `"./skills/"` alongside your extra path is redundant, not required.
- `type` on `commands` (required): `commands` names flat `.md` skill files or directories, as one path, a list of them, or an object keyed by command name (each entry carrying `source` or inline `content`). Setting it *replaces* the default `commands/` scan — to keep the default and add to it, list it explicitly: `["./commands/", "./extras/"]`. The reference table names only the string and array forms; the object form is the published schema's, so a manifest using it is valid and this clause admits all three.
- `type` on `agents` (required): `agents` names custom agent files as one path or a list of them, and *replaces* the default `agents/` scan — a manifest that sets it and expects `agents/` to still be read ships a plugin missing every agent in that folder. Claude Code v2.1.140 and later warns about the ignored folder in `claude plugin list`.
- `type` on `hooks` (required): `hooks` is a path to a hook config JSON file, a list of them, or the config inline as an object keyed by event name. The inline object is the whole `hooks.json` body, not a fragment of it — the same shape `hooks/hooks.json` carries.
- `type` on `mcpServers` (required): `mcpServers` is a path to an MCP config JSON file, a list of them, or the config inline as an object keyed by server name — the same shape `.mcp.json` carries. A `channels` entry's `server` must match a key here, so the inline object is the form to reach for when the two are authored together.
- `type` on `lspServers` (required): `lspServers` is a path to an LSP config JSON file, a list of them, or the config inline as an object keyed by server name — the same shape `.lsp.json` carries. These drive code intelligence (go to definition, find references), so a wrong-typed value costs the capability silently rather than loudly.
- `optional` on `$schema` (required): `$schema` is a JSON Schema URL for editor autocomplete; Claude Code ignores it at load time, so it costs nothing at runtime.
- `optional` on `displayName` (required): `displayName` is the name the UI shows in place of `name` — it may carry spaces and any casing and is never used for lookup. A marketplace entry's own `displayName` takes precedence over this one.
- `optional` on `version` (required): `version` is not checked as semver, and setting it pins users to that version until you change it — so bump it with every release, or leave it out.
- `optional` on `description` (required): `description` is a short explanation of what the plugin provides, shown wherever the plugin is listed.
- `optional` on `author` (required): `author` is an object: `name` is required, `email` and `url` optional.
- `optional` on `homepage` (required): `homepage` is the plugin's documentation URL, and it must parse as a URL.
- `optional` on `repository` (required): `repository` is the source repository URL; Claude Code does not validate it.
- `optional` on `license` (required): `license` is an SPDX identifier such as `MIT`.
- `optional` on `keywords` (required): `keywords` is an array of discovery tags.
- `optional` on `defaultEnabled` (required): `defaultEnabled` (a boolean, default `true`) sets whether the plugin starts enabled when the user has no `enabledPlugins` entry for it. Once that entry is written, later changes to this field no longer reach that user.
- `optional` on `skills` (required): `skills` names extra directories scanned for `<name>/SKILL.md` skills. It adds to the default `skills/` scan rather than replacing it.
- `optional` on `commands` (required): `commands` names flat `.md` files or directories, or maps command names to entries. Setting it replaces the default `commands/` scan; list `./commands/` explicitly to keep it.
- `optional` on `agents` (required): `agents` names agent `.md` files. Setting it replaces the default `agents/` scan; list `./agents/` explicitly to keep it.
- `optional` on `hooks` (required): `hooks` is a hook config `.json` path, an inline hooks object, or a list of either. It merges with `hooks/hooks.json` rather than replacing it.
- `optional` on `mcpServers` (required): `mcpServers` is an MCP config path, an inline server map, or a list of them. It merges with `.mcp.json`, and a server name declared later replaces an earlier one.
- `optional` on `outputStyles` (required): `outputStyles` names output-style files or directories. Setting it replaces the default `output-styles/` scan; list `./output-styles/` explicitly to keep it.
- `optional` on `lspServers` (required): `lspServers` is an LSP config `.json` path, an inline server map, or a list of them. It merges with `.lsp.json`.
- `optional` on `experimental` (required): `experimental` is the container for components whose shape may still change — `themes` and `monitors` among them. Declare those under it, not at the top level.
- `optional` on `userConfig` (required): `userConfig` declares the values Claude Code prompts for when the plugin is enabled. Each option carries a `type`, `title` and `description`. A `sensitive` option's value goes to the secure credential store rather than to settings.
- `optional` on `channels` (required): `channels` lists the message channels the plugin provides. Each entry's `server` must be the key of one of the plugin's `mcpServers`.
- `optional` on `dependencies` (required): `dependencies` lists plugins that must be enabled for this one to work, each as `"name"`, `"name@marketplace"`, or a `{ name, marketplace, version }` object.
- `optional` on `themes` (required): `themes` is recognized here only so the `forbidden_keys` clause can name the move: theme files belong under `experimental.themes`.
- `optional` on `monitors` (required): `monitors` is recognized here only so the `forbidden_keys` clause can name the move: monitors belong under `experimental.monitors`.
- `optional` on `settings` (required): `settings` is a property of the published manifest schema, though the reference's field tables omit it. It is admitted here so a schema-valid manifest never draws a `closed-keys` finding.
- `closed-keys` (required): This key is not one the manifest format documents. Claude Code ignores an unrecognized top-level field, so the plugin still loads — that is deliberate, and it is what lets one `plugin.json` double as a VS Code or Cursor extension manifest, an npm `package.json`, or an MCPB/DXT bundle manifest. `claude plugin validate` reports it as a warning; `--strict` fails it, which is the CI bar this contract holds, so the usual cause is a typo or a field left over from another tool. If the key is deliberate foreign metadata, this clause is the one to drop from your adopted contract.
