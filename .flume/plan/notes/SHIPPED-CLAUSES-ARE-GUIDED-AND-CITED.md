# Shipped clauses guided and cited: upstream drift seen

Done: 37 clauses gained guidance, 14 gained a dated cite. That is two more of each
than the census counted: `dial.type.clause[*].label` lacked guidance, and both
`requirement` clauses (shipped after the census) lacked cites. Temper-owned clauses
cite their corpus section via the repo's GitHub URL (`specs/...`, retrieved
2026-11-26). The SDK test now derives the contract set from every exported kind value
plus `rootDefaultContract`, recursing into `when` bodies. The Rust pins (`root_contract.rs`,
`builtin_lock_frozen.rs`) that held "only three root clauses cite" now hold "all six".

Drift seen while fetching plugins-reference on 2026-11-26. None of it is fixed here;
each item is a future entry:
- The manifest now documents top-level keys the `optional` allow-list lacks:
  `metadata`, `engines`, `types`, `workflows`, `network`, `icon`, `documentationUrl`,
  `supportUrl`, `privacyPolicyUrl`, `termsOfServiceUrl`. As a result, `closed-keys`
  false-fires on valid manifests, which breaks the decidable bar. `experimental` also
  holds `evals`.
- `hooks`, `mcpServers` and `lspServers` merge with their default files. `agents` takes
  files only, not directories. `mcpServers` also accepts a `.mcpb`/`.dxt` bundle path.
  The existing `type` guidance on these fields predates these facts.
- The `#component-path-fields` anchor may have moved; the page now has a "Component path
  forms" section.
- marketplace-reference documents `archive` (needs `url`) and `command` (needs `command`)
  plugin sources, and the `when` guards gate neither. git-subdir's `url` may also be
  `owner/repo`.
