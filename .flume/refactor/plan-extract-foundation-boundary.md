## Surface

`src/extract.rs` is declared **foundation** in `specs/process/architecture.md`'s
codemap, under the invariant "foundation depends on nothing internal —
holds today (`check`, `extract`, `tap`, `hash` import no sibling)." On disk
at HEAD this does not hold for `extract.rs`:

- `nested_members_from_rows` (764) and its private helper
  `embedded_member_from_row` (778) take `&[crate::drift::NestedMemberRow]` /
  `&crate::drift::NestedMemberRow` as real parameter types (not doc links) —
  `drift.rs` is the **pipeline** subsystem's core module, not a sibling
  foundation module. Sole caller: `src/builtin_kind.rs:578` (provider).
- `manifest_members` (1015) branches on
  `crate::kind::CollectionKeyPath::{HooksEvent,EnabledPlugins}` (1022, 1032),
  and `enablement_member_fields` (1075) / its inverse reference
  `crate::kind::ENABLEMENT_FIELD` (1077, 1095) — `kind.rs` is the **model**
  subsystem.

The module's own header (line 22-23) states a narrower, longstanding
boundary — "This module deliberately takes no dependency on
[`crate::contract`]" — which the code honors; the newer, broader
architecture.md claim ("nothing internal") does not match what the code has
always done here. Same shape as the two edges architecture.md already
declares in tension with its own map (`drift → install`,
`frontmatter → builtin_kind`): a real, verified edge the map's invariant
text does not admit.

Separately, noticed in the same read: `enablement_member_fields` (1075) and
`hook_member_fields` (1107) are `pub(crate)` with no caller outside
`extract.rs` itself (only used by `manifest_members` in the same file,
unlike their write-face siblings `enablement_entry_value`/`hook_matcher_group`
which are `pub(crate)` *and* called from `crate::drift`). Narrowing both to
private `fn` is a separate, purely mechanical fix
(`specs/process/engineering.md`, "An export earns its consumer") — filed
directly as a pending entry this tick, not part of this capture.

## Observed at

04610b1 (HEAD when observed).

## Suggested consolidation

Two directions, same shape as the map's own declared tensions — not a call
this capture makes:

- Move `nested_members_from_rows`/`embedded_member_from_row` into
  `drift.rs` (which already owns `NestedMemberRow`), returning
  `extract::EmbeddedMember` — pipeline depending on foundation, the
  invariant's intended direction; update `builtin_kind.rs`'s one call site.
  `manifest_members`'s two `CollectionKeyPath` branches and
  `enablement_member_fields`'s `ENABLEMENT_FIELD` read are the harder
  half: `crate::kind` is a `model`-subsystem module gating a much larger
  surface, so moving the *constant* down to `extract.rs` vs. moving the
  *function* up to `kind.rs`/`json_manifest.rs` both cost real ripple.
- Or amend architecture.md's invariant to admit `extract.rs`'s existing,
  narrower boundary ("no dependency on `crate::contract`") rather than the
  broader "nothing internal" it currently states — a third declared tension
  edge alongside the two the page already names.

Whichever plan drains this: it is a design call like the sibling edges, so
it likely wants `open-questions.md` (a new `(extract-foundation-edge)`
fork) rather than a pending entry that would have to invent the direction.
