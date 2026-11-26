# SDK package ships the built-in schemas

Shipped `sdk/schemas/{agent,command,output-style,rule,skill}.json` — the predicted kind
list matched the engine's frontmatter floor domain exactly. The drift test
(`tests/it/sdk_schemas_shipped.rs`) derives the kind list from `temper schema`'s own map
keys in a lockless dir, not a hard-coded list, and byte-compares each file against
`temper schema --kind <k>` stdout (trailing newline included). A floor change
regenerates with `BLESS_SDK_SCHEMAS=1 cargo test --test it sdk_schemas_shipped`
(drops orphans too). Modeled on the seam-bindings bless path rather than
`builtin_lock_frozen.rs`, which has no update path.

`output-style.json` is the vacuous `{ $schema, type: object }` — the floor carries no
mappable top-level clause for it. Not a defect, but the install modeline entry that
points at these URLs should know some kinds ship an assert-nothing schema.
