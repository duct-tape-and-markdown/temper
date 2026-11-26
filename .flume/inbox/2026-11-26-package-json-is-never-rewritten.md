# Install never rewrites the program's `package.json`; its `packageManager` names the manager

`specs/model/authoring.md` (0100): the program's `package.json` is a source temper scaffolds once and never writes, and its `packageManager` field is the declared manager. The code meets neither (census, 2026-11-26):
- `ensure_package_json` runs on every represented install (`src/install.rs:908-910`) and rewrites the file through `serde_json::to_string_pretty` without `preserve_order`, which sorts keys and drops the author's formatting;
- nothing reads `packageManager`: `spawn_npm_install` always runs `npm` (`:3445`), and the tool kind picks a manager by which lockfile exists (`src/tool.rs`).

Install writes `package.json` only when the file is absent, and the manager comes from `packageManager`, refusing loudly where it is needed and absent. Acceptance:
- **new:** re-running install leaves an existing `package.json` byte-identical;
- **new:** a `packageManager: "pnpm@…"` makes install and the tool use pnpm;
- **kept:** a first install scaffolds the file.
