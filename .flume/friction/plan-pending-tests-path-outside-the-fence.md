## Symptom

`TELEMETRY-EVENT-HOOK-TABLE-CROSSES-THE-GENERATED-SEAM` declared three
`tests[]` entries; the third named `sdk/test/builtins.test.ts`, which the
entry's `files[]` did not carry. The fence is a write allowance, so that
assertion was unbuildable by construction — build shipped the two Rust-side
pins, cut the third, and said so (6891a624's body). Nothing in the harness
refuses the shape: `.flume/chain.ts`'s `entryExtension.tests` validates
`{path, asserts}` are non-empty strings and the pending gate checks every
cited path *resolves on disk* — never that a `tests[].path` is inside the
entry's own `files[]`. Plan derived it, the gate accepted it, and the
contradiction only surfaced when build hit the fence.

## Cost this tick

One cut corner shipped (the entry's third test), one refactor capture filed
(`build-tap-hooks-ts-pin-needs-the-sdk-test-fence.md`), and one whole plan
tick spent draining that capture (bfc75016) — which found an equivalent pin
already running at `sdk/test/emit.test.ts:2218`, so the capture resolved to
no work at all. Net: a build tick's cut plus a plan tick, both on a
contradiction visible in the entry's own JSON at filing time.

## Suggested fix

A parse-gate assertion in `.flume/chain.ts`: every `tests[].path` must
appear in `files.new`/`files.edit`/`files.retire`. Cost is nil when the test
needs no edit — naming an unmodified file in the fence is legal — and the
check refuses the one shape build can never satisfy. A rule line in
`.claude/rules/pending-entry.md` is the weaker alternative (prose restating
what a gate could hold, per `specs/process/engineering.md`, "The fix lands
at the mechanism"); the chain's gates are out of the amendments channel's
scope, so this comes here rather than as an apply-ready diff.

Second, smaller lesson from the same entry, for whoever drains this: the pin
plan invented already existed under a different file. A `tests[]` entry
naming new coverage is worth one `rg` through the sibling suite first.
