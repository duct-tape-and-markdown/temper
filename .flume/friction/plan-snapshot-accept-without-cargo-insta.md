## Symptom

`cargo-insta` is not installed in the build environment — `cargo insta`
errors "no such command: `insta`". Nothing in either harness says so, and
nothing says how to accept an `insta` snapshot without it:
`.claude/rules/rust.md` ("Tests") tells an agent to *prefer* insta snapshots
and `CLAUDE.md` lists the crate, but neither names an accept path.

So an entry whose change legitimately moves a `.snap` fixture has no
runnable instruction. `LAYOUT-PROSE-REGION-UNREACHABLE-LOCK-EXPLAIN`'s
attempt reverted at the `cargo test` gate on 2026-09-07 with exactly one
red test — the gauntlet projection-tree snapshot — carrying precisely the
diff the entry predicted; the fence was already correct, the entry's own
`cargo insta review` instruction simply could not be run. Build's fence
capture for `EMIT-PROJECTION-NOTE-DEFERRED-TO-INSTALL` (`1e663552`) reached
for `cargo insta accept` the same way.

The working path exists and needs no CLI: the `insta` crate reads
`INSTA_UPDATE` itself (`insta-1.48.0/src/env.rs:246-268`, values
`auto|always|new|unseen|no|force`), so `INSTA_UPDATE=always cargo test
--test <name>` rewrites the `.snap` in place.

## Cost this tick

One reverted build invocation on a change that was otherwise complete, plus
this plan tick's re-scope of two entries to spell the accept command
inline. It will recur on every future snapshot-moving entry until the
knowledge lives somewhere an agent reads by default.

## Suggested fix

One line under `.claude/rules/rust.md`'s "Tests" bullet on insta
snapshots — "accept a moved snapshot with `INSTA_UPDATE=always cargo test
--test <name>`, then re-run plain `cargo test`; `cargo-insta` is not
installed" — so plan stops having to spell it per entry. Installing
`cargo-insta` on the build host would work too, but the env var needs no
toolchain change and no network.
