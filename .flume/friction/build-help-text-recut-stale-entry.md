Symptom: HELP-TEXT-RECUT (field report 1) described `guard --help` as still
speaking `shared`/`surface` and verbs' help as citing `specs/architecture/*`.
On disk at dispatch time, both were already false — `b159673` ("recut
EnforcementMode to {Note,Warn,Block}") had already rewritten every help/about
string touching `src/main.rs`, and no `specs/architecture` cite remained
anywhere in it. `b159673` is an ancestor of the tick's HEAD.

Cost: a full manual re-derivation (grep every "shared"/"surface"/"specs/"
hit, build the binary, run `--help` on every subcommand, diff-mine
`src/main.rs`'s git log to find where the fix actually landed) before
concluding the entry's premise no longer held — because field reports are
authored against a commit that may already be behind HEAD by the time build
picks them up, and nothing marks that gap.

Suggested fix: when plan routes an inbox/field-report entry, stamp the base
commit it was authored against in the entry (e.g. `observedAt`), so build can
diff base..HEAD on the cited file first and immediately see whether the
premise still holds, instead of re-deriving it from scratch.
