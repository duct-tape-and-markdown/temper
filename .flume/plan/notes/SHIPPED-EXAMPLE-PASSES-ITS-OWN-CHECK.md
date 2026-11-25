# Shipped example passes its own check

The inbox's four `root.locus-declared` hits on CLAUDE.md and the two rules
did not reproduce on this tree's engine: emit reported every projection
unchanged and the lock byte-current. The real findings from the example's
root were `install.gate-installed` (hook() with command strings, and no
PostToolUse Bash gate at all), `.claude/settings.json` with no settings
member, and the three `src/*.js` files at the `source` locus with no member.
Fixes: gateHook for all three gates; `settings({ name: "settings" })`
declared so settings.json is a projection; `source` declared
`commitment: "local"`, the remedy the finding itself names. `src/` is
committed, though, and `local` is documented as "per-machine and
uncommitted". It works, but read-only committed ground has no posture of
its own. That is a possible spec gap for a human to weigh. A
committed-but-not-projected locus may want its own spelling.

The SDK now refuses a non-constructor `prose` at member construction
(`memberProse`, prose.ts), naming member, kind and field, where emit used
to crash with `TypeError: mentions is not iterable`.
