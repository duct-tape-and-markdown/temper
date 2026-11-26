# MCP server absent type reads as stdio

Shipped. The default sits on `CustomKind::field_defaults`, a row-less built-in fact
lifted in `builtin_kind.rs` (`declared_field_defaults`, next to `bare_root_file`). There
is no lock column and no SDK `kind()` option, so authors get no new default surface.
`builtins.ts` cites it and changes only prose. The lock diff is the one guidance row.

The engine reads it in one place: a `when` guard locates the default where the member
is silent (`engine::validate_as_read`). The body's element stays the authored bytes.
`validate` keeps its signature and passes no defaults. `judge_members` gained a
kind-to-defaults map, so a requirement's guard over a satisfier reads it the same way.
Subkinds inherit the base's defaults (`compose::resolve_subkinds`).

The http+url line now lands as a pin; this tick re-ships the reverted 412b6b0a unchanged
but for this note.
