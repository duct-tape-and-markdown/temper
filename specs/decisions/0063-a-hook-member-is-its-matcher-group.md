# 0063 — a hook member is its matcher group

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(hook-member-identity)` fork (GH #32). A hook's address was
`hook:<Event>`, but Claude Code nests three levels: an event, a matcher
group, and one or more handlers (code.claude.com/docs/en/hooks, retrieved
2026-09-29). This repo carries three `PostToolUse` groups, so `check`
reported `hook (6)` over four addresses, and an equality lookup answered
first-wins. The SDK synthesizes the collision itself: the tap maps two
telemetry events onto `PostToolUse` with different matchers and emits
both at `hook:PostToolUse`. The 09-03 refusal of duplicate event
addresses was built and failed the self-host gate, because distinct
matchers are distinct groups. 0049 makes resolution total. Ruled by John
2026-09-29 on the session's recommendation.

## Decision

**A hook member is one matcher group; its identity is the event plus the
matcher.** A matcher-less group keeps the bare event. The group carries
its handlers as its own array, so several commands on one (event,
matcher) are one member, not several. Two members with one identity are
a malformed lock (0049), refused loud: they are one Claude Code group.

## Rejected

- **An author-supplied name.** A fields-shape member has no side channel
  in `settings.json` to store it, and the SDK authors tap hooks, so a
  derived name has nothing to derive from.
- **Event-only identity with an edge-ambiguity refusal.** It keeps a
  first-wins lookup for top-level members that 0049 retired, and leaves
  unaddressed duplicates silent.
- **An escape scheme for a matcher spelling `/`.** The grammar has no
  escaping; a matcher the address cannot spell is refused loud rather
  than encoded by a rule invented here.

## Consequences

- The address grammar spells the matcher as the name's discriminator in
  both `member_address` homes, engine and SDK.
- The SDK `hook()` carries a handler list. Tap synthesis joins an
  authored group at the same (event, matcher) rather than minting a second.
- `graph::member_lookup`'s equality branch becomes total.
- Unblocks HOOK-COLLECTION-ADDRESS-DUPLICATE-REFUSAL and
  INSTALL-LIFTS-A-REGISTRATION-MEMBER.
