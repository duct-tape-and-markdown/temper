## Surface

`HOOK-NAME-JOINS-ITS-MATCHER` is mis-scoped by exactly one file. The entry reasoned that
"any arm asserting the member's address must now read the joined form where a matcher is
present" (that is why it declared `tests/it/settings_kind.rs`), but it missed the one arm
that does so outside the fence:

- `tests/it/read_verbs.rs:1612` — `common::explain_in(&harness, "member:hook:PreToolUse/handler/0")`
- `tests/it/read_verbs.rs:1638` — `out.contains("`PreToolUse` (hook) contains it")`

`explain_narrates_a_composed_handler_member_the_gate_judged` drives
`ONE_HANDLER_SETTINGS` (`read_verbs.rs:1576`), whose one `PreToolUse` group binds
`"matcher": "Bash"`. Under 0074 that group's member is `hook:PreToolUse:Bash`, so its
composed handler is `hook:PreToolUse:Bash/handler/0` and the two literals above become
`member:hook:PreToolUse:Bash/handler/0` and `` `PreToolUse:Bash` (hook) contains it ``.

This is not avoidable within the entry. `compose.rs`'s `manifest_units` spells the
embedded host address off the host unit's id, and it must: if the handler's host segment
stayed the bare collection key, it would name no member at all where the group binds a
matcher, and two groups on one event would put their handlers back at one address —
the collision 0074 exists to break.

## Observed at

bbfe5861 (HEAD when observed).

## Suggested consolidation

Re-cut the entry with `tests/it/read_verbs.rs` added to `files.edit` ("the one composed
handler arm outside `hook_kind.rs` that spells its host's address; its fixture's group
binds `Bash`, so both literals take the joined form"). Everything else in the entry holds
as derived and was verified green this tick before the capture was filed:

- `src/member_address.rs` — grammar unchanged; gains the doc account of a `:`-carrying
  name and `is_name_qualifier` (the `/`-refusal predicate, beside `is_one_segment`; it
  admits the empty qualifier, which `is_one_segment` refuses, so the two are not one).
- `src/json_manifest.rs` — `RegistrationMember::name` joins the collection key with the
  group's lifted values (generic over `EntryShape::GroupArray`, so `matcher` is named
  nowhere here), `to_unit`'s id reads it, and `Manifest::parse` refuses an unjoinable
  value (`JsonManifestError::UnjoinableName`). The refusal sits at the read because
  `to_unit` must stay infallible — `tests/it/common/mod.rs:259` calls it and is itself
  out of fence.
- `src/compose.rs`, `src/install.rs`, `sdk/src/emit.ts` — verified, no behavior moved.
  `hook_claimed_events` already reads `registration.key`; `module_stems` already keys
  `GATE_HOOKS` on `hook.event`; `registrationFacts` maps `row.key` through unchanged.
  (The `install.rs` half of the entry is docs only: the SDK's `hook()` composes the name,
  so the module's authored `name` property stays the event and the stem with it.)
- `sdk/src/builtins.ts` — `hookMemberName`/`hookCollectionKey` beside `HOOK_MATCHER_KEY`,
  `hook` wrapped to compose; `sdk/src/declarations.ts` — `registrationRows` keys the
  collection.
- `examples/base-harness/.temper/lock.toml` — regenerated: one row, the `satisfies`
  member `hook:PreToolUse` → `hook:PreToolUse:Write|Edit|MultiEdit`. Its registration
  rows and the gauntlet's two snapshots did not churn at all (the corpus's hooks reach no
  finding and no lock row by name), so those three fence entries can be dropped from the
  re-cut.
- Tests written and green: the three identity arms in `tests/it/hook_kind.rs`, the
  `:`-bearing-name arm in `sdk/test/member-address.test.ts`, and two arms in
  `sdk/test/emit.test.ts` (two distinctly-named members both keyed `PostToolUse`; the
  `/`-matcher refusal at the call). `cargo fmt`, `cargo clippy -D warnings`, `cargo doc`
  and `pnpm --dir sdk test` (224 pass) were all green; `cargo test` was 813 pass / 1 fail,
  the one fail being the arm above.

One incidental finding worth a line in the re-cut, not a change: two registration rows on
one event now tie on `registrationRows`' `(kind, key)` sort, and the tie is resolved by
`Array.prototype.sort`'s stability — so the groups stay in authored order, which is the
order Claude Code loads the event's array in. The member names are now distinct and could
totally order the rows, but sorting on them would reorder groups within an event and move
`settings.json` bytes for no gain.
