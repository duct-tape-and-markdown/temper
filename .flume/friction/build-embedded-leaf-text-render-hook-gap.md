## Symptom

EMBEDDED-LEAF-TEXT (this tick) widened `EmbeddedMemberValue.leaves` (and a
collection entry's leaves) to accept a `Text` template alongside a bare
string. The default view (`emit.ts`'s `renderMemberToml`) resolves a `Text`
leaf's mentions — resolution-checked, loud on a dangling address — before
TOML-quoting it, and the same resolved string lands in the `nested_member`
declaration row. But `EMBEDDED-KIND-RENDER-HOOK` (3c6f50b/9bf90bc, shipped
just before this entry) gave a kind's own `render(value)` hook the *raw*
`EmbeddedMemberValue` — `value.leaves` un-resolved — and nothing in this
entry's scope touches that path (its `entry.files` named only
`renderMemberToml`'s leaf loop, ~72-81).

Net effect: a host kind **without** a `render` hook refuses loudly on a
dangling leaf mention; the identical leaf on a kind **with** a `render` hook
silently stringifies the raw `Text` object (`[object Object]`-shaped) with no
resolution check at all — the refusal behavior depends on an unrelated
authoring choice (whether the kind declares `render`). `resolveLeaf`
(`prose.ts`) is exported but the hook signature gives no way to reach it
without also owning the leaf-walk.

## Cost this tick

No cost paid renders this tick — the assigned entry stayed in its cited
scope, all gates are green, and no existing test exercises `render` + `Text`
leaves together. Flagging so it doesn't read as intentional. Cost is
prospective: the next kind author who declares a `render` hook over a
`Text`-leaf-bearing kind gets no mention-dangling refusal and a broken
render, discovered only by eyeballing output.

## Suggested fix

A `.flume/inbox.md` item scoping a follow-up: either (a) resolve every leaf
(mention-checked) before calling `value.render(value)`, so a hook always sees
plain strings and the refusal bar is uniform regardless of `render` — the
simpler contract, but forecloses a hook that wants the raw template — or (b)
document that `render` hooks own their own leaf resolution and export a
per-leaf helper (`resolveLeaf` is already exported from `prose.ts`, just not
re-exported through `index.ts`) plus a test proving the dangling-mention
refusal fires identically either way. (a) matches "the one display rule"
framing `contract.md`'s edge section uses elsewhere and is my recommendation.
