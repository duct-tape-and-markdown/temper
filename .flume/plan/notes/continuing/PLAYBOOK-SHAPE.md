# PLAYBOOK-SHAPE: the nested trigger waits on nested rules

Landed: `playbook()` in `sdk/src/playbook.ts`, exported from the patterns subpath and
composed by `defaultPatterns({ playbooks })`. It covers the paths trigger (skill listed
by default, `listing: "on-touch"` sets the skill's `paths`), no trigger (rule at launch),
all three invocations (user-only: the rule asks the user to run `/name`, the skill keeps
its description), topics as supporting docs linked by path mentions, and replaceable
`router`/`pointer` renderers. `execution` and `preloadedBy` are typed and throw, naming
the axis and the issues URL. A probe harness covering paths, on-touch, none and every
invocation emits and passes `temper check . --deny-advisories` (exit 0, no findings).

Not landed: the nested trigger. A nested area's surfaces are
`<dir>/.claude/skills/<name>/` and `<dir>/.claude/rules/<name>.md`. The skill already
places there (`location`). The rule can't: its locus root is `.claude/rules`, and
`location` is relative to that root (`emit.ts` `placement`). RULE-CLAIMS-NESTED-CLAUDE-RULES
is pending and widens the locus. This entry has no dependency on it. Re-gate the remainder
`blockedBy` it. Placing the rule at the root with `paths: ["<dir>/**"]` would invent a
placement the section doesn't name, so I didn't. `PlaybookTrigger` is a one-member union
today, so `{ nested: "<dir>" }` can be added as a member without breaking callers. The
skill takes `name: "<a>:<b>:<name>"` plus `location`, and `mentionOf` already renders
`/a/b:name`.

Consolidation: reference.ts's `Linked`/`linked`/`joined`/`prose` moved to `prose.ts`
(`prose` renamed `linkedText`) so the reference and the playbook render links one way.
