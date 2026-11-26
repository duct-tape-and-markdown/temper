# How does a member carry its placement as a path fact?

`specs/model/representation.md`, "member" (0109): "a name derived from a field is opaque, and a member's placement is its own path fact, never parsed out of its identity." The code places every member by parsing its name: `src/drift.rs` `placement_in_unit` turns each `:` back into `/`, `placing_template` picks a template off the name's leaf, and `sdk/src/emit.ts` (`placementInUnit`, `placingTemplate`) and `sdk/src/kind.ts` `identityFieldValue` mirror it. The read side builds a nested output style's identity as `<span>:<field>`.

The corpus states that the fact exists. It does not say how it is spelled, and these answers all fit it:

1. **The authoring surface.** A member init could take a path (harness-relative, or relative to the kind's locus root). It could be required everywhere, or optional with a default that splices a one-segment name whole into a starred template. A starless `**/` template such as `**/CLAUDE.md` would then require it.
2. **Identity.** A field-derived identity (output style) could drop its `<span>:` prefix and become the bare field value, so two same-named styles in different directories would be coincident addresses. Or it could keep the span as a path-derived prefix.
3. **The lock.** The path could be a member row column, or only the target row's existing `path`.

Lean: an optional `at` on the member init, relative to the kind's locus root, and required where no starred template can take the name whole. A field-derived identity is the field value alone. The lock carries the path on the member row, and import fills it from the read path. Both `placing_template` copies are retired.

Needed: a ruling in `representation.md` ("member" or "locus") and in `authoring.md`. Plan files no entry before then.
