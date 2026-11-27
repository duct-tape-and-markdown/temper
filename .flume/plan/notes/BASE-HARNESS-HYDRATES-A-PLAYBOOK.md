# BASE-HARNESS-HYDRATES-A-PLAYBOOK: example hydrates `corpus`; SDK gained `satisfies`

The example now composes one playbook, `corpus`, from
`.temper/playbooks/corpus.ts`. It is passed through
`defaultPatterns({ playbooks })` and path-triggered on `docs/**`, `src/**`
and `TODO.md`. Its facts are the old docs-discipline rule. It has two
topics: `verify-summary` (the old skill, which now links the scanner and
renderer systems by typed path mention) and `supersede-a-decision`. The
old rule and skill sources are retired and their projections reaped.
`emit --frozen` is clean and `check --deny-advisories` exits 0.

SDK addition the entry did not name: `PlaybookInit.satisfies?:
{ rule?, skill? }` (type `PlaybookSatisfies`). Without it, acceptance
cannot go green. The playbook's members could not join the requirement
graph, so `standards` and `operations` read unfilled (advisory). A flat
list, following `ReferenceInit.satisfies`, fails: `operations` declares
`kind: skill`, so a rule claiming it trips
`requirement.operations.kind`. Hence per surface. Plan or a human should
ratify this into specs/distribution.md's playbook axes, or rule against
it. The same gap stands open for `gateHooks()` (see the
BASE-HARNESS-HYDRATES-THE-STARTER note). Pinned by one SDK test.

Lost demo: the README's `dangling_import` example moved from
`skill:verify-summary`/`src/main.js` to `system:scanner`/`src/scan.js`
(real output, re-run). A playbook topic cannot declare `inputs`, so the
verify procedure no longer fingerprints its entrypoint.
