# An embedded subkind is chosen by its host

Shipped. `extend` accepts an embedded base (refusing `governs`/`exclude`, as for nested-file),
and `template:` with an embedded child swaps it for the base in the host's template layer
and in each layout collection whose `memberKind` is the base. If only a collection names
the base, the extended host also gains a path-less `{kind: child}` layer, because a
collection's `memberKind` is a string and the layer is what puts the subkind in play.
`kindFactKindsInPlay` gives a row to any embedded kind carrying a `base`.

Engine: `resolve_subkinds` binds a subkind whose base has no row, provided some kind
hosts that base as an embedded child (`hosted_embedded_kinds`). Without this, the common
case (an embedded base declaring no guidance, cite or leaves) refused at load. The gate's
embedded dispatcher passed no inherited clauses, so a base's clause never reached a
subkind's member. It now passes `inherited_clauses` (src/gate.rs), the same as the
custom-row loop.

No extract.rs change was needed. The host's row carries its own swapped `content`, so
the layout read keys members as the subkind.

Gaps for plan:
- An embedded subkind can add edge fields and counsel but no leaf. `placedBody` admits no
  field placement on an embedded base, and the delta has no `leaves`, so the base's leaf
  witness is inherited unchanged. Typing Cascade's invariant with new fields needs a decision.
- `blocks()` values are still gated on `admit` alone (`nestedMemberRows`). A host's own
  declared embedded template doesn't count, so an extended composed-body host must also
  `admit` its subkind.
