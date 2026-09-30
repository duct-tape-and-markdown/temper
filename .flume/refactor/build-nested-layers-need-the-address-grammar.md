# NESTED-LAYERS-COMPOSE-TO-ANY-DEPTH needs the address grammar widened

## Surface

The entry's fence is the path composition (`src/drift.rs`, `src/import.rs`,
`src/compose.rs`, `sdk/src/emit.ts`) plus tests. But a second layer changes the
**address** a child wears — `area:<a>/page/<b>/<kind>/<key>`, five segments — and
both faces' address grammar is capped at three by construction, in files the
fence does not carry:

- `src/member_address.rs:189` — `segment` is `splitn(4, '/')` and
  `parse_nested_address:219` refuses any tail, so a five-segment address reads as
  a **leaf** address, never a member one. Every deeper reader is downstream of
  this: `address_of:72` re-qualifies a five-segment id as
  `leaf:area:a/page/b/leaf/k`, so `src/gate.rs:119` (every finding's member
  address), `src/builtin_kind.rs:691` (a child's own nested-member rows),
  `src/admissibility.rs:220` and `src/graph.rs:2363` all spell an address no
  member wears. They self-correct once the reader reads any depth — they are the
  blast radius to verify, not necessarily separate edits.
- `sdk/src/member-address.ts:101` — `memberAddress` spells
  `nestedAddress(hostAddress(host.kind, host.name), …)`, flattening a nested host
  to `kind:name`: a two-layer child is keyed `page:b/leaf/k`, the grandparent
  lost. Its doc comment states the very invariant this entry removes ("The host
  segment is a **host** address, never a nested one … one level is the whole
  grammar here"), so the SDK's declaration rows disagree with the engine's
  composition the moment depth 2 emits.
- `src/read.rs:240-266` — `species` gates member-identity equality behind
  `nested_key(target).is_some()` and then falls through to `parse_leaf_address`,
  so `entry.tests[read_verbs.rs]` cannot go green. Verified at HEAD against a
  live one-layer corpus: `temper explain skill:alpha/supporting-doc/home`
  resolves the member, while `…/home/leaf/k` answers "No leaf … is in the
  surface's serialized nested-member leaves" with no corpus consulted.

Unbounded depth needs the same reader: `nested_file_path` must find its host's
kind *and its host's own host*, which `parse_nested_address` supplies at depth 1
and refuses at depth 2 and beyond.

## Open question the entry rests on

At depth ≥ 2 a member address and a leaf address are the **same string**:
`area:a/page/b/leaf/k` is both the two-layer child and a `leaf/k` child path under
`area:a`'s `page`/`b`. `representation.md` ("member") demands resolution be total,
and the cite does not settle which grain wins. Segment **parity** is the available
rule — a member address is odd (1, 3, 5 …), a leaf address even, because
`extract.rs:262`'s `addressed_leaves` joins a child path with `.` and never `/` —
but choosing it is a grammar decision, not a transcription, and it belongs beside
the two parsers rather than inside `drift`.

Secondary, bearing on the entry's stated projection `<root>/<a>/<b>/<dir>/<key>.<ext>`:
a template pattern with a directory segment folds the child's id through
`frontmatter::fold_file_id:384` as `<dir>-<key>`, which `splice_name` then places
back at `<dir>/<dir>-<key>.<ext>` — the round trip `refuse_ungoverned` guards
already fails one layer down, so the two-layer corpus's leaf pattern must sit at
the unit root (`*.md`) until the fold and the splice agree.

## Observed at

d2be6289 (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation

Re-scope the entry with `src/member_address.rs`, `sdk/src/member-address.ts` and
`src/read.rs` in the fence (and `src/gate.rs`, `src/builtin_kind.rs`,
`src/admissibility.rs`, `src/graph.rs` as the `address_of` blast radius), and key
the parity rule as an open question against `specs/model/representation.md`
("member") first — one reader per face, widened to any depth, with the
member-vs-leaf discrimination stated where both parsers already live.
