# Load moment: 0117's stale reading misses a same-stamp lock

Re-applied the prior shape (the read.rs conflict with the runtime-match strand
resolves by keeping both sides). The SDK, the explain tests and the build all
pass. `cargo test` still fails on
`cli::self_host_check_is_clean_over_tempers_own_surface`: `check` raises
required `kind.admissibility` for `memory` and `skill`, saying they
"collide with an embedded built-in".

Why 0117 did not unblock it: `retire_stale_builtin_rows` (src/compose.rs)
retires a stale built-in row only under *another* stamp. Every checkout build
stamps `0.0.25+unreleased`, and since c1a347cc the dogfood lock does too. So
the lock from the old engine and this new engine share one stamp, and the
collision stands. Every unreleased commit that moves a built-in fact hits the
same wall. `.temper/lock.toml` is outside build's fence, and the example locks
re-emit fine.

Decision needed (one of):
1. A stamp that tells unreleased builds apart, for example by hashing the
   embedded built-in rows into it. Committing the hash is not the answer:
   build.rs deliberately keeps committed locks stable when no source changed.
   A built-in-fact digest moves exactly when the snapshot goes stale.
2. Retire a divergent built-in-named row under any stamp, when it equals no
   admissible relocation. This gives up the authored-collision finding.
3. A human lands this entry together with the dogfood re-emit in one
   `chore(harness):` + `build:` pair. That is the one-off unblock, and 0117
   rejected it as the general fix.

Resolved diff: /tmp/load-moment-is-a-registration-fact.patch (git apply -3;
then re-emit examples/base-harness lock).
