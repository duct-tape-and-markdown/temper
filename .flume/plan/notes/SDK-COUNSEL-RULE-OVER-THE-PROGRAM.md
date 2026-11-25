# SDK counsel rule ships as `temper-sdk` over `.temper/**`

Shipped: `sdk/src/counsel.ts` (`SDK_COUNSEL`, ~3.1 KB) and a `temper-sdk` rule that
`guidanceRules` synthesizes on every emit, before the per-kind
`temper-guidance-<kind>` rules. The two share one collision refusal. The base-harness
example is re-emitted.

Consequences plan should know:
- The built-in `rule` kind is now in play in every program. So a `rule:<name>` mention
  that resolves nowhere defers to `check`; emit no longer refuses it, even in a program
  that authors no rule. Three dangling-mention tests moved from `rule:ghost` to
  `agent:ghost` so they keep testing the emit refusal. If emit should still refuse there,
  the deferral set must exclude kinds only synthesis put in play. That is a decision,
  not a build call.
- A "memberless" emit now writes one provenance row, the counsel rule's.
  `builtin_lock_frozen` asserts that prefix exactly, bar the hashes. The gauntlet
  snapshots were re-blessed: one more rule member and one more projection.
- The repo's own harness (`.claude/rules/temper-sdk.md`) owes a `chore(harness)`
  re-emit. Build cannot write `.claude/`.
- SDK-TSDOC-RULES-MOVE-TO-DELIVERED-HOMES can now unblock. Its additions to
  `SDK_COUNSEL` are bounded by the test "the SDK counsel stays within about 1k tokens"
  (≤4096 bytes), which leaves about 950 bytes of headroom.
