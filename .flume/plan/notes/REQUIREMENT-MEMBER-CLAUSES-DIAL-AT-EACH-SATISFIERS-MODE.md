# Member-grain clauses and fresh findings dial at each member's own mode

Shipped. Where the work landed versus the entry's file prediction:

- `src/drift.rs` needed no edit. The `input` rows already carry the declaring
  member's `kind:name` address, and `module` rows are owned by `root`, so the
  root's mode is correct for them. The source-dependency gap was really in
  `check` (`gate.rs`), where every `fresh` finding read the root-dialed clause.
  It now re-reads each `MemberFinding` at its own member's mode. Projection
  drift (`config_stale_members_from_doc`) goes through the same door.
- `EnforcementModes::for_member` missed root-hosted addresses
  (`root/tool/<key>`): a tool-file input with no mode of its own fell to the
  root instead of the `tool` kind's mode. `member_address::root_hosted_kind`
  closes that gap.
- `engine::judge_members` gained a `&[DivergingMembers]` argument. Peers stay
  the whole selection, so `unique-name` still ranges over every satisfier.
  Kind selections' peer clauses use the same split (same mechanism, same bug).
- Retired the unused `gate::source_dependency_stale`. Its doc claimed the guard
  called it, but the guard calls the drift function directly.

Observed: the dial's mode test is binary (only `block` versus not-`block`
matters), so a selection splits into at most two clause sets. A future mode
whose dial semantics differ from `warn`/`note` would break that assumption in
both `two_greens_dispatch` and the gate's selection loop.
