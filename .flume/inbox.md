<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Decision 0018's remaining scope, filed as real work, not "someday" — the
  ruling and its rationale are already ratified; nothing here needs a fresh
  cold read, only execution (observed at 29b4b17). Confirmed still
  unshipped, not incidentally covered by NESTED-MEMBER-LOCK-ROW/
  RETIRE-FOLD-MEMBERS: `EmbeddedMemberValue.collections`
  (`sdk/src/kind.ts:187-189`) and `NestedMemberRow.collections`
  (`src/drift.rs:1298`) are both still `BTreeMap`/`Record`-keyed, not an
  ordered list; no `render` hook exists anywhere in `sdk/src/kind.ts`;
  leaves are still plain `Record<string, string>` /
  `BTreeMap<String, String>` (`sdk/src/kind.ts:185`, `src/drift.rs:1295`),
  no `Text`/mention support.

  Three entries, independently green, per the ratified sequencing:

  1. **Ordered collections.** `EmbeddedMemberValue.collections` becomes an
     ordered list of `{key, leaves}` entries (SDK); `NestedMemberRow`'s
     collections follow the same shape, TOML `[[array-of-tables]]`
     preserving declaration order the way the lock already does for every
     other array-shaped family — no new mechanism, just the type change and
     its read-side fold. Retires the testbed's `1-classify`/
     `2-validate-currency` key-prefix ordering hack as a side effect.
  2. **The `render` hook.** A custom kind gains an optional SDK-side render
     function (`(value) => string`), erased at the seam — the engine never
     sees or parses its output, so no admissibility bar applies (0018's own
     ruling: an embedded kind's format is "writer-only and unconstrained").
     Absent `render`, the default view stays today's TOML fence — zero
     forced migration, byte-identical projection until a kind author opts
     in.
  3. **`Text`-valued leaves.** A leaf value may be a template + mentions,
     not just a bare string. Its mentions lift into the host's mention set
     as ordinary `[[declaration.mention]]` rows (the edge survives); its
     resolved display becomes the stored leaf string. Depends on nothing
     from 1/2; smallest of the three, do last per the original sequencing.

  Per `specs/model/representation.md`/`pipeline.md`'s current text and
  decision `0018-the-projection-is-not-the-database.md` — cite that
  decision's Consequences section directly; no new spec section needed,
  this is inside its already-ratified scope.

