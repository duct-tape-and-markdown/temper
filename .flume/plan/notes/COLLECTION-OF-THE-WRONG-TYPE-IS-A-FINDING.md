# Rejected manifest collection: what shipped, what stays open

Shipped: `Manifest::parse` no longer consumes a declared collection key whose value is not an object. It records the key and its JSON type in `Manifest::rejected_collections` and leaves the value in `opaque_fields`. The gate's `manifest_units` raises one `manifest.rejected-collection` error per key, for the kind that owns that collection. The type name comes from `schema::json_type`, now `pub(crate)`, so no second spelling of JSON type names exists.

Choices plan should check:
- Severity is error for both cases. The MCP file is rejected whole; settings skips only the value. Both mean none of the collection's registrations load. If settings should be a warning, the finding needs per-file severity.
- `drift.rs`: an emit reap no longer reports a residue key that a rendered segment replaces. A program declaring servers over a disk `mcpServers: []` overwrites the key and nothing more; it is not an ownerless-keys refusal.

Not covered (debt):
- The PreToolUse guard (`install.rs`, pending-manifest check) parses through `Manifest::parse` but never reads `rejected_collections`. A pending write of `"mcpServers": []` passes the guard and is only caught by the next `check`.
- `ManifestCache`'s second tuple element is always inserted as an empty map and never read.
