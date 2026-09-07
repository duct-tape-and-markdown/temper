<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->



## CHECK-UNDECLARED-MEMBER-AT-A-GOVERNED-LOCUS: cascade's coarse-assertion sweep — no new trip, four narrowings for the same pass — observed at c2415400

cascade-integrations swept tests/*.rs and sdk/test/*.ts at bf2e3eba for
negative substring assertions, unfiltered finding counts, and full-output
snapshots over a fixture that writes a lock plus an undeclared document at
a governed locus (table at
`~/.cache/cascade-integrations/audits/sweep-coarse-assertions.md`). Sixteen
sites sit over that shape. Only the two the build found trip
(tests/acceptance.rs:358, :395) — already in the fence at c2415400. The
rest is an amendment to the entry, not a re-cut:

- Two pass only because today's message does not use the word — narrow
  them in the same pass or the next wording change trips them:
  tests/json_document_format.rs:238 excludes "description" over
  `.claude-plugin/plugin.json` with a plugin-manifest kind row and no member
  row; tests/toml_document.rs:183 excludes "mode" over
  `.claude/local/knob.toml`, whose fixture kind (`common::kind_facts`, no
  `commitment = "local"`) is committed and so inside the new finding's
  population. Add both to files[] as narrowings to the rule under test.
- tests/gauntlet.rs:277's full snapshot holds only if the manifest-kind
  exclusion lands with the walk (the entry's one predicate,
  `content == File && governs.is_some() && commitment != Local`, does
  this); otherwise the three settings.json members surface. State it in
  acceptance so a builder who loosens the predicate sees the snapshot as
  the tripwire, not a stale file.
- The two acceptance arms' `!ok` under `--deny-advisories` (:362-366,
  :399-402) keep passing for a new additional reason once the warn exists,
  so they stop proving the extent clause — pin the rule id there rather
  than the exit code.
- Safe, for the record: the eleven other sites exclude rule ids or clause
  labels the new message cannot contain; the session-start negatives
  (:110, :171, :232, :178) exclude NOTIFY_INSTRUCTION text, which only an
  error-severity finding emits.

## explain on a nested member address appends two "is not a well-formed leaf address" lines — observed at ae74bf49

From cascade-integrations' #54 rehearsal: `explain` on a
`<host-address>/<kind>/<key>` address renders the member correctly, then
appends two copies of "is not a well-formed leaf address" — the read verb
trying the leaf namespace after the member namespace already resolved.
Cosmetic, but a refusal text on a successful resolution is a lie in the
output. Verify at HEAD in `src/read.rs`'s resolve path (the nested match
landed at 9c5c7a71) and fold into whichever address-grammar entry owns the
reader, or file alone if neither touches that branch.

## a source build and the published binary print the same `temper --version`; the SDK shim runs the published one regardless of PATH — observed at ae74bf49

From cascade-integrations, an ask, not a defect claim against a ship:
`temper --version` prints `temper 0.0.17` for a source build at ae74bf49
and for the published platform binary alike, and `sdk/bin/temper.js`
execs the `optionalDependencies` platform package whatever PATH says. So
an adopter probing an unreleased build under pnpm scripts silently runs
the published engine — lock and projections are byte-identical, only the
gate verdict differs, and nothing in the output tells the two apart. Two
candidate shapes, either closes it: a `-dev+<sha>` suffix on a non-tag
build (a `build.rs` reading `git describe`, falling back to the crate
version when git is absent), or the shim honouring an explicit override
(an env var naming a binary) — the suffix is the one the adopter can read
back, so it is the one to prefer, with the override as the pnpm-script
half. `specs/distribution.md` owns packaging; check whether it speaks to
version identity at all before scoping, and surface a fork if it does not.
