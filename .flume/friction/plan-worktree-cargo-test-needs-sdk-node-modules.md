## Symptom

`cargo test --test it` in a fresh phase worktree reports `734 passed; 48
failed` — and every failure is an artifact of `sdk/node_modules` not existing,
not of the code under test. The 48 are the SDK-round-trip family
(`type_predicate::an_sdk_authored_*`, `settings_kind::*`,
`read_verbs::explain_narrates_a_custom_embedded_kinds_guidance_after_a_full_sdk_round_trip`,
`projection_path_seam::*`, …): `common::ensure_sdk_built` shells `npm run
build` in `sdk/`, which cannot run `tsc` with no `node_modules`, so `sdk/dist`
stays stale and the seam tests fail. Nothing in the output names the cause —
no `ENOENT`, no npm error surfaced through the test harness, just 48 reds.

The gates never see this: `.flume/chain.ts` runs `cargo test` and `pnpm --dir
sdk test` **afterMerge, on the trunk**, and says so (`sdkGate`'s doc: "where
sdk/node_modules exists"). The `setupWorktree` comment (`chain.ts:612`)
reasons only about cargo's own deps — "cargo resolves deps from the global
registry cache under `~/.cargo`, shared across worktrees for free; only
`target/` is per-worktree" — which is true and incomplete: 48 tests also need
a pnpm tree that is *not* shared across worktrees. So the one actor who hits
this is an agent verifying inside its phase worktree, which is exactly what
plan's audit motion asks for ("verify on disk, never the log alone").

`CLAUDE.md` carries the install line, but frames it as an **emit**
precondition ("Fresh clone, before the first emit"), not a `cargo test` one;
`.claude/rules/sdk.md` is `paths:`-scoped to `sdk/`, so it does not load for a
plan tick that touches no TS. The failure signature is the expensive part: 48
reds in the exact window a plan tick is auditing read as a regression the ship
introduced, and a tick that believed them would file entries against
phantom breakage.

## Cost this tick

One wasted full test run, plus ~5 tool calls to diagnose (grep for panics —
none; inspect the pre-existing `build-sdk-dist-race-across-test-binaries.md`
capture, which is an adjacent but different failure; `ls sdk/node_modules`).
Then `pnpm -C sdk install && pnpm -C sdk build` (~450ms + tsc) and a re-run:
`782 passed; 0 failed`. No commit reverted — the tick's verdict was
unaffected, because the diagnosis landed before the audit concluded. The
next tick that skips the diagnosis is the one that pays for real.

Re-measured at 9ec9d1d1: the class is unchanged but the signature grew with
the suite — `785 passed; 63 failed`, then `848 passed; 0 failed` after the
install. So the count above is a snapshot, not the signature to match on; the
signature is "every red is an SDK-seam test and no output names a cause".

## Suggested fix

One line of operational knowledge somewhere an in-worktree agent will read it
before running the suite — `.flume/PROTOCOL.md`'s command list is the natural
home, since it is the one place that is both phase-facing and unscoped:
`cargo test` in a fresh worktree needs `pnpm -C sdk install && pnpm -C sdk
build` first, or 48 SDK-seam tests fail for that reason alone.

The mechanical alternative is a `setupWorktree` hook in `.flume/chain.ts` that
runs `pnpm --dir sdk install` per worktree, which would also retire the
adjacent `build-sdk-dist-race-across-test-binaries.md` race if it built
`sdk/dist` once up front rather than leaving it to `ensure_sdk_built` racing
across test binaries. That is the drainer's call — it trades a per-worktree
install for a class of confusing failures.
