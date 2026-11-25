<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `marketplace` kind

a relative-path `source` resolves against a *local copy* of the marketplace, so it silently fails to resolve for users who added the marketplace by direct URL to `marketplace.json` — only that one file is downloaded. Reach for `github`, `url`, or `npm` when the catalog is distributed by URL. Where a git source pins both `ref` and `sha`, the `sha` is the effective pin. A marketplace entry's `defaultEnabled` beats the same field in the plugin's own `plugin.json`, while `version` runs the other way — `plugin.json` wins.

## Locus

Each member is its own document, at:

- `.claude-plugin/marketplace.json`

## Clauses

- `required` on `name` (required): A marketplace declares a `name` — the identity users type when installing from it (`/plugin install my-tool@your-marketplace`) and the key each user registers it under. There is no directory fallback the way `plugin.json` has one.
- `min_len` on `name` (required): A present-but-empty name cannot key an install or qualify a plugin reference. Give it a real one.
- `allowed_chars` on `name` (required): Kebab-case, no spaces — lowercase letters, digits, and hyphens. The name is typed after an `@` to qualify every plugin installed from the marketplace, so a space or a capital makes it unquotable exactly where users type it.
- `deny` on `name` (required): This name is reserved for official Anthropic use and a third-party marketplace cannot load under it. Claude Code re-checks the reserved list on every load, not only when the marketplace is added — so publishing under one of these names strands every user who already added you: the marketplace stops loading and reports that it is registered from an untrusted source, and they must remove and re-add it. The list also grows (before v2.1.205, `first-party-plugins` and `healthcare` were not on it). Names that merely *impersonate* an official marketplace — `official-claude-plugins`, `anthropic-plugins-v2` — are blocked too, but impersonation is a judgment no clause can decide: this clause holds the enumerated list, and steering clear of the lookalikes is yours.
- `required` on `owner` (required): A marketplace names its maintainer. This clause decides the `owner` object's presence; the clause below decides that its `name` is filled.
- `required` on `owner.name` (required): `owner.name` is the maintainer or team behind the catalog, and it is required — an `owner` object carrying only `email` does not satisfy the schema. `owner.email` beside it is optional.
- `required` on `plugins` (required): The `plugins` array is the catalog — a marketplace without it lists nothing. An empty array is a valid, if empty, catalog.
- `required` on `plugins[*].name` (required): Every catalog entry declares a `name` — the plugin identifier users type when installing (`/plugin install my-plugin@marketplace`), kebab-case and without spaces. An entry with no name cannot be installed, and this clause names the entry that omitted it by its own index.
- `required` on `plugins[*].source` (required): Every catalog entry declares a `source` — where the plugin is fetched from. A listed plugin with no source resolves to nothing. Note that a relative-path source resolves against a *local copy* of the marketplace, so it fails to resolve for users who added the marketplace by direct URL to `marketplace.json`.
- `when` (required)
- `when` (required)
- `when` (required)
- `when` (required)
- `when` (required)
