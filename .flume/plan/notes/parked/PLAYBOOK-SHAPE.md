# Parked: the nested trigger still waits on RULE-CLAIMS-NESTED-CLAUDE-RULES

Re-gate this entry `blockedBy` RULE-CLAIMS-NESTED-CLAUDE-RULES. It still
carries gate `open`, so a tick dispatches here, finds the same blocker,
and lands nothing. That has now happened twice.

What stands: 5e0d4522 landed the playbook for the paths trigger and no
trigger, all three invocations, topics, renderers, and the execution and
preloading refusals. The continuing note lists what it covered, and it
stays in place for the tick that finishes the entry.

What blocks it: the nested trigger's rule has to land at
`<dir>/.claude/rules/<name>.md`. The rule kind's locus is still
`{ root: ".claude/rules", glob: "**/*.md" }` (`sdk/src/builtins.ts:463`).
A member's `location` is relative to that root (`emit.ts`
`placementThrough`), so emit cannot place the rule there, and check
would not claim the file if it did. Spec 0118 (d68c687f) now permits the
interior globstar that RULE-CLAIMS-NESTED-CLAUDE-RULES parked on. That
entry is buildable but not shipped. It touches `src/` glob grammar and
the built-in lock, so doing it here would take over another entry's scope.

Once that entry ships, the remainder is small. Add
`{ nested: "<dir>" }` to `PlaybookTrigger` (today a one-member union, so
callers do not break). Then place the skill with
`location: "<dir>/.claude/skills"` and the rule under the widened locus.
Identity is `a:b:name`, and `mentionOf` already renders `/a:b:name`.
Last, add the nested case to the probe harness that runs
`check --deny-advisories`.
